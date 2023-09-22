#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

#[path = "../src/utils.rs"] mod utils;
#[path = "../src/tmdb.rs"] mod tmdb;
#[path = "../src/imdb.rs"] mod imdb;

#[cfg(test)]
mod tmdb_tests {
    use std::fs;
    use crate::imdb::{Imdb};
    use crate::tmdb::{Tmdb, search_tmdbs, search_tmdbs_big, search_tmdbs_by_imdb, get_tmdbs};

    #[test]
    fn search_tmdbs_found() {
        let data = fs::read_to_string("tests/files/tmdb/movies_metadata_test.csv").expect("Something went wrong reading the file");
        let mut tmdbs = Vec::<Tmdb>::new();
        let title = "Father of the Bride Part II";
        let year = "1995";
        
        search_tmdbs(&data, &mut tmdbs, &title.to_string(), &year.to_string(), true).unwrap();
        
        assert_eq!(tmdbs.len(), 1);
        assert_eq!("tt0113041", tmdbs[0].imdb_id);
    }

    #[test]
    fn search_tmdbs_not_found_wrong_year() {
        let data = fs::read_to_string("tests/files/tmdb/movies_metadata_test.csv").expect("Something went wrong reading the file");
        let mut tmdbs = Vec::<Tmdb>::new();
        let title = "Father of the Bride Part II";
        let year = "1895";
        
        search_tmdbs(&data, &mut tmdbs, &title.to_string(), &year.to_string(), true).unwrap();
        
        assert_eq!(tmdbs.len(), 0);
    }

    #[test]
    fn search_tmdbs_not_found() {
        let data = fs::read_to_string("tests/files/tmdb/movies_metadata_test.csv").expect("Something went wrong reading the file");
        let mut tmdbs = Vec::<Tmdb>::new();
        let title = "not found";
        let year = "2025";
        
        search_tmdbs(&data, &mut tmdbs, &title.to_string(), &year.to_string(), true).unwrap();
        
        assert_eq!(tmdbs.len(), 0);
    }
    
    #[test]
    fn search_tmdbs_big_found() {
        let data = fs::read_to_string("tests/files/tmdb/archive_test.csv").expect("Something went wrong reading the file");
        let mut tmdbs = Vec::<Tmdb>::new();
        let title = "Black Panther: Wakanda Forever";
        let year = "2022";
        
        search_tmdbs_big(&data, &mut tmdbs, &title.to_string(), &year.to_string(), true).unwrap();
        
        assert_eq!(tmdbs.len(), 1);
        assert_eq!("505642", tmdbs[0].id);
    }
    
    #[test]
    fn search_tmdbs_big_not_found_wrong_year() {
        let data = fs::read_to_string("tests/files/tmdb/archive_test.csv").expect("Something went wrong reading the file");
        let mut tmdbs = Vec::<Tmdb>::new();
        let title = "Black Panther: Wakanda Forever";
        let year = "1022";
        
        search_tmdbs_big(&data, &mut tmdbs, &title.to_string(), &year.to_string(), true).unwrap();
        
        assert_eq!(tmdbs.len(), 0);
    }

    #[test]
    fn search_tmdbs_big_not_found() {
        let data = fs::read_to_string("tests/files/tmdb/archive_test.csv").expect("Something went wrong reading the file");
        let mut tmdbs = Vec::<Tmdb>::new();
        let title = "not found";
        let year = "2025";        
        
        search_tmdbs_big(&data, &mut tmdbs, &title.to_string(), &year.to_string(), true).unwrap();

        assert_eq!(tmdbs.len(), 0);
    }

    #[test]
    fn search_tmdbs_by_imdb_imdb_list_empty_found() {
        let data = fs::read_to_string("tests/files/tmdb/movies_metadata_test.csv").expect("Something went wrong reading the file");
        let imdbs = Vec::<Imdb>::new();
        let title = "Father of the Bride Part II".to_string();
        let title_optional = String::new();
        let year = "1995".to_string();
        
        let tmdbs = search_tmdbs_by_imdb(&data, &imdbs, (&title, &title_optional, &year));

        assert_eq!(tmdbs.len(), 1);
        assert_eq!("tt0113041", tmdbs[0].imdb_id);
    }
    
    #[test]
    fn search_tmdbs_by_imdb_imdb_list_empty_using_title_optional_found() {
        let data = fs::read_to_string("tests/files/tmdb/movies_metadata_test.csv").expect("Something went wrong reading the file");
        let imdbs = Vec::<Imdb>::new();
        let title = "no-found".to_string();
        let title_optional = "Father of the Bride Part II".to_string();
        let year = "1995".to_string();
        
        let tmdbs = search_tmdbs_by_imdb(&data, &imdbs, (&title, &title_optional, &year));

        assert_eq!(tmdbs.len(), 1);
        assert_eq!("tt0113041", tmdbs[0].imdb_id);
    }

    #[test]
    fn search_tmdbs_by_imdb_one_item_in_imdb_list_found() {
        let data = fs::read_to_string("tests/files/tmdb/movies_metadata_test.csv").expect("Something went wrong reading the file");
        let mut imdbs = Vec::<Imdb>::new();
        imdbs.push(Imdb {
            tconst: "tt0113041".to_string(),
            titleType: String::new(),
            
            primaryTitle: String::new(),
            originalTitle: String::new(),
            year: String::new(),
            runtimeMinutes: String::new(),
            genres: String::new(),
            ratings: None,
        });
        let title = String::new();
        let title_optional = String::new();
        let year = String::new();
        
        let tmdbs = search_tmdbs_by_imdb(&data, &imdbs, (&title, &title_optional, &year));
        
        assert_eq!(tmdbs.len(), 1);
        assert_eq!("tt0113041", tmdbs[0].imdb_id);
    }
    
    #[test]
    fn get_tmdbs_found() {
        let data_tmdbs = fs::read_to_string("tests/files/tmdb/movies_metadata_test.csv").expect("Something went wrong reading the file");
        let data_tmdbs_big = fs::read_to_string("tests/files/tmdb/archive_test.csv").expect("Something went wrong reading the file");
        let imdbs = Vec::<Imdb>::new();
        let title = "Father of the Bride Part II".to_string();
        let title_optional = String::new();
        let year = "1995".to_string();
        
        let tmdbs = get_tmdbs(&data_tmdbs, &data_tmdbs_big, &imdbs, (&title, &title_optional, &year));

        assert_eq!(tmdbs.len(), 1);
        assert_eq!("tt0113041", tmdbs[0].imdb_id);
    }
}
