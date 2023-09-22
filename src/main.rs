#![allow(non_snake_case)]

extern crate base64;
extern crate serde_derive;
extern crate csv;

mod utils;
mod imdb;
mod tmdb;
mod just_watch;

use just_watch::{Edge, Justwatch, get_justwatchs, load_justwatchs};
use imdb::{Imdb, get_basics, get_akas, get_ratings, update_imdbs_with_akas, load_basics, load_akas, load_ratings};
use tmdb::{Tmdb, get_tmdbs, load_tmdbs, load_tmdbs_big};

use std::io::{prelude::*, BufReader};
use std::fs;
use serde::{Deserialize, Serialize};
use regex::Regex;
use chrono::Local;

// configuration
static WORKING_FILE: &str = "result/movies.json";
static OUTPUT_FILE: &str = "result/movies.result.json";
static RAW_LIST_MOVIE_FILE: &str = "result/movies.txt";
static UPDATING: bool = true;
static FILTER_NODE_FORMAT: &[&str] = &["movie", "tvSeries", "short"];

#[derive(Debug, Deserialize, Clone)]
#[derive(Serialize)]
struct NodeRecord {
    extra: String,
    format: String,
    full_node_name: String,
    node_name: String,
}
impl Default for NodeRecord {
    fn default() -> NodeRecord {
        NodeRecord {
            extra: String::new(),
            format: String::new(),
            node_name: String::new(),
            full_node_name: String::new(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[derive(Serialize)]
struct MovieRecord {
    title: String,
    year: String,

    imdb_confirmed: bool,
    imdb: Vec<Imdb>,
    justwatch_confirmed: bool,
    justwatch: Vec<Justwatch>,
    node: Vec<NodeRecord>,
    tmdb_confirmed: bool,
    tmdb: Vec<Tmdb>,
}
impl Default for MovieRecord {
    fn default() -> MovieRecord {
        MovieRecord {
            title: String::new(),
            year: String::new(),            
            node: Vec::<NodeRecord>::new(),
            
            imdb_confirmed: false,
            imdb: Vec::<Imdb>::new(),
            justwatch_confirmed: false,
            justwatch: Vec::<Justwatch>::new(),
            tmdb_confirmed: false,
            tmdb: Vec::<Tmdb>::new(),
        }
    }
}

fn load_movies(movies_fileName: &str) -> Vec<MovieRecord> {
    let movies_data = fs::read_to_string(movies_fileName.to_string()).expect("Something went wrong reading the file");
    match serde_json::from_str(&format!("{}", movies_data)) {
        Ok(json) => json,
        Err(_) => Vec::new(),
    }
}

fn save_movies(path: &str, movies: Vec<MovieRecord>) -> Result<(), Box<dyn std::error::Error>> {
    println!("----------------------------------------------------------------");
    println!("generating json...");
    let json = serde_json::to_string(&movies).unwrap();
    if std::path::Path::new(path).exists() {
        let date = Local::now();
        fs::rename(path, format!("{}-{}", path, date.format("%Y%m%d%H%M%S")))?;
    }
    let mut result = std::fs::File::create(path)?;
    write!(result, "{}", json)?;
    println!("json generated!");

    Ok(())
}

fn get_title(title_original: &str) -> String {
    let mut title: String = str::replace(title_original, ".", " ");
    title = (&title.trim_end_matches(" ")).to_string();
    title.trim_start_matches(" ").to_string()
}

fn parse_node_name(node_name: &String) -> (String, String, String, String, String) {
    let re = Regex::new(r"(.*)\((.*)\)(.*)?[.](.*)?").unwrap();
    // TODO: if there is ". " -> "  " and it won't find it
    match re.captures(&node_name) {
        None => {},
        Some(cap) => {
            let titles: Vec<&str> = cap.get(1).map_or("", |m| m.as_str()).split('~').collect();
            let year = cap.get(2).map_or("", |m| m.as_str()).to_string();
            let extra = cap.get(3).map_or("", |m| m.as_str()).to_string();
            let format = cap.get(4).map_or("", |m| m.as_str()).to_string();
            let title = get_title(&titles[0]);
            let title_optional = if titles.len() > 1 { get_title(&titles[1]) } else { String::new() };
            return (title, year, extra, format, title_optional)
        }
    };
    (String::new(), String::new(), String::new(), String::new(), String::new())
}

fn update_movie(
        movie: &mut MovieRecord,

        node_name: &String,
        full_node_name: &String,
        data: &String,
        data_akas: &String,
        justwatch_edges: &Vec<Edge>,
        data_tmdbs: &String, 
        data_tmdbs_big: &String,
        data_ratings: &String
    
    ) -> Option<()> {

    let (title, year, extra, format, title_optional) = parse_node_name(&node_name);

    let filter_node_format:  &[_] = FILTER_NODE_FORMAT;
    if filter_node_format.contains(&format.as_str()) {
        println!("----------------------------------------------------------------");
        println!("# Title: '{}' ~ '{}' - Year: ({}) '{}' '. {}'", &title, &title_optional, &year, &extra, &format);
        
        if !movie.imdb_confirmed && year != "" { 
            let mut imdbs = get_basics(data, (&title, &title_optional, &year));
    
            let imdbs_akas = if imdbs.len() == 0 { get_akas(data_akas, (&title, &title_optional)) } else { Vec::<Imdb>::new() };
            update_imdbs_with_akas(&mut imdbs, &imdbs_akas, &year, &data);

            for imdb in imdbs.iter_mut() {
                imdb.ratings = Some(get_ratings(&data_ratings, &imdb.tconst).averageRating);
            }

            movie.imdb = imdbs;
        };
        if !movie.justwatch_confirmed { 
            let justwatchs = get_justwatchs(justwatch_edges, &movie.imdb, &title);
            movie.justwatch = justwatchs; 
        };
        if !movie.tmdb_confirmed { 
            let tmdbs = get_tmdbs(&data_tmdbs, &data_tmdbs_big, &movie.imdb, (&title, &title_optional, &year));
            movie.tmdb = tmdbs; 
        };
    
        movie.title = String::from(title);
        movie.year = String::from(year);
        
        let mut nodeRecord : NodeRecord = NodeRecord { ..Default::default() };
        match movie.node.iter_mut().find(|node| node.node_name == String::from(node_name)) { 
            None => { 
                nodeRecord.node_name = String::from(node_name);
                nodeRecord.full_node_name = String::from(full_node_name);
                nodeRecord.extra = String::from(extra);
                nodeRecord.format = String::from(format);
                movie.node.push(nodeRecord);
            },
            Some(_file) => { 
        } };
    }

    Some(())
}

fn get_node_name(full_node_name: &String) -> String {
    let separators: &[_] = &['\\', '/'];
    let found_index_wrapped = full_node_name.rfind(separators);
    if found_index_wrapped != None {
        let found_index = found_index_wrapped.unwrap() + 1;
        return full_node_name.chars().skip(found_index).take(full_node_name.len() - found_index).collect();
    }
    full_node_name.clone()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

        let data = load_basics()?;
        let data_akas = load_akas()?;
        let justwatch_edges = load_justwatchs()?;
        let data_tmdbs = load_tmdbs()?;
        let data_tmdbs_big = load_tmdbs_big()?;
        let data_ratings = load_ratings()?;

        println!("checking files...");
        
        let updating = UPDATING;
        let mut movies: Vec<MovieRecord> = Vec::new();
        if updating {
            movies = load_movies(WORKING_FILE);
        }
        
        let file = std::fs::File::open(RAW_LIST_MOVIE_FILE)?;
        let reader = BufReader::new(file);
    
        let filter_node_format: &[_] = FILTER_NODE_FORMAT;
        for (_index, line) in reader.lines().enumerate() {            
            let full_node_name = line.unwrap();
            let node_name = get_node_name(&full_node_name);

            let (title, year, _extra, format, _title_optional) = parse_node_name(&node_name);
            // println!("# Title: '{}' ~ '{}' - Year: ({}) '{}' '. {}'", &title, &_title_optional, &year, &_extra, &format);
            
            if filter_node_format.contains(&format.as_str()) {
                if title == "" {
                    // no title, problems with the node name
                    let mut nodeRecord : NodeRecord = NodeRecord { ..Default::default() };
                    nodeRecord.node_name = String::from(node_name.clone());
                    nodeRecord.full_node_name = String::from(full_node_name);
                    match movies.iter_mut().find(|movie| match movie.node.iter().find(|node| node.node_name == node_name) { None => { false }, Some(_file) => { true } } ) {
                        None => {
                            match movies.iter_mut().find(|movie| movie.title == "" && movie.year == "") {
                                None => {
                                    // add (push)
                                    let mut movie : MovieRecord = MovieRecord { ..Default::default() };
                                    movie.node.push(nodeRecord);
                                    movies.push(movie);
                                },
                                Some(movie) => {
                                    // update
                                    movie.node.push(nodeRecord);
                                }
                            }
                        }
                        Some(_) => {}
                    };
                    continue;
                }

                match movies.iter_mut().find(|movie| movie.title == title && movie.year == year) {
                    None => {
                        // add (push)
                        let mut movie : MovieRecord = MovieRecord { ..Default::default() };
                                    
                        let mut nodeRecord : NodeRecord = NodeRecord { ..Default::default() };
                        nodeRecord.node_name = node_name.clone();
                        nodeRecord.full_node_name = full_node_name.clone();
                        movie.node.push(nodeRecord); // TODO: find first if exists

                        update_movie(
                            &mut movie,
                            &node_name,
                            &full_node_name,
                            &data,
                            &data_akas,
                            &justwatch_edges,
                            &data_tmdbs,
                            &data_tmdbs_big,
                            &data_ratings
                        );

                        movies.push(movie);
                    },
                    Some(mut movie) => {
                        // update
                        update_movie(
                            &mut movie,
                            &node_name,
                            &full_node_name,
                            &data,
                            &data_akas,
                            &justwatch_edges,
                            &data_tmdbs,
                            &data_tmdbs_big,
                            &data_ratings
                        );
                    },
                }            
            }
        }

        save_movies(OUTPUT_FILE, movies)?;

    Ok(())
}
