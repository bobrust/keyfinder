#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Key<'a> {
    pub tonic: &'a str,
    pub chords: Vec<String>,
}

impl<'a> Key<'a> {
    #[must_use]
    pub fn new(tonic: &'a str, chords: Vec<String>) -> Self {
        Self { tonic, chords }
    }
}

const MAJOR_SHARPS: [&str; 24] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B", "C", "C#", "D", "D#", "E",
    "F", "F#", "G", "G#", "A", "A#", "B",
];

const MAJOR_FLATS: [&str; 24] = [
    "C", "Db", "D", "Eb", "Fb", "F", "Gb", "G", "Ab", "A", "Bb", "Cb", "C", "Db", "D", "Eb", "Fb",
    "F", "Gb", "G", "Ab", "A", "Bb", "B",
];

const MINOR_SHARPS: [&str; 24] = [
    "Cm", "C#m", "Dm", "D#m", "Em", "Fm", "F#m", "Gm", "G#m", "Am", "A#m", "Bm", "Cm", "C#m", "Dm",
    "D#m", "Em", "Fm", "F#m", "Gm", "G#m", "Am", "A#m", "Bm",
];

const MINOR_FLATS: [&str; 24] = [
    "Cm", "Dbm", "Dm", "Ebm", "Em", "Fm", "Gbm", "Gm", "Abm", "Am", "Bbm", "Bm", "Cm", "Dbm", "Dm",
    "Ebm", "Em", "Fm", "Gbm", "Gm", "Abm", "Am", "Bbm", "Bm",
];

const MAJOR_INTERVALS: [usize; 7] = [2, 2, 1, 2, 2, 2, 1];
const MINOR_INTERVALS: [usize; 7] = [2, 1, 2, 2, 1, 2, 1];

pub fn init(keys: &mut Vec<Key>) {
    generate_major_keys(keys, true);
    generate_major_keys(keys, false);
    generate_minor_keys(keys, true);
    generate_minor_keys(keys, false);
    remove_duplicate_keys(keys);
}

fn remove_duplicate_keys(keys: &mut Vec<Key>) {
    keys.sort();
    keys.dedup_by(|a, b| a.tonic == b.tonic);
}

fn generate_major_keys(keys: &mut Vec<Key>, has_sharps: bool) {
    let mut scale = Vec::new();

    for i in 0..12 {
        let mut idx: usize = 0;

        idx += i;

        let tonic = if has_sharps {
            MAJOR_SHARPS[idx]
        } else {
            MAJOR_FLATS[idx]
        };

        if is_not_on_the_circle_of_fifths(tonic, true) {
            continue;
        }

        (0..7).for_each(|j| {
            let mut current_chord = if has_sharps {
                MAJOR_SHARPS[idx].to_string()
            } else {
                MAJOR_FLATS[idx].to_string()
            };

            // Create minor chords according to it's scale degree
            // I ii iii IV IV vi vii
            if j == 1 || j == 2 || j == 5 || j == 6 {
                current_chord.push('m');
            }

            scale.push(current_chord);

            idx += MAJOR_INTERVALS[j];
        });

        keys.push(Key::new(tonic, scale.clone()));
        scale.clear();
    }
}

fn generate_minor_keys(keys: &mut Vec<Key>, has_sharps: bool) {
    let mut scale = Vec::new();

    for i in 0..12 {
        let mut idx: usize = 0;

        idx += i;

        let tonic = if has_sharps {
            MINOR_SHARPS[idx]
        } else {
            MINOR_FLATS[idx]
        };

        if is_not_on_the_circle_of_fifths(tonic, false) {
            continue;
        }

        (0..7).for_each(|j| {
            let mut current_chord = if has_sharps {
                MINOR_SHARPS[idx].to_string()
            } else {
                MINOR_FLATS[idx].to_string()
            };

            // Create major chords according to it's scale degree
            // vi vii I ii iii IV V
            if j == 2 || j == 5 || j == 6 {
                current_chord.pop();
            }

            scale.push(current_chord);

            idx += MINOR_INTERVALS[j];
        });

        keys.push(Key::new(tonic, scale.clone()));
        scale.clear();
    }
}

fn is_not_on_the_circle_of_fifths(chord: &str, is_major: bool) -> bool {
    // Filter the scales that are not on the circle of 5ths.
    // These scales have uncommon chords with double flats and sharps.
    let accidental_major_keys = ["D#", "E#", "Fb", "G#", "A#", "B#"];
    let accidental_minor_keys = ["Dbm", "E#m", "Fbm", "Gbm", "B#m", "Cbm"];

    let mut result = false;

    if is_major {
        for not_allowed in accidental_major_keys {
            if chord == not_allowed {
                result = true;
                break;
            }
        }
    }

    if !is_major {
        for not_allowed in accidental_minor_keys {
            if chord == not_allowed {
                result = true;
                break;
            }
        }
    }

    result
}

#[cfg(test)]
mod init_tests {

    use crate::keys::{init, Key};

    #[test]
    fn major_and_minor_keys_both_have_15_keys_thus_should_result_in_30_keys() {
        let mut keys: Vec<Key> = Vec::new();

        init(&mut keys);

        assert_eq!(keys.len(), 30);
    }

    #[test]
    fn keys_should_contain_circle_of_fifths_keys() {
        let mut keys: Vec<Key> = Vec::new();

        init(&mut keys);

        assert!(keys.iter().any(|k| k.tonic == "Cb"));
        assert!(keys.iter().any(|k| k.tonic == "Gm"));
        assert!(keys.iter().any(|k| k.tonic == "Bbm"));
    }

    #[test]
    fn keys_should_not_contain_accidental_keys() {
        let mut keys: Vec<Key> = Vec::new();

        init(&mut keys);

        assert!(!keys.iter().any(|k| k.tonic == "Fb"));
        assert!(!keys.iter().any(|k| k.tonic == "G#"));
        assert!(!keys.iter().any(|k| k.tonic == "Cbm"));
        assert!(!keys.iter().any(|k| k.tonic == "E#m"));
    }

    #[test]
    fn keys_should_contain_diatonic_chords() {
        let mut keys: Vec<Key> = Vec::new();

        init(&mut keys);

        assert_eq!(keys[0].chords, ["A", "Bm", "C#m", "D", "E", "F#m", "G#m",]);
        assert_eq!(keys[6].chords, ["Bb", "Cm", "Dm", "Eb", "F", "Gm", "Am"]);
        assert_eq!(keys[9].chords, ["C", "Dm", "Em", "F", "G", "Am", "Bm",]);
        assert_eq!(keys[29].chords, ["Gm", "Am", "A#", "Cm", "Dm", "D#", "F"]);
    }
}
