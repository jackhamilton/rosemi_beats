use std::sync::Mutex;
use std::sync::LazyLock;
use crate::gdrust_trinkets::persistence::storage::Storage;
use freezable_trait::Freezable;
use godot::prelude::*;

#[derive(GodotClass, Debug)]
#[class(base=Node)]
pub struct StorageAccessNode {
    pub base: Base<Node>
}

static STORAGE_REFS: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(0));

#[godot_api]
impl StorageAccessNode {
    #[func]
    pub fn load_string(&self, key: String) -> String {
        Storage::load(key)
    }

    #[func]
    pub fn save_string(&self, key: String, value: String) {
        Storage::save(key, &value)
    }

    pub fn save_string_ref(&self, key: String, value: &String) {
        Storage::save(key, value)
    }

    #[func]
    pub fn load_bool(&self, key: String) -> bool {
        Storage::load(key)
    }

    #[func]
    pub fn save_bool(&self, key: String, value: bool) {
        Storage::save(key, &value)
    }

    pub fn save_bool_ref(&self, key: String, value: &bool) {
        Storage::save(key, value)
    }

    #[func]
    pub fn load_int(&self, key: String) -> i32 {
        Storage::load(key)
    }

    #[func]
    pub fn save_int(&self, key: String, value: i32) {
        Storage::save(key, &value)
    }

    pub fn save_int_ref(&self, key: String, value: &i32) {
        Storage::save(key, value)
    }

    #[func]
    pub fn load_float(&self, key: String) -> f32 {
        Storage::load(key)
    }

    #[func]
    pub fn save_float(&self, key: String, value: f32) {
        Storage::save(key, &value)
    }

    pub fn save_float_ref(&self, key: String, value: &f32) {
        Storage::save(key, value)
    }

    pub fn save_value<T: Freezable>(&self, key: String, value: T) {
        Storage::save(key, &value)
    }

    pub fn save_value_ref<T: Freezable>(&self, key: String, value: &T) {
        Storage::save(key, value)
    }

    pub fn load_value<T: Freezable + Default>(&self, key: String) -> T {
        Storage::load(key)
    }
}

#[godot_api]
impl INode for StorageAccessNode {
    fn init(base: Base<Node>) -> Self {
        *STORAGE_REFS.lock().expect("Failed to lock storage") += 1;
        Self {
            base
        }
    }

    fn enter_tree(&mut self) {
        let active = *STORAGE_REFS.lock().expect("Failed to lock storage");
        if active == 1 {
            Storage::load_all();
        }
    }

    fn exit_tree(&mut self) {
        let mut active = STORAGE_REFS.lock().expect("Failed to lock storage");
        if *active == 1 {
            Storage::save_all();
        }
        *active -= 1;
    }
}
