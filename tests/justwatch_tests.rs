#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

#[path = "../src/utils.rs"] mod utils;
#[path = "../src/imdb.rs"] mod imdb;
#[path = "../src/just_watch.rs"] mod justwatch;

#[cfg(test)]
mod justwatch_tests {
    use crate::imdb::{Imdb};
    use crate::justwatch::{load_justwatch_file, load_justwatch_files, get_justwatchs};

    #[test]
    fn get_justwatchs_title_found() {
        let justwatch_edges = load_justwatch_file("tests/files/justwatch/test.json").unwrap();
        let imdbs = Vec::<Imdb>::new();
        let title = "Nightmare Alley";
        
        let justwatchs = get_justwatchs(&justwatch_edges, &imdbs, &title.to_string());
        
        assert_eq!(justwatchs.len(), 1);
        assert_eq!("tm855362", justwatchs[0].id);
    }

    #[test]
    fn get_justwatchs_title_not_found() {
        let justwatch_edges = load_justwatch_file("tests/files/justwatch/test.json").unwrap();
        let imdbs = Vec::<Imdb>::new();
        let title = "not found";
        
        let justwatchs = get_justwatchs(&justwatch_edges, &imdbs, &title.to_string());
        
        assert_eq!(justwatchs.len(), 0);
    }

    #[test]
    fn get_justwatchs_imdb_id_found() {
        let justwatch_edges = load_justwatch_file("tests/files/justwatch/test.json").unwrap();
        let mut imdbs = Vec::<Imdb>::new();        
        imdbs.push(Imdb {
            tconst: "tt7740496".to_string(),
            titleType: String::new(),
            
            primaryTitle: String::new(),
            originalTitle: String::new(),
            year: String::new(),
            runtimeMinutes: String::new(),
            genres: String::new(),
            ratings: None,
        });
        let title = "not found";
        
        let justwatchs = get_justwatchs(&justwatch_edges, &imdbs, &title.to_string());
        
        assert_eq!(justwatchs.len(), 1);
        assert_eq!("tm855362", justwatchs[0].id);
    }

    #[test]
    fn get_justwatchs_several_imdb_ids_found() {
        let justwatch_edges = load_justwatch_file("tests/files/justwatch/test.json").unwrap();
        let mut imdbs = Vec::<Imdb>::new();        
        imdbs.push(Imdb {
            tconst: "tt1950186".to_string(),
            titleType: String::new(),
            
            primaryTitle: String::new(),
            originalTitle: String::new(),
            year: String::new(),
            runtimeMinutes: String::new(),
            genres: String::new(),
            ratings: None,
        });     
        imdbs.push(Imdb {
            tconst: "tt7740496".to_string(),
            titleType: String::new(),
            
            primaryTitle: String::new(),
            originalTitle: String::new(),
            year: String::new(),
            runtimeMinutes: String::new(),
            genres: String::new(),
            ratings: None,
        });
        let title = "not found";
        
        let justwatchs = get_justwatchs(&justwatch_edges, &imdbs, &title.to_string());
        
        assert_eq!(justwatchs.len(), 2);
        assert_eq!("tm855362", justwatchs[0].id);
        assert_eq!("tm420873", justwatchs[1].id);
    }

    #[test]
    fn get_justwatchs_imdb_id_not_found() {
        let justwatch_edges = load_justwatch_file("tests/files/justwatch/test.json").unwrap();
        let mut imdbs = Vec::<Imdb>::new();
        imdbs.push(Imdb {
            tconst: "not found".to_string(),
            titleType: String::new(),
            
            primaryTitle: String::new(),
            originalTitle: String::new(),
            year: String::new(),
            runtimeMinutes: String::new(),
            genres: String::new(),
            ratings: None,
        });
        let title = "not found";
        
        let justwatchs = get_justwatchs(&justwatch_edges, &imdbs, &title.to_string());
        
        assert_eq!(justwatchs.len(), 0);
    }

    #[test]
    fn get_justwatchs_title_found_in_1_using_more_than_one_input_file() {
        let justwatch_edges = load_justwatch_files(&["tests/files/justwatch/test.json", "tests/files/justwatch/test2.json"]).unwrap();
        let imdbs = Vec::<Imdb>::new();
        let title = "Nightmare Alley";
        
        let justwatchs = get_justwatchs(&justwatch_edges, &imdbs, &title.to_string());
        
        assert_eq!(justwatchs.len(), 1);
        assert_eq!("tm855362", justwatchs[0].id);
    }

    #[test]
    fn get_justwatchs_title_found_in_2_using_more_than_one_input_file() {
        let justwatch_edges = load_justwatch_files(&["tests/files/justwatch/test.json", "tests/files/justwatch/test2.json"]).unwrap();
        let imdbs = Vec::<Imdb>::new();
        let title = "Derry Girls";
        
        let justwatchs = get_justwatchs(&justwatch_edges, &imdbs, &title.to_string());
        
        assert_eq!(justwatchs.len(), 1);
        assert_eq!("ts77795", justwatchs[0].id);
    }
}
