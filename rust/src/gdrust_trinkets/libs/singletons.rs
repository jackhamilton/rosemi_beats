use crate::gdrust_trinkets::persistence::storage_access::StorageAccessNode;
use godot::prelude::*;

pub struct TrinketSingletons {}

impl TrinketSingletons {
    pub fn get_storage() -> Gd<StorageAccessNode> {
        Self::get_singleton("Storage")
    }

    pub fn get_singleton<T: GodotClass + Inherits<Object>>(key: &'static str) -> Gd<T> {
        godot::classes::Engine::singleton()
            .get_singleton(key).unwrap_or_else(|| panic!("No singleton found. Please provide an autoload singleton named \"{key}\"."))
            .try_cast::<T>().expect("Singleton is not of provided type.")
    }
}
