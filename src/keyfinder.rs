use crate::keys::Key;

#[derive(Debug)]
pub enum KeyError {
    EmptyInputError,
    NotFound,
}

pub fn find_keys<'a>(
    user_chords: &Vec<String>,
    key_data: Vec<Key<'a>>,
) -> Result<Vec<Key<'a>>, KeyError> {
    let mut keys_result: Vec<Key> = Vec::new();

    if user_chords.is_empty() {
        return Err(KeyError::EmptyInputError);
    }

    for current_key in key_data {
        let is_in_key = is_chord_in_key(user_chords, &current_key.chords);
        if is_in_key {
            keys_result.push(current_key);
        }
    }

    if keys_result.is_empty() {
        return Err(KeyError::NotFound);
    }

    Ok(keys_result)
}

#[cfg(test)]
mod find_keys_tests {
    use crate::keyfinder::find_keys;
    use crate::keys::Key;

    #[test]
    fn empty_user_input_should_return_error() {
        let user_chords = Vec::<String>::new();
        let key_data = vec![Key::new("C", vec!["C".to_string()])];

        let res = find_keys(&user_chords, key_data);

        assert!(res.is_err());
        // assert_eq!(res, Err(KeyError::EmptyInputError));
        // assert_eq!(res, Err(KeyError::EmptyInputError));
    }

    #[test]
    fn should_be_ok() {
        let user_chords = vec!["C".to_string()];
        let key_data = vec![Key::new("C", vec!["C".to_string(), "C".to_string()])];

        let res = find_keys(&user_chords, key_data);

        assert!(res.is_ok());
    }
}

fn is_chord_in_key(chords: &Vec<String>, chords_in_key: &[String]) -> bool {
    for chord in chords {
        if !chords_in_key.contains(chord) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod is_chord_in_key_tests {

    use crate::keyfinder::is_chord_in_key;

    #[test]
    fn when_all_chords_are_in_a_key_return_true() {
        let user_chords = vec!["C".to_string(), "Dm".to_string(), "F".to_string()];
        let chords_in_key = vec![
            "C".to_string(),
            "Dm".to_string(),
            "Em".to_string(),
            "F".to_string(),
            "G".to_string(),
            "Am".to_string(),
            "Bm".to_string(),
        ];

        assert!(is_chord_in_key(&user_chords, &chords_in_key));
    }

    #[test]
    fn when_all_chords_are_not_in_a_key_return_false() {
        let user_chords = vec!["C".to_string(), "Dm".to_string(), "F".to_string()];
        let chords_in_key = vec!["C".to_string(), "Fm".to_string()];

        assert!(!is_chord_in_key(&user_chords, &chords_in_key));
    }
}

/// Retreives chords from a string slice by splitting it up by whitespaces.
///
/// Chords will be stripped to their most basic chord quality (major or minor).
///
/// E.g:
/// ``user_input``: "Cmaj7 Dm F G7" returns the vec \["C", "Dm", "F", "G"]
pub fn get_user_chords(user_input: &mut str) -> Vec<String> {
    let split: Vec<String> = user_input
        .split_whitespace()
        // .map(|s| s.replace(&["7", "maj", "dim"][..], ""))
        .map(|s| s.replace("maj7", ""))
        .map(|s| s.replace('Œî', ""))
        .map(|s| s.replace('7', ""))
        .map(|s| s.replace("dom", ""))
        .map(|s| s.replace("dim", ""))
        .map(|s| s.replace('o', ""))
        .map(|s| s.replace('√∏', ""))
        .map(|s| s.replace("aug", ""))
        .map(|s| s.replace('+', ""))
        .map(|s| s.replace('6', ""))
        .map(|s| s.replace('9', ""))
        .map(|s| s.replace("13", ""))
        .map(|s| s.replace('-', "m"))
        .collect();
    split
}
