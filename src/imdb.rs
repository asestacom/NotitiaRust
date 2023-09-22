use crate::utils::{title_ready, get_line_by_index, search_indices, remove_diacritics};
use std::fs;
use serde::{Deserialize, Serialize};

static LANGUAGE: &str = "ES";
static DATA: &str = "files/imdb/title.basics.reduced.es.tsv";
static DATA_AKAS: &str = "files/imdb/title.akas.reduced.es.tsv";
static DATA_RATINGS: &str = "files/imdb/title.ratings.tsv";
static TITLE_TYPES: &[&str] = &["movie", "short", "tvSeries", "tvMiniSeries", "tvMovie"];

#[derive(Debug, Deserialize, Clone)]
#[derive(Serialize)]
#[allow(non_camel_case_types)]
pub struct TitleAkasRecord {
    pub titleId: String,
    pub ordering: String,
    pub title: String,
    pub region: String,
    pub language: String,
    pub types: String,
    pub attributes: String,
    pub isOriginalTitle: String,
}

#[derive(Debug, Deserialize, Clone)]
#[derive(Serialize)]
#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub struct TitleBasicsRecord {
    pub tconst: String,
    pub titleType: String,
    pub primaryTitle: String,
    pub originalTitle: String,
    pub isAdult: String,
    pub startYear: String,
    pub endYear: String,
    pub runtimeMinutes: String,
    pub genres: String,
}
impl Default for TitleBasicsRecord {
    fn default() -> TitleBasicsRecord {
        TitleBasicsRecord {
            tconst: String::new(),
            titleType: String::new(),
            primaryTitle: String::new(),
            originalTitle: String::new(),
            isAdult: String::new(),
            startYear: String::new(),
            endYear: String::new(),
            runtimeMinutes: String::new(),
            genres: String::new(),
        }
    }
}

// title.ratings.tsv:
// tconst
// averageRating
// numVotes
#[derive(Debug, Deserialize, Clone)]
#[derive(Serialize)]
#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub struct ImdbRatings {
    pub tconst: String,
    pub averageRating: String,
    pub numVotes: String,
}
impl Default for ImdbRatings {
    fn default() -> ImdbRatings {
        ImdbRatings {
            tconst: String::new(),
            averageRating: String::new(),
            numVotes: String::new(),
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug, Deserialize, Clone)]
#[derive(Serialize)]
pub struct Imdb {
    pub tconst: String,
    pub titleType: String,

