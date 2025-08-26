use godot::classes::IPanelContainer;
use godot::classes::Label;
use godot::classes::LinkButton;
use godot::classes::PanelContainer;
use godot::classes::Texture2D;
use godot::classes::TextureRect;
use godot::classes::VBoxContainer;
use godot::prelude::*;

use crate::loader::SongMetadata;
use crate::save::storage::Storage;
use crate::ui::difficulty_button::Difficulty;
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
    pub link: Option<Gd<LinkButton>>,

    #[export]
    pub best_score_label: Option<Gd<Label>>,
    #[export]
    pub image_container: Option<Gd<TextureRect>>,

    pub base: Base<PanelContainer>
}

#[godot_api]
impl DisplayScreen {
    pub fn setup(&mut self, title: String, subtitle: String, image: Gd<Texture2D>, difficulties: Vec<Song>, song_file: String, link: String, metadata: Gd<SongMetadata>) {
        self.title_label.as_mut().expect("No title label linked.").set_text(&title);
        self.subtitle_label.as_mut().expect("No subtitle label linked.").set_text(&subtitle);
        self.image_container.as_mut().expect("No image container linked.").set_texture(&image);
        let button_container_mut = self.button_container.as_mut().expect("Could not lock button container");
        for mut child in button_container_mut.get_children().iter_shared() {
            child.queue_free();
        }
        let mut difficulty_scores: Vec<(i32, i64, i32, bool)> = vec![];
        for difficulty in difficulties {
            let button = DifficultyButton::new(difficulty.difficulty, difficulty.clone(), song_file.clone(), metadata.clone());
            let score: i64 = match Storage::get_scores().get(&format!("{}{}", title, difficulty.difficulty)) {
                Some(score) => *score,
                None => 0,
            };
            let combo: i32 = match Storage::get_combos().get(&format!("{}{}", title, difficulty.difficulty)) {
                Some(combo) => *combo,
                None => 0,
            };
            difficulty_scores.push((difficulty.difficulty as i32, score, combo, combo >= difficulty.max_combo));
            button_container_mut.add_child(&button);
        }
        self.link.as_mut().expect("No link button").set_uri(&link);
        let mut best_score: String = "BEST\n".to_string();
        for score in difficulty_scores {
            let text = Difficulty::from(score.0 as u8).get_text();
            if score.3 {
                best_score.push_str(&format!("{}: {} (FULL)\n", text, score.1));
            } else {
                best_score.push_str(&format!("{}: {} ({}x)\n", text, score.1, score.2));
            }
        }
        self.best_score_label.as_mut().expect("No score label").set_text(best_score.strip_suffix("\n").expect("Error"));
    }
}

#[godot_api]
impl IPanelContainer for DisplayScreen {
    fn init(base: Base<PanelContainer>) -> Self {
        Self {
            button_container: None,
            title_label: None,
            subtitle_label: None,
            link: None,
            best_score_label: None,
            image_container: None,
            base
        }
    }
}

