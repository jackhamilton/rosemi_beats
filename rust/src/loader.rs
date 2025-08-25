use godot::prelude::*;

pub struct Loader {}

#[derive(Debug)]
pub struct IncludedSong {
    pub text: &'static str,
    pub audio: String,
    pub bpm: f32,
    pub metadata: Gd<SongMetadata>
}


#[derive(GodotClass, Debug)]
#[class(base=Node)]
pub struct SongMetadata {
    #[var]
    pub title: GString,
    #[var]
    pub subtitle: GString,
    #[var]
    pub max_combo: i32,
    #[var]
    pub image_resource: GString,

    pub base: Base<Node>
}

#[godot_api]
impl SongMetadata {}

#[godot_api]
impl INode for SongMetadata {
    fn init(base: Base<Node>) -> Self {
        Self {
            title: "".into(),
            subtitle: "".into(),
            max_combo: 0,
            image_resource: "".into(),
            base
        }
    }
}

impl Loader {
    pub fn get_res() -> Vec<IncludedSong> {
        let theme = "res://Assets/Audio/theme.mp3";
        let files = vec![
            IncludedSong {
                text: include_str!("res/theme.sm"),
                audio: theme.to_string(),
                bpm: 120.0,
                metadata: Gd::from_init_fn(|base| {
                    SongMetadata {
                        title: "The Great Rosemi-sama Theme Song".into(),
                        subtitle: "author unsure".into(),
                        max_combo: 108,
                        image_resource: "res://Assets/Images/theme.png".into(),
                        base
                    }


                })
            }
        ];
        files
    }
}
