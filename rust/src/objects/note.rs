use crate::{nodes::node_spawner::Spawner, step_converter::NoteType};
use godot::prelude::*;

#[derive(GodotClass, Debug)]
#[class(base=Node2D)]
pub struct Note {
    #[var]
    pub timestamp: f32,
    pub note_type: NoteType,
    pub spawner_ref: Option<Gd<Spawner>>,

    pub base: Base<Node2D>
}

impl Note {
    pub fn from_timestamp_type(timestamp: f32, note_type: NoteType, spawner_ref: Gd<Spawner>) -> Gd<Self> {
        Gd::from_init_fn(|base| {
            Self {
                timestamp,
                note_type,
                spawner_ref: Some(spawner_ref),
                base
            }
        })
    }

    pub fn get_note_speed(&self) -> u8 {
        self.spawner_ref.as_ref().expect("No spawner reference found").bind().note_speed
    }
}

#[godot_api]
impl INode2D for Note {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            timestamp: 0.0,
            note_type: NoteType::Empty,
            spawner_ref: None,
            base
        }
    }
}
