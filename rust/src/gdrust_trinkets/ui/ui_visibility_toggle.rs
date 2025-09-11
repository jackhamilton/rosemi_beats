use godot::classes::IButton;
use godot::classes::Button;
use godot::classes::Control;
use godot::{prelude::*};

#[derive(GodotClass, Debug)]
#[class(base=Button)]
pub struct UIVisibilityToggleButton {
    #[export]
    pub toggles: Option<Gd<Control>>,

    pub base: Base<Button>
}

impl UIVisibilityToggleButton {}

#[godot_api]
impl IButton for UIVisibilityToggleButton {
    fn init(base: Base<Button>) -> Self {
        Self {
            toggles: None,
            base
        }
    }

    fn pressed(&mut self) {
        let new_state = !self.toggles.as_ref().expect("No toggle UI assigned").is_visible();
        let toggles = self.toggles.as_mut().expect("No toggle UI assigned");
        toggles.set_visible(new_state);
    }
}
