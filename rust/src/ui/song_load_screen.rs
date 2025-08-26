use godot::classes::Texture2D;
use godot::classes::VBoxContainer;
use godot::prelude::*;
use godot::classes::Control;
use godot::classes::IControl;

use crate::loader::Loader;
use crate::loader::SongMetadata;
use crate::save::storage::Storage;
use crate::ui::song_cell::SongCell;
use crate::ui::song_display_screen::DisplayScreen;
use crate::step_converter::Song;

#[derive(GodotClass, Debug)]
#[class(base=Control)]
pub struct LoadScreen {
    #[export]
    pub insertion_container: Option<Gd<VBoxContainer>>,
    #[export]
    pub cell_scene: Option<Gd<PackedScene>>,
    #[export]
    pub song_display: Option<Gd<DisplayScreen>>,

    pub base: Base<Control>
}

#[godot_api]
impl LoadScreen {
    pub fn select(&mut self, song: Vec<Song>, metadata: Gd<SongMetadata>, audio: String) {
        let display = self.song_display.as_mut().expect("Could not lock song display");
        let meta = metadata.bind();
        let image = try_load::<Texture2D>(&meta.image_resource).expect("Improper image referenced");
        display.bind_mut().setup(meta.title.to_string(), meta.subtitle.to_string(), image, song, audio, meta.url.clone().into(), metadata.clone());
    }
}

#[godot_api]
impl IControl for LoadScreen {
    fn init(base: Base<Control>) -> Self {
        Self {
            insertion_container: None,
            cell_scene: None,
            song_display: None,
            base
        }
    }

    fn enter_tree(&mut self) {
        Storage::load();
        let res = Loader::get_res();
        let mut load_cell: Option<Gd<SongCell>> = None;
        for item in &res {
            let song = Song::from_str(item.text, item.bpm, item.metadata.clone());
            let metadata = item.metadata.clone();
            let cell = self.cell_scene.as_ref().expect("Cell scene not provided to menu");
            let mut init_cell = cell.try_instantiate_as::<SongCell>().expect("Could not cast scene to SongCell");
            let arg = self.to_gd().clone();
            init_cell.bind_mut().setup(metadata.clone(), item.audio.clone(), arg);
            init_cell.bind_mut().song = Some(song.clone());
            self.insertion_container.as_mut().expect("No insertion container mapped").add_child(&init_cell);
            if load_cell.is_none() {
                load_cell = Some(init_cell);
                self.select(song, metadata, item.audio.clone());
                load_cell.as_mut().expect("no load cell").bind_mut().toggled_on_entry = true;
            }
        }
    }
}

