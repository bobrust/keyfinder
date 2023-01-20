use eframe::{egui, App};

use crate::{
    keyfinder::{self, find_keys, get_user_chords},
    keys::{init, Key},
};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(Debug)]
pub struct KeyFinderApplication<'a> {
    user_chords: String,
    key_result: String,
    #[serde(skip)]
    key_data: Vec<Key<'a>>,
}

impl<'a> Default for KeyFinderApplication<'a> {
    fn default() -> KeyFinderApplication<'a> {
        let mut kd: Vec<Key<'a>> = Vec::new();
        init(&mut kd);

        Self {
            user_chords: String::new(),
            key_result: String::new(),
            key_data: kd,
        }
    }
}

impl<'a> KeyFinderApplication<'a> {
    #[must_use]
    pub fn new(cc: &eframe::CreationContext) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Self::default()
    }
}

impl<'a> App for KeyFinderApplication<'a> {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            user_chords: user_input,
            key_result,
            key_data,
        } = self;

        let space = 10.;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(space);
                ui.heading("FIND THE KEY OF A CHORD PROGRESSION");
                ui.label("Extract The Song Key From Chord Progression.");
                ui.add_space(space);
            });
            ui.label("Paste your chord progression into the textarea and press the button \
                     to get the Key in which this song was composed.");
            ui.add_space(space);
            ui.add_sized([ui.available_width(), 40.0],  egui::TextEdit::multiline(user_input));
            ui.add_space(space);

            ui.vertical_centered(|ui| {
                if ui.button("Find Song Key").clicked() { 
                    key_result.clear(); 
                    let user_chords = get_user_chords(user_input);
                    let kd = key_data.clone();
                    let key_search_result = find_keys(&user_chords, kd);

                    match key_search_result {
                        Ok(_) => {
                            key_result.push_str("Your chord progression ");
                            key_result.push_str(&(*user_input).to_string());
                            key_result.push_str(" is in the key(s) of ");
                            if let Ok(vec) = key_search_result {
                                for key in vec {
                                    key_result.push_str(key.tonic);
                                    key_result.push_str(", ");
                                }
                            }
                        }
                        Err(keyfinder::KeyError::EmptyInputError) => key_result.push_str("Your input is empty..."),
                        Err(keyfinder::KeyError::NotFound) => {
                            key_result.push_str(&format!("This is embarrassing, \
                                                         we could not find a Key to this chord progression {}... \
                                                         Sorry :(", &user_input));
                        }
                    }
                }
            });

            ui.add_space(space);
            ui.separator();
            ui.add_space(space);
            ui.label(&*key_result);
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
