use freezable_trait::Freezable;
use godot::prelude::GodotClass;
use std::ops::DerefMut;
use godot::classes::file_access::ModeFlags;
use std::ops::Deref;
use godot::classes::FileAccess;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

pub static SINGLETON: LazyLock<Mutex<Storage>> = LazyLock::new(|| Mutex::new(Storage {
    items: HashMap::new()
}));

#[derive(Serialize, Deserialize)]
#[derive(GodotClass)]
#[class(base = Node, no_init)]
pub struct Storage {
    items: HashMap<String, String>
}

impl Storage {
    pub fn save_st<T: Freezable>(key: &'static str, item: &T) {
        Self::save(key.to_string(), item)
    }

    pub fn save<T: Freezable>(key: String, item: &T) {
        let string = serde_json::to_string(&item).expect("Error serializing string");
        SINGLETON.lock().expect("Could not lock storage").items.insert(key.to_string(), string);
    }

    pub fn load_st<T: Freezable + std::default::Default>(key: &'static str) -> T {
        Self::load(key.to_string())
    }

    pub fn load<T: Freezable + std::default::Default>(key: String) -> T {
        let binding = SINGLETON.lock().expect("Failed to lock singleton");
        let items = &binding.deref().items;
        let item = items.get(&key);
        match item {
            Some(value) => serde_json::from_str(value).expect("Failed to deserialize"),
            None => T::default()
        }
    }

    pub fn save_all() {
        let mut file = FileAccess::open("user://game_data.save", ModeFlags::WRITE).expect("Failed to open data file");
        let singleton = SINGLETON.lock().expect("Failed to lock singleton");
        let json = serde_json::to_string(singleton.deref()).expect("Could not convert to string");
        file.store_string(&json);
    }

    pub fn load_all() {
        if let Some(file) = FileAccess::open("user://game_data.save", ModeFlags::READ) {
            let json = file.get_as_text();
            let data: Storage = serde_json::from_str(json.to_string().as_str()).expect("Failed to read file");
            *SINGLETON.lock().expect("Failed to lock singleton").deref_mut() = data;
        }
    }
}
