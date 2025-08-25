use godot::classes::Theme;
use godot::prelude::*;
use godot::classes::Button;
use godot::classes::IButton;

use crate::nodes::scene_root::SceneRoot;
use crate::step_converter::Song;

#[derive(GodotClass, Debug)]
#[class(base=Button, no_init)]
pub struct DifficultyButton {
    #[var]
    pub theme: Gd<Theme>,
    #[var]
    pub difficulty_str: GString,
    pub song: Song,
    pub song_file: String,

    pub base: Base<Button>
}

#[godot_api]
impl DifficultyButton {
    pub fn new(difficulty: u8, song: Song, song_file: String) -> Gd<Self> {
        let diff = Difficulty::from(difficulty);

        let mut button = Gd::from_init_fn(|base| {
            DifficultyButton {
                theme: diff.get_button_theme(),
                difficulty_str: diff.get_text().into(),
                song,
                song_file,

                base
            }
        });
        button.set_custom_minimum_size(Vector2 { x: 80.0, y: 24.0 });
        button
    }
}

#[derive(GodotConvert)]
#[godot(via = u8)]
pub enum Difficulty {
    Easy = 0,
    Medium = 1,
    Hard = 2
}

impl Difficulty {
    pub fn get_text(&self) -> String {
        match self {
            Self::Easy => "EASY".to_string(),
            Self::Medium => "MEDIUM".to_string(),
            Self::Hard => "HARD".to_string(),
        }
    }

    fn get_button_theme(&self) -> Gd<Theme> {
        match self {
            Self::Easy => try_load::<Theme>("res://Themes/UI/easy_button.tres").expect("Could not load easy button theme"),
            Self::Medium => try_load::<Theme>("res://Themes/UI/medium_button.tres").expect("Could not load medium button theme"),
            Self::Hard => try_load::<Theme>("res://Themes/UI/hard_button.tres").expect("Could not load hard button theme"),
        }
    }

    pub fn from(num: u8) -> Self {
        match num {
            0 => Self::Easy,
            1 => Self::Medium,
            _ => Self::Hard
        }
    }
}

#[godot_api]
impl IButton for DifficultyButton {
    fn enter_tree(&mut self) {
        let difficulty_str = &self.difficulty_str.clone();
        self.base_mut().set_text(difficulty_str);
        let theme = &self.theme.clone();
        self.base_mut().set_theme(theme);
    }

    fn pressed(&mut self) {
        let main_scene = try_load::<PackedScene>("res://main.tscn").expect("Main scene not found");
        let mut main = main_scene.instantiate_as::<SceneRoot>();
        let children = self.base_mut().get_tree().expect("No tree").get_root().expect("No root").get_children();
        self.base_mut().get_tree().expect("Tree not found").get_root().expect("No root").add_child(&main);
        main.bind_mut().start(self.song.clone(), self.song_file.clone());
        for mut child in children.iter_shared() {
            child.queue_free();
        }
    }
}

