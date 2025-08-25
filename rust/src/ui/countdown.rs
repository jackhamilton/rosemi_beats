use godot::classes::ILabel;
use godot::{classes::Label, prelude::*};

use crate::nodes::scene_root::SceneRoot;

#[derive(GodotClass, Debug)]
#[class(base=Label)]
pub struct Countdown {
    #[export]
    pub time: i32,
    pub elapsed: f32,
    #[export]
    pub root: Option<Gd<SceneRoot>>,

    pub base: Base<Label>
}

impl Countdown {}

#[godot_api]
impl ILabel for Countdown {
    fn init(base: Base<Label>) -> Self {
        Self {
            time: 4,
            elapsed: 0.0,
            root: None,
            base
        }
    }

    fn process(&mut self, delta: f32) {
        self.elapsed += delta;
        let remaining = (self.time as f32 - self.elapsed) as i32;
        self.base_mut().set_text(&format!("{}", remaining));
        if remaining <= 0 {
            self.root.as_mut().expect("No root attached").signals().start_game().emit();
            self.base_mut().queue_free();
        }
    }
}
