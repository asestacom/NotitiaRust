#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

#[path = "../src/utils.rs"] mod utils;
#[path = "../src/imdb.rs"] mod imdb;

#[cfg(test)]
mod imdb_tests {
    use std::fs;
    use crate::imdb::{Imdb, get_basics, get_akas, get_ratings, update_imdbs_with_akas};

    #[test]
    fn search_basics_found() {
        let data = fs::read_to_string("tests/files/imdb/title.basics.reduced.es_test.tsv").expect("Something went wrong reading the file");
        let title = "Blade Runner";
        let title_optional = "not found";
        let year = "1982";
        
        let imdbs = get_basics(&data, (&title.to_string(), &title_optional.to_string(), &year.to_string()));
        
        assert_eq!(imdbs.len(), 1);
        assert_eq!("tt0083658", imdbs[0].tconst);
    }

    #[test]
    fn search_basics_optional_found() {
        let data = fs::read_to_string("tests/files/imdb/title.basics.reduced.es_test.tsv").expect("Something went wrong reading the file");
        let title = "not found";
        let title_optional = "Blade Runner";
        let year = "1982";
        
        let imdbs = get_basics(&data, (&title.to_string(), &title_optional.to_string(), &year.to_string()));
        
        assert_eq!(imdbs.len(), 1);
        assert_eq!("tt0083658", imdbs[0].tconst);
    }

    #[test]
    fn search_basics_not_found_wrong_year() {
        let data = fs::read_to_string("tests/files/imdb/title.basics.reduced.es_test.tsv").expect("Something went wrong reading the file");
        let title = "Blade Runner";
        let title_optional = "not found";
        let year = "1882";
        
        let imdbs = get_basics(&data, (&title.to_string(), &title_optional.to_string(), &year.to_string()));
        
        assert_eq!(imdbs.len(), 0);
    }

    #[test]
    fn search_basics_not_found() {
        let data = fs::read_to_string("tests/files/imdb/title.basics.reduced.es_test.tsv").expect("Something went wrong reading the file");
        let title = "not found";
        let title_optional = "not found";
        let year = "2025";
        
        let imdbs = get_basics(&data, (&title.to_string(), &title_optional.to_string(), &year.to_string()));
        
        assert_eq!(imdbs.len(), 0);
    }
    
    #[test]
    fn get_akas_found() {
        let data = fs::read_to_string("tests/files/imdb/title.akas.reduced.es_test.tsv").expect("Something went wrong reading the file");
        
        let imdbs = get_akas(&data, (&"Blade Runner".to_string(), &"not found".to_string()));
        
        assert_eq!(imdbs.len(), 1);
        assert_eq!("tt0083658", imdbs[0].tconst);
    }

    #[test]
    fn get_akas_optional_found() {
        let data = fs::read_to_string("tests/files/imdb/title.akas.reduced.es_test.tsv").expect("Something went wrong reading the file");
        
        let imdbs = get_akas(&data, (&"not found".to_string(), &"Blade Runner".to_string()));
        
        assert_eq!(imdbs.len(), 1);
        assert_eq!("tt0083658", imdbs[0].tconst);
    }

    #[test]
    fn get_akas_not_found() {
        let data = fs::read_to_string("tests/files/imdb/title.akas.reduced.es_test.tsv").expect("Something went wrong reading the file");
        
        let imdbs = get_akas(&data, (&"not found".to_string(), &"not found".to_string()));
        
        assert_eq!(imdbs.len(), 0);
    }

    #[test]
    fn get_ratings_found() {
        let data = fs::read_to_string("tests/files/imdb/title.ratings_test.tsv").expect("Something went wrong reading the file");
        
        let rating = get_ratings(&data, &"tt1856101".to_string());
        
        assert_eq!(rating.tconst, "tt1856101");
        assert_eq!(rating.averageRating, "8.0");
        assert_eq!(rating.numVotes, "578708");
    }

    #[test]
    fn get_ratings_found_similar_to_other_tconst() {
        let data = fs::read_to_string("tests/files/imdb/title.ratings_test.tsv").expect("Something went wrong reading the file");
        
        let rating = get_ratings(&data, &"tt18561016".to_string());
        
        assert_eq!(rating.tconst, "tt18561016");
        assert_eq!(rating.averageRating, "5.7");
        assert_eq!(rating.numVotes, "19");
    }

    #[test]
    fn get_ratings_not_found() {
        let data = fs::read_to_string("tests/files/imdb/title.ratings_test.tsv").expect("Something went wrong reading the file");
        
        let rating = get_ratings(&data, &"not found".to_string());
        
        assert_eq!(rating.tconst, "");
        assert_eq!(rating.averageRating, "");
        assert_eq!(rating.numVotes, "");
    }

    #[test]
    fn update_imdbs_with_akas_found_and_added() {
        let data = fs::read_to_string("tests/files/imdb/title.basics.reduced.es_test.tsv").expect("Something went wrong reading the file");
        let mut imdbs = Vec::<Imdb>::new();
        let mut imdbs_akas = Vec::<Imdb>::new();
        imdbs_akas.push(Imdb {
            tconst: "tt0083658".to_string(),
            titleType: String::new(),
            
            primaryTitle: String::new(),
            originalTitle: String::new(),
            year: String::new(),
            runtimeMinutes: String::new(),
            genres: String::new(),
            ratings: None,
        });
        let year = "1982";

        update_imdbs_with_akas(&mut imdbs, &imdbs_akas, &year, &data);

        assert_eq!(imdbs.len(), 1);
        assert_eq!("tt0083658", imdbs[0].tconst);
    }

    #[test]
    fn update_imdbs_with_akas_not_found() {
        let data = fs::read_to_string("tests/files/imdb/title.basics.reduced.es_test.tsv").expect("Something went wrong reading the file");
        let mut imdbs = Vec::<Imdb>::new();
        let mut imdbs_akas = Vec::<Imdb>::new();
        imdbs_akas.push(Imdb {
            tconst: "tt18561016".to_string(),
            titleType: String::new(),
            
            primaryTitle: String::new(),
            originalTitle: String::new(),
            year: String::new(),
            runtimeMinutes: String::new(),
            genres: String::new(),
            ratings: None,
        });
        let year = "1922";

        update_imdbs_with_akas(&mut imdbs, &imdbs_akas, &year, &data);

        assert_eq!(imdbs.len(), 0);
    }

    #[test]
    fn update_imdbs_with_akas_empty() {
        let data = fs::read_to_string("tests/files/imdb/title.basics.reduced.es_test.tsv").expect("Something went wrong reading the file");
        let mut imdbs = Vec::<Imdb>::new();
        let imdbs_akas = Vec::<Imdb>::new();
        let year = "1982";

        update_imdbs_with_akas(&mut imdbs, &imdbs_akas, &year, &data);

        assert_eq!(imdbs.len(), 0);
    }
}
