use godot::classes::{ButtonGroup, InputEvent};
use godot::classes::{IButton, InputMap};
use godot::{classes::Button, prelude::*};

#[derive(GodotClass, Debug)]
#[class(base=Button)]
pub struct RemapButton {
    #[export]
    pub action_name: GString,
    pub group: Option<Gd<ButtonGroup>>,
    pub awaiting_action: Option<String>,

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
        let button_group = self.group.as_mut().expect("No button group defined");
        for btn in button_group.get_buttons().iter_shared() {
            let mut remap_btn = btn.try_cast::<RemapButton>().expect("All buttons in group should be RemapButtons");
            remap_btn.bind_mut().update_text();
        }
    }

    pub fn await_action(&mut self, action: String) {
        self.awaiting_action = Some(action);
    }
}

#[godot_api]
impl IButton for RemapButton {
    fn init(base: Base<Button>) -> Self {
        Self {
            action_name: "".into(),
            group: None,
            awaiting_action: None,

            base
        }
    }

    fn ready(&mut self) {
        self.update_text();
    }

    fn pressed(&mut self) {
        self.base_mut().set_text("[set]");
        self.await_action(self.action_name.to_string());
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_pressed() {
            if let Some(action) = &self.awaiting_action {
                self.remap(action.to_string(), event.clone());
            }
            self.awaiting_action = None;
        }
    }
}
