#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

#[path = "../src/utils.rs"] mod utils;

#[cfg(test)]
mod utils_tests {
    use std::fs;
    use crate::utils::{remove_diacritics, title_ready, search_indices, get_line_by_index};

    #[test]
    fn get_line_by_index_test_beginning_of_file() {
        let data = fs::read_to_string("tests/files/imdb/title.ratings_test.tsv").expect("Something went wrong reading the file");
        let index = 0;

        let result = get_line_by_index(&data, &index, false);

        assert_eq!("const\taverageRating\tnumVotes", result);
    }

    #[test]
    fn get_line_by_index_test_middle() {
        let data = fs::read_to_string("tests/files/imdb/title.ratings_test.tsv").expect("Something went wrong reading the file");
        let index = 60;

        let result = get_line_by_index(&data, &index, true);

        assert_eq!("tt0083658\t8.1\t773387", result);
    }

    #[test]
    fn search_indices_one_found() {
        let data = fs::read_to_string("tests/files/imdb/title.ratings_test.tsv").expect("Something went wrong reading the file");
        let text_to_find = "tt0325980".to_string();

        let list_indexes = search_indices(&data, &text_to_find);
        let result = get_line_by_index(&data, &list_indexes[0], true);

        assert_eq!(1, list_indexes.len());
        assert_eq!(170, list_indexes[0]);
        assert_eq!("tt0325980\t8.1\t1130622", result);
    }

    #[test]
    fn search_indices_two_found() {
        let data = fs::read_to_string("tests/files/imdb/title.ratings_test.tsv").expect("Something went wrong reading the file");
        let text_to_find = "177".to_string();

        let list_indexes = search_indices(&data, &text_to_find);
        let result1 = get_line_by_index(&data, &list_indexes[0], true);
        let result2 = get_line_by_index(&data, &list_indexes[1], true);

        assert_eq!(2, list_indexes.len());
        assert_eq!(131, list_indexes[0]);
        assert_eq!(213, list_indexes[1]);
        assert_eq!("tt4116284\t7.3\t177", result1);
        assert_eq!("tt0038777\t6.9\t177", result2);
    }

    #[test]
    fn search_indices_not_found() {
        let data = fs::read_to_string("tests/files/imdb/title.ratings_test.tsv").expect("Something went wrong reading the file");
        let text_to_find = "not-found".to_string();

        let list_indexes = search_indices(&data, &text_to_find);
        
        assert_eq!(0, list_indexes.len());
    }
    
    #[test]
    fn title_ready_without_special_chars() {
        assert_eq!("name without special chars", title_ready(&"name without special chars".to_string()));
    }
    
    #[test]
    fn title_ready_with_special_chars() {
        assert_eq!("name with special chars", title_ready(&"'.name, with; special: chars?'".to_string()));
    }

    #[test]
    fn remove_diacritics_some_accents() {
        assert_eq!("aeiou", remove_diacritics(&"áéíóú".to_string()));
    }
}
