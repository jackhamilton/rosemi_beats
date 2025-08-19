use godot::classes::IPanelContainer;
use godot::classes::Label;
use godot::classes::PanelContainer;
use godot::classes::Texture2D;
use godot::classes::TextureRect;
use godot::classes::VBoxContainer;
use godot::prelude::*;

use crate::ui::difficulty_button::DifficultyButton;
use crate::step_converter::Song;

#[derive(GodotClass, Debug)]
#[class(base=PanelContainer)]
pub struct DisplayScreen {
    #[export]
    pub button_container: Option<Gd<VBoxContainer>>,
    #[export]
    pub title_label: Option<Gd<Label>>,
    #[export]
    pub subtitle_label: Option<Gd<Label>>,
    #[export]
    pub image_container: Option<Gd<TextureRect>>,

    pub base: Base<PanelContainer>
}

#[godot_api]
impl DisplayScreen {
    pub fn setup(&mut self, title: String, subtitle: String, image: Gd<Texture2D>, difficulties: Vec<Song>, song_file: String) {
        self.title_label.as_mut().expect("No title label linked.").set_text(&title);
        self.subtitle_label.as_mut().expect("No subtitle label linked.").set_text(&subtitle);
        self.image_container.as_mut().expect("No image container linked.").set_texture(&image);
        let button_container_mut = self.button_container.as_mut().expect("Could not lock button container");
        for mut child in button_container_mut.get_children().iter_shared() {
            child.queue_free();
        }
        for difficulty in difficulties {
            let button = DifficultyButton::new(difficulty.difficulty, difficulty, song_file.clone());
            button_container_mut.add_child(&button);
            godot_print!("Adding button");
        }
    }
}

#[godot_api]
impl IPanelContainer for DisplayScreen {
    fn init(base: Base<PanelContainer>) -> Self {
        Self {
            button_container: None,
            title_label: None,
            subtitle_label: None,
            image_container: None,
            base
        }
    }
}

