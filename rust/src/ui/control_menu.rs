use gdrust_trinkets::persistence::storage::Storage;
use gdrust_trinkets::ui::remap_button::RemapButton;
use gdrust_trinkets::ui::volume_hslider::VolumeHSlider;
use godot::classes::Button;
use godot::classes::IControl;
use godot::classes::Control;
use godot::{prelude::*};

use crate::gdrust_trinkets;

#[derive(GodotClass, Debug)]
#[class(base=Control)]
pub struct ControlMenu {
    pub awaiting_action: Option<String>,
    pub remap_buttons: Vec<Gd<RemapButton>>,
    #[export]
    pub volume_slider: Option<Gd<VolumeHSlider>>,
    #[export]
    pub menu_button: Option<Gd<Button>>,

    pub base: Base<Control>
}

#[godot_api]
impl ControlMenu {
    #[signal]
    pub fn remapped();

    pub fn back_to_menu(&mut self) {
        let main_scene = try_load::<PackedScene>("res://start_menu.tscn").expect("Menu scene not found");
        let children = self.base_mut().get_tree().expect("No tree").get_root().expect("No root").get_children();
        self.base_mut().get_tree().expect("Tree not found").get_root().expect("No root").add_child(&main_scene.instantiate().expect("Failed to instantiate menu"));
        self.base_mut().get_tree().expect("Failed to get tree").set_pause(false);
        for mut child in children.iter_shared() {
            child.queue_free();
        }
    }
}

#[godot_api]
impl IControl for ControlMenu {
    fn init(base: Base<Control>) -> Self {
        Self {
            volume_slider: None,
            menu_button: None,
            awaiting_action: None,
            remap_buttons: vec![],
            base
        }
    }

    fn ready(&mut self) {
        self.menu_button.as_ref().expect("No menu button attached").signals().pressed().connect_other(self, |this| this.back_to_menu());
    }

    fn process(&mut self, _delta: f32) {
        if self.base().is_visible() {
            if !self.base().get_tree().expect("Failed to get tree").is_paused() {
                self.base_mut().get_tree().expect("Failed to get tree").set_pause(true);
            }
        } else if self.base().get_tree().expect("Failed to get tree").is_paused() {
            self.base_mut().get_tree().expect("Failed to get tree").set_pause(false);
        }
    }
}