    pub primaryTitle: String,
    pub originalTitle: String,
    pub year: String,
    pub runtimeMinutes: String,
    pub genres: String,
    pub ratings: Option<String>,
}
impl Default for Imdb {
    fn default() -> Imdb {
        Imdb {
            tconst: String::new(),
            titleType: String::new(),
            
            primaryTitle: String::new(),
            originalTitle: String::new(),
            year: String::new(),
            runtimeMinutes: String::new(),
            genres: String::new(),
            ratings: None,
        }
    }
}

fn get_aka(line: &String) -> TitleAkasRecord {
    let columns_tab = if line.find('\t') != None { line.split('\t') } else { line.split(',') };
    let mut columns: Vec<&str> = columns_tab.collect();
    TitleAkasRecord {
        isOriginalTitle: columns.pop().unwrap().to_string(),
        attributes: columns.pop().unwrap().to_string(),
        types: columns.pop().unwrap().to_string(),
        language: columns.pop().unwrap().to_string(),
        region: columns.pop().unwrap().to_string(),
        title: columns.pop().unwrap().to_string(),
        ordering: columns.pop().unwrap().to_string(),
        titleId: columns.pop().unwrap().to_string(),
    }
}

fn get_basic(line: &String) -> TitleBasicsRecord {
    let columns_tab = if line.find('\t') != None { line.split('\t') } else { line.split(',') };
    let mut columns: Vec<&str> = columns_tab.collect();
    TitleBasicsRecord {
        genres: columns.pop().unwrap().to_string(),
        runtimeMinutes: columns.pop().unwrap().to_string(),
        endYear: columns.pop().unwrap().to_string(),
        startYear: columns.pop().unwrap().to_string(),
        isAdult: columns.pop().unwrap().to_string(),
        originalTitle: columns.pop().unwrap().to_string(),
        primaryTitle: columns.pop().unwrap().to_string(),
        titleType: columns.pop().unwrap().to_string(),
        tconst: columns.pop().unwrap().to_string(),
    }
}

fn get_rating(line: &String) -> ImdbRatings {
    let columns_tab = if line.find('\t') != None { line.split('\t') } else { line.split(',') };
    let mut columns: Vec<&str> = columns_tab.collect();
    ImdbRatings {
        numVotes: columns.pop().unwrap().to_string(),
        averageRating: columns.pop().unwrap().to_string(),
        tconst: columns.pop().unwrap().to_string(),
    }
}

fn search_basics(data: &String, imdbs: &mut Vec::<Imdb>, current_title: &String, current_year: &String, tab_at_the_beginning: bool) -> Result<(), Box<dyn std::error::Error>>  {
    println!("search_basics...");
    let title = title_ready(&current_title);
    let title_tabs = if tab_at_the_beginning { format!("\t{}\t", title_ready(&current_title)) } else { format!("{}\t", title_ready(&current_title)) };
    let titleTypes:  &[_] = TITLE_TYPES;
    for i in search_indices(&data, &title_tabs) {
        let line = get_line_by_index(&data, &i, true);
        let basic = get_basic(&line);

        if titleTypes.contains(&basic.titleType.as_str()) {
            if basic.tconst.to_ascii_lowercase() == title 
                || title_ready(&basic.primaryTitle.to_ascii_lowercase()) == title 
                || title_ready(&basic.originalTitle.to_ascii_lowercase()) == title {
                if &basic.startYear == current_year {
                    if imdbs.iter().find(|&item| item.tconst == basic.tconst) == None {
                        imdbs.push(Imdb {
                            tconst: basic.tconst,
                            titleType: basic.titleType,
                            
                            primaryTitle: basic.primaryTitle,
                            originalTitle: basic.originalTitle,
                            year: basic.startYear,
                            runtimeMinutes: basic.runtimeMinutes,
                            genres: basic.genres,
                            ratings: None,
                        });
                    }
                }
            }
        }
    }

    Ok(())
}

fn search_ratings(data: &String, ratings: &mut ImdbRatings, imdb_id: &String) -> Result<(), Box<dyn std::error::Error>>  {    
    println!("search_ratings...");
    let id = format!("{}\t", &imdb_id);
    for i in search_indices(&data, &id) {
        let line = get_line_by_index(&data, &i, true);
        let rating = get_rating(&line);
        ratings.tconst = rating.tconst;
        ratings.averageRating = rating.averageRating;
        ratings.numVotes = rating.numVotes;
        break;
    }

    Ok(())
}

fn search_akas(data: &String, imdbs: &mut Vec::<Imdb>, current_title: &String) -> Result<(), Box<dyn std::error::Error>> {
    println!("search_akas...");
    let title = remove_diacritics(&title_ready(&current_title));
    println!("{:?}", title);
    for i in search_indices(&data, &title) {
        let line = get_line_by_index(&data, &i, true);
        let aka = get_aka(&line);

        if title_ready(&aka.titleId.to_ascii_lowercase()) == title 
            || title_ready(&aka.title.to_ascii_lowercase()) == title {
            if aka.region == LANGUAGE {
                if imdbs.iter().find(|&item| item.tconst == aka.titleId) == None {
                    let mut imdb = Imdb { ..Default::default() };
                    imdb.tconst = aka.titleId;
                    imdb.titleType = String::new();
                    imdbs.push(imdb);
                }
            }
        }
    }
    
    Ok(())
}

pub fn load_basics() -> Result<String, Box<dyn std::error::Error>> {
    println!("Loading data for imdb basics...");
    let path = DATA;
    let data = fs::read_to_string(path).expect("Something went wrong reading the file");

    Ok(data)
}
        
pub fn load_akas() -> Result<String, Box<dyn std::error::Error>> {
    println!("Loading data for imdb akas...");
    let path = DATA_AKAS;
    let data_akas = remove_diacritics(&fs::read_to_string(path).expect("Something went wrong reading the file"));

    Ok(data_akas)    
}

pub fn load_ratings() -> Result<String, Box<dyn std::error::Error>> {
    println!("Loading data for imdb ratings...");
    let path = DATA_RATINGS;
    let data = fs::read_to_string(path).expect("Something went wrong reading the file");

    Ok(data)
}

pub fn get_basics(data: &String, (title, title_optional, year): (&String, &String, &str)) -> Vec<Imdb> {
    let mut imdbs = Vec::<Imdb>::new();
    search_basics(&data, &mut imdbs, &title.to_string(), &year.to_string(), true).unwrap();
    if imdbs.len() == 0 && title_optional.len() > 0 {
        search_basics(&data, &mut imdbs, &title_optional.to_string(), &year.to_string(), true).unwrap();
    }
    imdbs
}

pub fn get_akas(data_akas: &String, (title, title_optional): (&String, &String)) -> Vec<Imdb> {
    let mut imdbs_akas = Vec::<Imdb>::new();
    search_akas(&data_akas, &mut imdbs_akas, &title.to_string()).unwrap();
    if imdbs_akas.len() == 0 && title_optional.len() > 0 {
        search_akas(&data_akas, &mut imdbs_akas, &title_optional.to_string()).unwrap();
    }
    imdbs_akas
}

pub fn get_ratings(data: &String, imdb_id: &String) -> ImdbRatings {
    let mut ratings = ImdbRatings { ..Default::default() };
    search_ratings(&data, &mut ratings, &imdb_id.to_string()).unwrap();
    ratings
}

pub fn update_imdbs_with_akas(imdbs: &mut Vec<Imdb>, imdbs_akas: &Vec<Imdb>, year: &str, data: &String) -> () {
    if imdbs_akas.len() > 0 {
        for aka in imdbs_akas {

            let mut basic_imdbs = Vec::<Imdb>::new();
            println!("search_basics (akas)...");
            search_basics(&data, &mut basic_imdbs, &aka.tconst, &year.to_string(), false).unwrap();

            if basic_imdbs.len() > 0 {
                for basic_imdb in basic_imdbs {
                    if imdbs.iter().find(|&item| item.tconst == basic_imdb.tconst) == None {
                        imdbs.push(Imdb {
                            tconst: String::from(&basic_imdb.tconst),
                            titleType: String::from(&basic_imdb.titleType),
            
                            primaryTitle: basic_imdb.primaryTitle,
                            originalTitle: basic_imdb.originalTitle,
                            year: basic_imdb.year,
                            runtimeMinutes: basic_imdb.runtimeMinutes,
                            genres: basic_imdb.genres,
                            ratings: None,
                        });
                    }
                }
            }
        }
    }
}
