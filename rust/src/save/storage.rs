use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::{LazyLock, Mutex};
use godot::global::godot_print;
use serde::{Serialize, Deserialize};

use godot::classes::file_access::ModeFlags;
use godot::classes::{FileAccess};

#[derive(Debug, Serialize, Deserialize)]
pub struct Storage {
    pub scores: HashMap<String, i64>
}

pub static SINGLETON: LazyLock<Mutex<Storage>> = LazyLock::new(|| Mutex::new(Storage { scores: HashMap::new() }));

impl Storage {
    pub fn save() {
        let mut file = FileAccess::open("user://game_data.save", ModeFlags::WRITE).expect("Failed to open data file");
        let singleton = SINGLETON.lock().expect("Failed to lock singleton");
        let json = serde_json::to_string(singleton.deref()).expect("Could not convert to string");
        file.store_string(&json);
    }

    pub fn load() {
        if let Some(file) = FileAccess::open("user://game_data.save", ModeFlags::READ) {
            let json = file.get_as_text();
            let data: Storage = serde_json::from_str(json.to_string().as_str()).expect("Failed to read file");
            *SINGLETON.lock().expect("Failed to lock singleton").deref_mut() = data;
            godot_print!("Loaded score data");
        }
    }

    pub fn get_scores() -> HashMap<String, i64> {
        let singleton = SINGLETON.lock().expect("Failed to lock singleton");
        singleton.scores.clone()
    }

    pub fn set_score(title: String, score: i64) {
        SINGLETON.lock().expect("Failed to lock singleton").deref_mut().scores.insert(title, score);
        Self::save();
    }
}
