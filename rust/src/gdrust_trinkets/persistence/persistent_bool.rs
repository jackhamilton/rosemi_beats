use crate::gdrust_trinkets::libs::singletons::TrinketSingletons;
use crate::gdrust_trinkets::persistence::storage_access::StorageAccessNode;
use godot::prelude::*;

#[derive(GodotClass, Debug)]
#[class(base=Node)]
pub struct PersistentBool {
    storage_access: Option<Gd<StorageAccessNode>>,
    #[export]
    pub key: GString,
    #[var]
    pub value: bool,

    pub base: Base<Node>
}

#[godot_api]
impl INode for PersistentBool {
    fn init(base: Base<Node>) -> Self {
        Self {
            storage_access: None,
            key: "".into(),
            value: false,
            base
        }
    }

    fn enter_tree(&mut self) {
        self.storage_access = Some(TrinketSingletons::get_storage());

        let storage = self.storage_access.as_ref().expect("No storage access found").bind();
        if self.key.is_empty() {
            panic!("Cannot load value for empty key");
        }
        self.value = storage.load_bool(self.key.clone().into());
    }

    fn exit_tree(&mut self) {
        let storage = self.storage_access.as_ref().expect("No storage access found").bind();
        if self.key.is_empty() {
            panic!("Cannot load value for empty key");
        }
        storage.save_bool(self.key.clone().into(), self.value);
    }
}
