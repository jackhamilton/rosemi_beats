use godot::classes::Label;
use godot::classes::Texture2D;
use godot::classes::TextureRect;
use godot::prelude::*;
use godot::classes::Control;
use godot::classes::IControl;

use crate::loader::SongMetadata;
use crate::step_converter::Song;

#[derive(GodotClass, Debug)]
#[class(base=Control)]
pub struct SongCell {
    #[export]
    pub image_rect: Option<Gd<TextureRect>>,
    #[export]
    pub title_label: Option<Gd<Label>>,
    #[export]
    pub subtitle_label: Option<Gd<Label>>,

    pub song: Option<Vec<Song>>,

    pub base: Base<Control>
}

#[godot_api]
impl SongCell {
    #[func]
    pub fn setup(&mut self, metadata: Gd<SongMetadata>) {
        let image_rect_ref = self.image_rect.as_mut().expect("No image rect on base");
        let texture_path: String = metadata.bind().image_resource.clone().to_string();
        let texture = try_load::<Texture2D>(&texture_path).expect("Texture not found");
        image_rect_ref.set_texture(&texture);
        let title_label = self.title_label.as_mut().expect("No title label on base");
        title_label.set_text(&metadata.bind().title);
        let subtitle_label = self.subtitle_label.as_mut().expect("No subtitle label on base");
        subtitle_label.set_text(&metadata.bind().subtitle);
    }
}

#[godot_api]
impl IControl for SongCell {
    fn init(base: Base<Control>) -> Self {
        Self {
            image_rect: None,
            title_label: None,
            subtitle_label: None,
            song: None,
            base,
        }
    }
}

