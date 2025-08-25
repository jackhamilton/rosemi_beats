use godot::classes::IControl;
use godot::classes::Control;
use godot::classes::InputEvent;
use godot::classes::InputMap;
use godot::{prelude::*};

use crate::ui::remap_button::RemapButton;

#[derive(GodotClass, Debug)]
#[class(base=Control)]
pub struct ControlMenu {
    pub awaiting_action: Option<String>,
    pub remap_buttons: Vec<Gd<RemapButton>>,

    pub base: Base<Control>
}

#[godot_api]
impl ControlMenu {
    #[signal]
    pub fn remapped();

    pub fn await_action(&mut self, action: String) {
        self.awaiting_action = Some(action);
    }

    pub fn remap(&mut self, action: String, input_event: Gd<InputEvent>) {
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
            awaiting_action: None,
            remap_buttons: vec![],
            base
        }
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
                self.base_mut().set_visible(!!visible);
            }
        }
    }
}
