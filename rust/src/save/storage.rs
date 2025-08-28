use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::{LazyLock, Mutex};
use serde::{Serialize, Deserialize};

use godot::classes::file_access::ModeFlags;
use godot::classes::{FileAccess};

#[derive(Debug, Serialize, Deserialize)]
pub struct Storage {
    pub scores: HashMap<String, i64>,
    pub combos: HashMap<String, i32>,
    pub volume: f32,
    pub controls_seen: bool,
}

pub static SINGLETON: LazyLock<Mutex<Storage>> = LazyLock::new(|| Mutex::new(Storage {
    scores: HashMap::new(),
    combos: HashMap::new(),
    volume: 100.0,
    controls_seen: false,
}));

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
        }
    }

    pub fn set_volume(vol: f32) {
        SINGLETON.lock().expect("Failed to lock singleton").deref_mut().volume = vol;
        Self::save();
    }

    pub fn get_volume() -> f32 {
        let singleton = SINGLETON.lock().expect("Failed to lock singleton");
        singleton.volume
    }

    pub fn get_scores() -> HashMap<String, i64> {
        let singleton = SINGLETON.lock().expect("Failed to lock singleton");
        singleton.scores.clone()
    }

    pub fn set_score(title: String, difficulty: i32, score: i64) {
        SINGLETON.lock().expect("Failed to lock singleton").deref_mut().scores.insert(format!("{}{}", title, difficulty), score);
        Self::save();
    }

    pub fn get_combos() -> HashMap<String, i32> {
        let singleton = SINGLETON.lock().expect("Failed to lock singleton");
        singleton.combos.clone()
    }

    pub fn set_combo(title: String, difficulty: i32, combo: i32) {
        SINGLETON.lock().expect("Failed to lock singleton").deref_mut().combos.insert(format!("{}{}", title, difficulty), combo);
        Self::save();
    }

    pub fn get_controls_seen() -> bool {
        let singleton = SINGLETON.lock().expect("Failed to lock singleton");
        singleton.controls_seen
    }

    pub fn set_controls_seen(controls_seen: bool) {
        SINGLETON.lock().expect("Failed to lock singleton").deref_mut().controls_seen = controls_seen;
        Self::save();
    }
}
