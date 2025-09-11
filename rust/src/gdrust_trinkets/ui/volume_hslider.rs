use godot::classes::AudioServer;
use godot::classes::IHSlider;
use godot::classes::HSlider;
use godot::{prelude::*};

use crate::gdrust_trinkets::libs::singletons::TrinketSingletons;

#[derive(GodotClass, Debug)]
#[class(base=HSlider)]
pub struct VolumeHSlider {
    #[export]
    pub persisted: bool,
    #[export]
    pub persistence_key: GString,
    #[export]
    pub bus: GString,
    #[export]
    pub peak_volume: f32,
    pub base: Base<HSlider>
}

impl VolumeHSlider {}

#[godot_api]
impl IHSlider for VolumeHSlider {
    fn init(base: Base<HSlider>) -> Self {
        Self {
            persisted: false,
            persistence_key: "".into(),
            bus: "Master".into(),
            peak_volume: 60.0,
            base
        }
    }

    fn value_changed(&mut self, new_value: f64) {
        godot_print!("Confirming that this runs when the slider is installed. Delete once confirmed.");
        AudioServer::singleton().set_bus_volume_db(
            AudioServer::singleton().get_bus_index(&self.bus.to_string()),
            (1.0 - new_value as f32) * -self.peak_volume
        );
    }

    fn enter_tree(&mut self) {
        self.base_mut().set_max(1.0);
        if self.persisted {
            if self.persistence_key.is_empty() {
                panic!("No persistence key provided");
            }
            let storage = TrinketSingletons::get_storage();
            let volume = storage.bind().load_float(self.persistence_key.to_string());
            self.base_mut().set_value(volume.into());
        }
    }

    fn exit_tree(&mut self) {
        if self.persisted {
            let storage = TrinketSingletons::get_storage();
            let value = self.base().get_value();
            storage.bind().save_float(self.persistence_key.to_string(), value as f32);
        }
    }
}
