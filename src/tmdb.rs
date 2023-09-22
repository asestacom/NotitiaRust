use crate::utils::{title_ready, get_line_by_index, search_indices};
use crate::imdb::{Imdb};
use std::fs;
use serde::{Deserialize, Serialize};

static DATA_TMDBS: &str = "files/tmdb/movies_metadata.csv";
static DATA_TMDBS_BIG: &str = "files/tmdb/archive.csv";

#[derive(Debug, Deserialize, Clone)]
#[derive(Serialize)]
#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub struct TitleTmdbsRecord {
    pub adult: String, // (boolean) = False
    pub belongs_to_collection: String, // (json) = "{'id': 1565, 'name': '28 Days/Weeks Later Collection', 'poster_path': '/4AeGz1Jwnq6ky2kfLT95Tm8nEiw.jpg', 'backdrop_path': '/bpn1vynevsKLwAMRikY1L6cb18p.jpg'}"
    pub budget: String, // 5000000
    pub genres: String, // (json) = "[{'id': 27, 'name': 'Horror'}, {'id': 53, 'name': 'Thriller'}, {'id': 878, 'name': 'Science Fiction'}]"
    pub homepage: String, //
    pub id: String, // 170
    pub imdb_id: String, // tt0289043
    pub original_language: String, // en
    pub original_title: String, // 28 Days Later
    pub overview: Option<String>, // "Twenty-eight days after a killer virus was accidentally unleashed from a British research facility, ..."
    pub popularity: String, // 17.656951
    pub poster_path: String, // (no parece funcionar muchos, usa belongs_to_collection) = /xaYdxi1PBEAYvqknvAmMPK5Eff3.jpg
    pub production_companies: String, // (json) = "[{'name': 'DNA Films', 'id': 284}, {'name': 'British Film Council', 'id': 10889}]"
    pub production_countries: String, // (json) = "[{'iso_3166_1': 'GB', 'name': 'United Kingdom'}]"
    pub release_date: String, // 2002-10-31
    pub revenue: String, // 82719885
    pub runtime: String, // 113.0
    pub spoken_languages: String, // (json) = "[{'iso_639_1': 'es', 'name': 'Español'}, {'iso_639_1': 'en', 'name': 'English'}]"
    pub status: String, // Released
    pub tagline: String, // His fear began when he woke up alone. His terror began when he realised he wasn't.,
    pub title: String, // 28 Days Later
    pub video: String, // False
    pub vote_average: String, // 7.1
    pub vote_count: String, // 1816
}
impl Default for TitleTmdbsRecord {
    fn default() -> TitleTmdbsRecord {
        TitleTmdbsRecord {
            adult: String::new(), // (boolean) = False
            belongs_to_collection: String::new(), // (json) = "{'id': 1565, 'name': '28 Days/Weeks Later Collection', 'poster_path': '/4AeGz1Jwnq6ky2kfLT95Tm8nEiw.jpg', 'backdrop_path': '/bpn1vynevsKLwAMRikY1L6cb18p.jpg'}"
            budget: String::new(), // 5000000
            genres: String::new(), // (json) = "[{'id': 27, 'name': 'Horror'}, {'id': 53, 'name': 'Thriller'}, {'id': 878, 'name': 'Science Fiction'}]"
            homepage: String::new(), //
            id: String::new(), // 170
            imdb_id: String::new(), // tt0289043
            original_language: String::new(), // en
            original_title: String::new(), // 28 Days Later
            overview: None, // "Twenty-eight days after a killer virus was accidentally unleashed from a British research facility, ..."
            popularity: String::new(), // 17.656951
            poster_path: String::new(), // (no parece funcionar muchos, usa belongs_to_collection) = /xaYdxi1PBEAYvqknvAmMPK5Eff3.jpg
            production_companies: String::new(), // (json) = "[{'name': 'DNA Films', 'id': 284}, {'name': 'British Film Council', 'id': 10889}]"
            production_countries: String::new(), // (json) = "[{'iso_3166_1': 'GB', 'name': 'United Kingdom'}]"
            release_date: String::new(), // 2002-10-31
            revenue: String::new(), // 82719885
            runtime: String::new(), // 113.0
            spoken_languages: String::new(), // (json) = "[{'iso_639_1': 'es', 'name': 'Español'}, {'iso_639_1': 'en', 'name': 'English'}]"
            status: String::new(), // Released
            tagline: String::new(), // His fear began when he woke up alone. His terror began when he realised he wasn't.,
            title: String::new(), // 28 Days Later
            video: String::new(), // False
            vote_average: String::new(), // 7.1
            vote_count: String::new(), // 1816
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[derive(Serialize)]
#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub struct TitleTmdbsBigRecord {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imdb_id: Option<String>,
    pub title: String,
    pub genres: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub popularity: String,
    pub production_companies: String,
    pub release_date: String,
    pub budget: String,
    pub revenue: String,
    pub runtime: String,
    pub status: String,
    pub tagline: String,
    pub vote_average: String,
    pub vote_count: String,
    pub credits: String,
    pub keywords: String,
    pub poster_path: String,
    pub backdrop_path: String,
    pub recommendations: String,
    /*
    id: 505642
    title: Black Panther: Wakanda Forever
    genres: Action-Adventure-Science Fiction
    original_language: en
    overview: Queen Ramonda Shuri M’Baku Okoye and the Dora Milaje fight to protect their nation from intervening world...
    popularity: 7658.731
    production_companies: Marvel Studios
    release_date: 2022-11-09
    budget: 250000000.0
    revenue: 835000000.0
    runtime: 162.0
    status: Released
    tagline: Forever.
    vote_average: 7.497
    vote_count: 2531.0
    credits: Letitia Wright-Lupita Nyong'o-Danai Gurira-Winston Duke-Dominique...
    keywords: loss of loved one-hero-sequel-superhero-based on comic-death of mother-mourning-...
    poster_path: /sv1xJUazXeYqALzczSZ3O6nkH75.jpg
    backdrop_path: /xDMIl84Qo5Tsu62c9DGWhmPI67A.jpg
    recommendations: 436270-829280-76600-56969-312634-1037858-238-551271-22023-736526-899112-468073-632856-1050535-1050957-112627-774752-322484-338958-882598
    */
}
impl Default for TitleTmdbsBigRecord {
    fn default() -> TitleTmdbsBigRecord {
        TitleTmdbsBigRecord {
            id: String::new(),
            imdb_id: None,
            title: String::new(),
            genres: String::new(),
            original_language: String::new(),
            overview: None,
            popularity: String::new(),
            production_companies: String::new(),
            release_date: String::new(),
            budget: String::new(),
            revenue: String::new(),
            runtime: String::new(),
            status: String::new(),
            tagline: String::new(),
            vote_average: String::new(),
            vote_count: String::new(),
            credits: String::new(),
            keywords: String::new(),
            poster_path: String::new(),
            backdrop_path: String::new(),
            recommendations: String::new(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[derive(Serialize)]
#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub struct TmdbRecord {
    pub id: String,
    pub title: String,
    pub genres: String,
    pub original_language: String,
    pub overview: Option<String>,
    pub popularity: String,
    pub production_companies: String,
    pub release_date: String,
    pub budget: String,
    pub revenue: String,
    pub runtime: String,
    pub status: String,
    pub tagline: String,
    pub vote_average: String,
    pub vote_count: String,
    pub credits: String,
    pub keywords: String,
    pub poster_path: String,
    pub backdrop_path: String,
    pub recommendations: String,
    pub imdb_id: String,
}
impl Default for TmdbRecord {
    fn default() -> TmdbRecord {
        TmdbRecord {
            id: String::new(),
            imdb_id: String::new(),
            title: String::new(),
            genres: String::new(),
            original_language: String::new(),
            overview: None,
            popularity: String::new(),
            production_companies: String::new(),
            release_date: String::new(),
            budget: String::new(),
            revenue: String::new(),
            runtime: String::new(),
            status: String::new(),
            tagline: String::new(),
            vote_average: String::new(),
            vote_count: String::new(),
            credits: String::new(),
            keywords: String::new(),
            poster_path: String::new(),
            backdrop_path: String::new(),
            recommendations: String::new(),
        }
    }
}

#[derive(PartialEq)]
#[derive(Debug, Deserialize, Clone)]
#[derive(Serialize)]
pub struct Tmdb {
    pub id: String, // "id" = 170
    pub imdb_id: String, // "imdb_id" = tt0289043
    pub overview: Option<String>, // "overview" = "..."

    pub poster_path: String, // from belongs_to_collection
    pub backdrop_path: String, // from belongs_to_collection
}
impl Default for Tmdb {
    fn default() -> Tmdb {
        Tmdb {
            id: String::new(),
            imdb_id: String::new(),
            overview: None,
            
            poster_path: String::new(),
            backdrop_path: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct BelongsToCollection {
    pub id: usize, // 1565
    pub name: String, // '28 Days/Weeks Later Collection'
    pub poster_path: String, // '/4AeGz1Jwnq6ky2kfLT95Tm8nEiw.jpg'
    pub backdrop_path: String, // '/bpn1vynevsKLwAMRikY1L6cb18p.jpg'
    pub overview: Option<String>, // 'In the year 180, the death of emperor Marcus Aurelius throws the Roman Empire into chaos...'
}

pub fn search_tmdbs(data_tmdbs: &String, tmdbs: &mut Vec::<Tmdb>, current_title: &String, current_year: &String, tab_at_the_beginning: bool) -> Result<(), Box<dyn std::error::Error>> {
    let title = title_ready(&current_title);
    let title_tabs = if tab_at_the_beginning { format!(",{},", title_ready(&current_title)) } else { format!("{},", title_ready(&current_title)) };
    for i in search_indices(&data_tmdbs, &title_tabs) {
        let line = get_line_by_index(&data_tmdbs, &i, true);
        let tmdb = get_tmdb(&line);

        let is_imdb_id = tmdb.imdb_id.to_ascii_lowercase() == title;
        if is_imdb_id || title_ready(&tmdb.title.to_ascii_lowercase()) == title {
            if is_imdb_id || tmdb.release_date.get(0..4) == Some(current_year) { // TODO: 1995-12-15 -> 1995
                if tmdbs.iter().find(|&item| item.id == tmdb.id) == None {
                    let mut new_tmdb = Tmdb {
                        id: tmdb.id,
                        imdb_id: tmdb.imdb_id,
                        overview: tmdb.overview,

                        poster_path: String::new(),
                        backdrop_path: String::new()
                    };
                    if tmdb.belongs_to_collection != "" {
                        match serde_json::from_str::<BelongsToCollection>(&tmdb.belongs_to_collection.replace("'", "\"")) {
                            Ok(belongs_to_collection_json) => {
                                new_tmdb.poster_path = belongs_to_collection_json.poster_path;
                                new_tmdb.backdrop_path = belongs_to_collection_json.backdrop_path;
                            },
                            Err(err) => {
                                println!("{:?}", err);
                            }
                        };
                    }
                    tmdbs.push(new_tmdb);
                }
            }
        }
    }

    Ok(())
}

pub fn search_tmdbs_big(data_tmdbs: &String, tmdbs: &mut Vec::<Tmdb>, current_title: &String, current_year: &String, tab_at_the_beginning: bool) -> Result<(), Box<dyn std::error::Error>> {
    let title = title_ready(&current_title);
    let title_tabs = if tab_at_the_beginning { format!(",{},", title_ready(&current_title)) } else { format!("{},", title_ready(&current_title)) };
    for i in search_indices(&data_tmdbs, &title_tabs) {
        let line = get_line_by_index(&data_tmdbs, &i, true);
        let tmdb = get_tmdb_big(&line);

        if tmdb.id.to_ascii_lowercase() == title 
            || title_ready(&tmdb.title.to_ascii_lowercase()) == title {
            if tmdb.release_date.get(0..4) == Some(current_year) { // TODO: 1995-12-15 -> 1995
                if tmdbs.iter().find(|&item| item.id == tmdb.id) == None {
                    tmdbs.push(Tmdb {
                        id: tmdb.id,
                        imdb_id: String::new(),
                        overview: tmdb.overview,
                        
                        poster_path: tmdb.poster_path,
                        backdrop_path: tmdb.backdrop_path,
                        });
                };
            }
        }
    }

    Ok(())
}

pub fn get_tmdb(line: &String) -> TitleTmdbsRecord {

    let line_and_header = "adult,belongs_to_collection,budget,genres,homepage,id,imdb_id,original_language,original_title,overview,popularity,poster_path,production_companies,production_countries,release_date,revenue,runtime,spoken_languages,status,tagline,title,video,vote_average,vote_count\n".to_string() + line;

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .comment(Some(b'#'))
        .from_reader(line_and_header.as_bytes());
    let mut record_return = TitleTmdbsRecord { ..Default::default() };
    for result in rdr.deserialize() {
        let record: TitleTmdbsRecord = match result {
            Ok(record) => record,
            Err(_err) => {
                /*match *err.kind() {
                    csv::ErrorKind::Deserialize { .. } => { println!("{:?}", err); }
                    _ => { println!("not a deserialize error"); }
                }*/
                continue;
            }
        };
        record_return = record;
    }
    record_return
}

pub fn get_tmdb_big(line: &String) -> TitleTmdbsBigRecord {
    let line_and_header = "id,title,genres,original_language,overview,popularity,production_companies,release_date,budget,revenue,runtime,status,tagline,vote_average,vote_count,credits,keywords,poster_path,backdrop_path,recommendations\n".to_string() + line;

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .comment(Some(b'#'))
        .from_reader(line_and_header.as_bytes());
    let mut record_return = TitleTmdbsBigRecord { ..Default::default() };
    for result in rdr.deserialize() {
        let record: TitleTmdbsBigRecord = match result {
            Ok(record) => record,
            Err(_err) => {
                /*match *err.kind() {
                    csv::ErrorKind::Deserialize { .. } => { println!("{:?}", err); }
                    _ => { println!("not a deserialize error"); }
                }*/
                continue;
            }
        };
        record_return = record;
    }
    record_return
}

pub fn search_tmdbs_by_imdb(data_tmdbs: &String, imdbs: &Vec<Imdb>, (title, title_optional, year): (&String, &String, &str)) -> Vec<Tmdb> {
    let mut tmdbs = Vec::<Tmdb>::new();
    if imdbs.len() == 0 {
        println!("search_tmdbs...");
        search_tmdbs(&data_tmdbs, &mut tmdbs, &title.to_string(), &year.to_string(), true).unwrap();
        if tmdbs.len() == 0 && title_optional.len() > 0 {
            println!("search_tmdbs...");
            search_tmdbs(&data_tmdbs, &mut tmdbs, &title_optional.to_string(), &year.to_string(), true).unwrap();
        }
    } else if imdbs.len() == 1 {
        for imdb in imdbs {
            println!("search_tmdbs...");
            search_tmdbs(&data_tmdbs, &mut tmdbs, &imdb.tconst.to_string(), &year.to_string(), true).unwrap();
        }
    } else {
        // more than one
    }
    tmdbs
}

pub fn get_tmdbs(data_tmdbs: &String, data_tmdbs_big: &String, imdbs: &Vec<Imdb>, (title, title_optional, year): (&String, &String, &String)) -> Vec<Tmdb> {
    let mut tmdbs = search_tmdbs_by_imdb(data_tmdbs, &imdbs, (&title, &title_optional, &year));

    println!("search_tmdbs_big...");
    let mut tmdbs_big = Vec::<Tmdb>::new();
    if tmdbs.len() > 0 {
        for tmdb in &tmdbs {
            search_tmdbs_big(&data_tmdbs_big, &mut tmdbs_big, &tmdb.id.clone().to_string(), &year, false).unwrap();
        }
        for tmdb_big in tmdbs_big.iter_mut() {
            for tmdb in tmdbs.iter_mut() {
                if tmdb.id == tmdb_big.id {
                    tmdb.poster_path = tmdb_big.poster_path.to_string();
                    tmdb.backdrop_path = tmdb_big.backdrop_path.to_string();
                    tmdb_big.id = "0".to_string();
                    break;
                }
            }
        }
    } else {
        search_tmdbs_big(&data_tmdbs_big, &mut tmdbs_big, &title.to_string(), &year.to_string(), true).unwrap();
        if title_optional.len() > 0 {
            search_tmdbs_big(&data_tmdbs_big, &mut tmdbs_big, &title_optional.to_string(), &year.to_string(), true).unwrap();
        }
    }
    while tmdbs_big.len() > 0 {
        let tmdb = tmdbs_big.pop().unwrap();
        if tmdb.id != "0" {
            tmdbs.push(tmdb);
        }
    }

    tmdbs
}

pub fn load_tmdbs() -> Result<String, Box<dyn std::error::Error>> {
    println!("Loading data for tmdb...");
    let path_tmdbs = DATA_TMDBS;
    let data_tmdbs = fs::read_to_string(path_tmdbs).expect("Something went wrong reading the file");

    Ok(data_tmdbs)
}

pub fn load_tmdbs_big() -> Result<String, Box<dyn std::error::Error>> {
    println!("Loading data for tmdb...");
    let path_tmdbs_big = DATA_TMDBS_BIG;
    let data_tmdbs_big = fs::read_to_string(path_tmdbs_big).expect("Something went wrong reading the file");

    Ok(data_tmdbs_big)
}
