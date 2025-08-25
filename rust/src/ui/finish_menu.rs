use godot::{classes::{Button, Control, IControl, Label}, prelude::*};

use crate::{save::storage::Storage, ui::song_load_screen::LoadScreen};

#[derive(GodotClass, Debug)]
#[class(base=Control)]
pub struct FinishMenu {
    #[export]
    pub menu_button: Option<Gd<Button>>,
    #[export]
    pub title_label: Option<Gd<Label>>,
    #[export]
    pub score_label: Option<Gd<Label>>,
    #[export]
    pub combo_label: Option<Gd<Label>>,
    pub song_title: String,
    pub song_difficulty: i32,
    pub score: i64,
    pub combo: i32,

    pub base: Base<Control>
}

impl FinishMenu {
    pub fn setup(
        &mut self,
        succeeded: bool,
        fc: bool,
        score: i64,
        combo: i32,
        song_title: String,
        song_difficulty: i32
    ) {
        let title_label = self.title_label.as_mut().expect("No title label");
        let score_label = self.score_label.as_mut().expect("No score label");
        let combo_label = self.combo_label.as_mut().expect("No combo label");
        if succeeded {
            if fc {
                title_label.set_text("FULL COMBO");
            } else {
                title_label.set_text("FINISH");
            }
        } else {
            title_label.set_text("FAILED");
        }
        score_label.set_text(&format!("{score}").to_string());
        combo_label.set_text(&format!("{combo}x").to_string());
        let menu_button = self.menu_button.as_ref().expect("No menu button");
        menu_button.signals().pressed().connect_other(self, |this| this.menu());

        self.song_title = song_title;
        self.score = score;
        self.combo = combo;
        self.song_difficulty = song_difficulty;
    }


    pub fn menu(&mut self) {
        Storage::set_score(self.song_title.clone(), self.song_difficulty, self.score);
        Storage::set_combo(self.song_title.clone(), self.song_difficulty, self.combo);
        let main_scene = try_load::<PackedScene>("res://start_menu.tscn").expect("Menu scene not found");
        let mut scene = main_scene.instantiate_as::<LoadScreen>();
        scene.set_name("StartMenu");
        self.base_mut().get_tree().expect("Tree not found").get_root().expect("No root").add_child(&scene);
        self.base_mut().get_node_as::<Node>("/root/root").queue_free();
    }
}

#[godot_api]
impl IControl for FinishMenu {
    fn init(base: Base<Control>) -> Self {
        Self {
            title_label: None,
            score_label: None,
            combo_label: None,
            menu_button: None,
            song_title: "".to_string(),
            score: 0,
            combo: 0,
            song_difficulty: 0,
            base
        }
    }
}
