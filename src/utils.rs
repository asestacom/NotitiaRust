/// https://docs.rs/diacritics
/// Used for removing diacritics from a string.
/// # Examples
/// ```
/// let string = "TÅRÖÄÆØ";
/// let new_string = diacritics::remove_diacritics(string);
/// assert_eq!("TAROAAO", new_string);
/// ```
pub fn remove_diacritics(string: &str) -> String {
    let chars = string.chars();
    chars.fold(String::new(), |acc, c| acc + &find_char_match(c))
}
fn find_char_match(c: char) -> String {
    match c {
        'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' | 'Æ' => "A".to_string(),
        'Þ' => "B".to_string(),
        'Ç' | 'Č' => "C".to_string(),
        'Ď' | 'Ð' => "D".to_string(),
        'Ě' | 'È' | 'É' | 'Ê' | 'Ë' => "E".to_string(),
        'Ƒ' => "F".to_string(),
        'Ì' | 'Í' | 'Î' | 'Ï' => "I".to_string(),
        'Ň' | 'Ñ' => "N".to_string(),
        'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' | 'Ø' => "O".to_string(),
        'Ř' => "R".to_string(),
        'ß' => "ss".to_string(),
        'Š' => "S".to_string(),
        'Ť' => "T".to_string(),
        'Ů' | 'Ù' | 'Ú' | 'Û' | 'Ü' => "U".to_string(),
        'Ý' => "Y".to_string(),
        'Ž' => "Z".to_string(),

        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'æ' => "a".to_string(),
        'þ' => "b".to_string(),
        'ç' | 'č' => "c".to_string(),
        'ď' | 'ð' => "d".to_string(),
        'ě' | 'è' | 'é' | 'ê' | 'ë' => "e".to_string(),
        'ƒ' => "f".to_string(),
        'ì' | 'í' | 'î' | 'ï' => "i".to_string(),
        'ñ' | 'ň' => "n".to_string(),
        'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' => "o".to_string(),
        'ř' => "r".to_string(),
        'š' => "s".to_string(),
        'ť' => "t".to_string(),
        'ů' | 'ù' | 'ú' | 'û' | 'ü' => "u".to_string(),
        'ý' | 'ÿ' => "y".to_string(),
        'ž' => "z".to_string(),
        _ => c.to_string(),
    }
}

pub fn title_ready(current_title: &String) -> String {
    let mut title = current_title.clone().to_ascii_lowercase();
    title.retain(|c| !r#":.,'?;'"#.contains(c));
    title
}

pub fn search_indices(data: &String, current_title: &String) -> Vec<usize> {
    let title = title_ready(&current_title);

    let special_chrs:  &[_] = &[b':', b'.', b'\'', b'?', b';']; // b',', 

    let title_a: &[u8] = title.as_bytes();
    let data_a: &[u8] = data.as_bytes();

    let mut indices: Vec<usize> = vec!();

    let begin_data = 0;
    let end_data = data_a.len();
    let begin = 0;
    let end = title_a.len();
    let mut j = begin_data;
    let mut last_eol: usize = 0;
    while j < end_data {
        let mut current_index: bool = true;
        if data_a[j] == b'\n' || data_a[j] == b'\r' {
            last_eol = j + 1;
        }
        let mut jj = j;
        let mut i = begin;
        while i < end {
            if special_chrs.contains(&data_a[jj + i]) { // ignore certain chrs
                jj = jj + 1;
            } else if data_a[jj + i] != title_a[i] // compare chrs
                && data_a[jj + i] < (255 - (b'a' - b'A')) && data_a[jj + i] + (b'a' - b'A') != title_a[i] { // lower case
                current_index = false;
                break;
            } else {
                i = i + 1;
            }
        }
        if current_index {
            indices.push(last_eol);
        }
        j = j + 1;
    }

    indices
}

pub fn get_line_by_index(data: &String, index: &usize, calculate_line_begins: bool) -> String {
    let mut index_l: usize = *index;
    if calculate_line_begins {
        index_l = data[..*index].rfind("\n").unwrap();
    }
    let index_r: Option<usize> = data[*index..].find("\n");
    let begin = index_l + 1;
    let end = if index_r != None { index_r.unwrap() + *index } else { data.len() };
    (&data[begin..end]).to_string()
}
