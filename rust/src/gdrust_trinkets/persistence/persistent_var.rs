use crate::gdrust_trinkets::persistence::storage_access::StorageAccessNode;
use freezable_trait::Freezable;
use godot::prelude::*;

pub struct PersistentVar<T: Freezable + Default> {
    storage_access: Gd<StorageAccessNode>,
    key: String,
    pub value: T
}

impl<T: Freezable + Default> PersistentVar<T> {
    pub fn new(mut storage_access: Gd<StorageAccessNode>, key: String) -> Self {
        let value = storage_access.bind_mut().load_value(key.clone());
        Self {
            storage_access,
            key,
            value,
        }
    }
}

impl<T: Freezable + Default> Drop for PersistentVar<T> {
    fn drop(&mut self) {
        self.storage_access.bind_mut().save_value_ref(self.key.clone(), &self.value);
    }
}
