use godot::classes::{IButton, InputMap};
use godot::{classes::Button, prelude::*};

use crate::ui::control_menu::{ControlMenu};

#[derive(GodotClass, Debug)]
#[class(base=Button)]
pub struct RemapButton {
    #[export]
    pub control_menu: Option<Gd<ControlMenu>>,
    #[export]
    pub action_name: GString,

    pub base: Base<Button>
}

impl RemapButton {
    pub fn update_text(&mut self) {
        let events = InputMap::singleton().action_get_events(&self.action_name.to_string());
        if let Some(first_event) = events.get(0) {
            self.base_mut().set_text(&first_event.as_text().replacen("(Physical)", "").trim_suffix(" "));
        } else {
            self.base_mut().set_text("?");
        }
    }
}

#[godot_api]
impl IButton for RemapButton {
    fn init(base: Base<Button>) -> Self {
        Self {
            control_menu: None,
            action_name: "".into(),

            base
        }
    }

    fn ready(&mut self) {
        self.update_text();
        let self_ptr = self.to_gd();
        let control = self.control_menu.as_mut().expect("No mapped control menu");
        control.bind_mut().remap_buttons.push(self_ptr);
    }

    fn pressed(&mut self) {
        self.base_mut().set_text("[set]");
        let menu = self.control_menu.as_mut().expect("could not reference control menu");
        menu.bind_mut().await_action(self.action_name.to_string());
    }
}
