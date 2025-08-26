use godot::prelude::*;

pub struct Loader {}

#[derive(Debug)]
pub struct IncludedSong {
    pub text: &'static str,
    pub audio: Gd<Resource>,
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
    pub url: GString,
    #[var]
    pub max_combo: i32,
    #[var]
    pub offset: f32,
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
            url: "".into(),
            max_combo: 0,
            offset: 0.0,
            image_resource: "".into(),
            base
        }
    }
}

impl Loader {
    pub fn get_res() -> Vec<IncludedSong> {
        let files = vec![
            IncludedSong {
                text: include_str!("res/theme.sm"),
                audio: load("res://Assets/Audio/theme.mp3"),
                bpm: 120.0,
                metadata: Gd::from_init_fn(|base| {
                    SongMetadata {
                        title: "The Great Rosemi-sama Theme Song".into(),
                        subtitle: "ikoany, Rosemi L".into(),
                        url: "https://youtu.be/mXrIBio12vI".into(),
                        max_combo: 108,
                        offset: 0.0,
                        image_resource: "res://Assets/Images/theme.jpg".into(),
                        base
                    }
                })
            },
            IncludedSong {
                text: include_str!("res/theme.sm"),
                audio: load("res://Assets/Audio/konton_boogie.mp3"),
                bpm: 190.0,
                metadata: Gd::from_init_fn(|base| {
                    SongMetadata {
                        title: "Konton Boogie".into(),
                        subtitle: "Rosemi L, jon -YAKITORY, VESEN, puy".into(),
                        url: "https://youtu.be/5sVQNEQeaKg".into(),
                        max_combo: 1000,
                        offset: 0.0,
                        image_resource: "res://Assets/Images/konton_boogie.jpg".into(),
                        base
                    }
                })
            },
            IncludedSong {
                text: include_str!("res/kyoufuu_all_back.sm"),
                audio: load("res://Assets/Audio/kyoufuu_all_back.mp3"),
                bpm: 135.0,
                metadata: Gd::from_init_fn(|base| {
                    SongMetadata {
                        title: "Kyoufuu All Back".into(),
                        subtitle: "Yukopi, Rosemi L (tensai)".into(),
                        url: "https://youtu.be/0ZAPJ7CiY18".into(),
                        max_combo: 494,
                        offset: 0.107333,
                        image_resource: "res://Assets/Images/kyoufuu_all_back.jpg".into(),
                        base
                    }
                })
            },
            IncludedSong {
                text: include_str!("res/ringo_mogire_beam.sm"),
                audio: load("res://Assets/Audio/ringo_mogire_beam_edit.mp3"),
                bpm: 180.0,
                metadata: Gd::from_init_fn(|base| {
                    SongMetadata {
                        title: "Ringo Mogire Beam".into(),
                        subtitle: "Sonny Brisko, Rosemi L, Satoji/Nyse.S.W, Yoshiken, Kairi, RUMSKII, YAMI, Kenji Otsuki & Zetsubou Shoujotachi".into(),
                        url: "https://youtu.be/Yj65Wf4n4j4".into(),
                        max_combo: 1000,
                        offset: 0.0,
                        image_resource: "res://Assets/Images/ringo_mogire_beam.jpg".into(),
                        base
                    }
                })
            },
            IncludedSong {
                text: include_str!("res/theme.sm"),
                audio: load("res://Assets/Audio/marchen_debut.mp3"),
                bpm: 172.0,
                metadata: Gd::from_init_fn(|base| {
                    SongMetadata {
                        title: "Marchen Debut".into(),
                        subtitle: "Rosemi L ft. Shu \"shamino\" Yamino, IOSYS, Milkuriem, CReiFu".into(),
                        url: "https://youtube.com/watch?v=Khol5_-LLxA".into(),
                        max_combo: 1000,
                        offset: 0.0,
                        image_resource: "res://Assets/Images/marchen_debut.jpg".into(),
                        base
                    }
                })
            },
            IncludedSong {
                text: include_str!("res/theme.sm"),
                audio: load("res://Assets/Audio/VITAMIN_SUMMER.mp3"),
                bpm: 190.0,
                metadata: Gd::from_init_fn(|base| {
                    SongMetadata {
                        title: "VITAMIN SUMMER".into(),
                        subtitle: "Rosemi L, Liella, VESEN, m3llowone".into(),
                        url: "https://youtu.be/bmGDLC4CWso".into(),
                        max_combo: 1000,
                        offset: 0.045188,
                        image_resource: "res://Assets/Images/VITAMIN_SUMMER.jpg".into(),
                        base
                    }
                })
            },
            IncludedSong {
                text: include_str!("res/theme.sm"),
                audio: load("res://Assets/Audio/ending.mp3"),
                bpm: 140.0,
                metadata: Gd::from_init_fn(|base| {
                    SongMetadata {
                        title: "The Great Rosemi-sama Ending Theme".into(),
                        subtitle: "ikoany, Rosemi L, marlon, majormilk".into(),
                        url: "https://youtu.be/p0yvR2aGi1w".into(),
                        max_combo: 1000,
                        offset: 0.0,
                        image_resource: "res://Assets/Images/ending.jpg".into(),
                        base
                    }
                })
            }

        ];
        files
    }
}
