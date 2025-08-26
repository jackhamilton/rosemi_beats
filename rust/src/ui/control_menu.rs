use godot::classes::AudioServer;
use godot::classes::Button;
use godot::classes::IControl;
use godot::classes::Control;
use godot::classes::InputEvent;
use godot::classes::InputMap;
use godot::classes::Slider;
use godot::{prelude::*};

use crate::save::storage::Storage;
use crate::ui::remap_button::RemapButton;

#[derive(GodotClass, Debug)]
#[class(base=Control)]
pub struct ControlMenu {
    pub awaiting_action: Option<String>,
    pub remap_buttons: Vec<Gd<RemapButton>>,
    #[export]
    pub volume_slider: Option<Gd<Slider>>,
    #[export]
    pub menu_button: Option<Gd<Button>>,

    pub base: Base<Control>
}

#[godot_api]
impl ControlMenu {
    #[signal]
    pub fn remapped();

    pub fn await_action(&mut self, action: String) {
        self.awaiting_action = Some(action);
    }

    pub fn back_to_menu(&mut self) {
        let main_scene = try_load::<PackedScene>("res://start_menu.tscn").expect("Menu scene not found");
        let children = self.base_mut().get_tree().expect("No tree").get_root().expect("No root").get_children();
        self.base_mut().get_tree().expect("Tree not found").get_root().expect("No root").add_child(&main_scene.instantiate().expect("Failed to instantiate menu"));
        self.base_mut().get_tree().expect("Failed to get tree").set_pause(false);
        for mut child in children.iter_shared() {
            child.queue_free();
        }
    }

    pub fn change_volume(&mut self, volume: f32) {
        AudioServer::singleton().set_bus_volume_db(AudioServer::singleton().get_bus_index("Master"), ((100.0 - volume)/100.0) * -60.0);
        Storage::set_volume(volume);
    }

    pub fn remap(&mut self, action: String, input_event: Gd<InputEvent>) {
        let actions = InputMap::singleton().get_actions();
        for action in actions.iter_shared() {
            let events = InputMap::singleton().action_get_events(&action);
            for event in events.iter_shared() {
                if event.is_match(&input_event) {
                    InputMap::singleton().action_erase_events(&action);
                    break;
                }
            }
        }

        InputMap::singleton().action_erase_events(&action);
        InputMap::singleton().action_add_event(&action, &input_event);
        for mut btn in self.remap_buttons.clone() {
            btn.bind_mut().update_text();
        }
        self.signals().remapped().emit();
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
        let vol = Storage::get_volume();
        let slider = self.volume_slider.as_mut().expect("No volume slider attached");
        slider.set_value(vol as f64);
        // self.change_volume(vol);
        let slider = self.volume_slider.as_ref().expect("No volume slider attached");
        // slider.signals().value_changed().connect_other(self, |this, volume| this.change_volume(volume as f32));
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

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_pressed() {
            if let Some(action) = &self.awaiting_action {
                self.remap(action.to_string(), event.clone());
            }
            self.awaiting_action = None;
            if event.is_action("pause") {
                let visible = self.base().is_visible();
                self.base_mut().set_visible(!visible);
            }
        }
    }
}
