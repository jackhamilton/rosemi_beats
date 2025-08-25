use godot::classes::Button;
use godot::classes::Label;
use godot::classes::Texture2D;
use godot::classes::TextureRect;
use godot::prelude::*;
use godot::classes::Control;
use godot::classes::IControl;

use crate::loader::SongMetadata;
use crate::step_converter::Song;
use crate::ui::song_load_screen::LoadScreen;

#[derive(GodotClass, Debug)]
#[class(base=Control)]
pub struct SongCell {
    #[export]
    pub image_rect: Option<Gd<TextureRect>>,
    #[export]
    pub title_label: Option<Gd<Label>>,
    #[export]
    pub subtitle_label: Option<Gd<Label>>,
    #[export]
    pub button: Option<Gd<Button>>,

    pub toggled_on_entry: bool,
    pub entered: bool,
    pub load_screen_ref: Option<Gd<LoadScreen>>,

    pub song: Option<Vec<Song>>,
    pub meta: Option<Gd<SongMetadata>>,
    pub audio: Option<String>,

    pub base: Base<Control>
}

#[godot_api]
impl SongCell {
    #[func]
    pub fn setup(&mut self, metadata: Gd<SongMetadata>, audio: String, load_screen_ref: Gd<LoadScreen>) {
        let image_rect_ref = self.image_rect.as_mut().expect("No image rect on base");
        let texture_path: String = metadata.bind().image_resource.clone().to_string();
        let texture = try_load::<Texture2D>(&texture_path).expect("Texture not found");
        image_rect_ref.set_texture(&texture);
        let title_label = self.title_label.as_mut().expect("No title label on base");
        title_label.set_text(&metadata.bind().title);
        let subtitle_label = self.subtitle_label.as_mut().expect("No subtitle label on base");
        subtitle_label.set_text(&metadata.bind().subtitle);
        self.meta = Some(metadata);
        self.audio = Some(audio);
        self.load_screen_ref = Some(load_screen_ref);
        self.button.as_ref().expect("No button attached").signals().toggled().connect_other(self, |this, toggled| this.toggled(toggled));
    }

    #[func]
    pub fn toggled(&mut self, toggled: bool) {
        if toggled && self.entered {
            let screen = self.load_screen_ref.as_mut().expect("No reference to load screenn");
            let song = self.song.as_ref().expect("No song provided");
            let metadata = self.meta.as_ref().expect("No metadata provided");
            let audio = self.audio.as_ref().expect("No audio provided");
            screen.bind_mut().select(song.to_vec(), metadata.clone(), audio.to_string());
        }
    }
}

#[godot_api]
impl IControl for SongCell {
    fn init(base: Base<Control>) -> Self {
        Self {
            image_rect: None,
            title_label: None,
            subtitle_label: None,
            button: None,
            toggled_on_entry: false,
            entered: false,
            load_screen_ref: None,
            song: None,
            meta: None,
            audio: None,
            base,
        }
    }

    fn ready(&mut self) {
        if self.toggled_on_entry {
            self.button.as_mut().expect("No button").set_pressed(true);
        }
        self.entered = true;
    }
}

