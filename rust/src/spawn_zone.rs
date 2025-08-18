use crate::{fail_note::FailNote, node_spawner::Spawner, note::Note, step_converter::NoteType};
use godot::{classes::ColorRect, prelude::*};

#[derive(GodotClass, Debug)]
#[class(base=Node2D)]
pub struct SpawnZone {
    #[export]
    pub rect: Option<Gd<ColorRect>>,
    #[export]
    pub inverted: bool,

    pub base: Base<Node2D>
}

impl SpawnZone {
    pub fn spawn_note(&mut self, note_scene: &Gd<PackedScene>, timestamp: f32, note_type: NoteType, spawner_ref: Gd<Spawner>) {
        let mut instantiated = note_scene.instantiate().expect("Error instantiating note scene").try_cast::<Note>().expect("Note scene does not contain note");
        let mut note = instantiated.bind_mut();
        note.timestamp = timestamp;
        note.note_type = note_type;
        note.spawner_ref = Some(spawner_ref);
        let rect = self.rect.as_ref().expect("Error accessing SpawnZone rect");
        let x: f32 = {
            if self.inverted {
                rect.get_rect().size.x
            } else {
                0.0
            }
        };
        let spawn = Vector2 {
            x,
            y: rect.get_rect().size.y / 2.0
        };
        note.base_mut().set_position(spawn);
        drop(note);
        self.base_mut().add_child(&instantiated);
    }
}

#[godot_api]
impl INode2D for SpawnZone {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            rect: None,
            inverted: false,
            base
        }
    }

    fn process(&mut self, _delta: f64) {
        let rect_ref = self.rect.as_mut().expect("No reference to base rect");
        // I know
        let rect_rect = rect_ref.get_rect();
        let inverted = self.inverted;
        let mut base_ref = self.base_mut();
        let children = base_ref.get_children();
        for child in children.iter_shared() {
            if let Ok(mut note) = child.try_cast::<Note>() {
                if note.is_instance_valid() {
                    let mut note_ref = note.bind_mut();
                    let spawner = note_ref.spawner_ref.as_ref().expect("Note has no spawner reference").bind();
                    let ahead = spawner.seconds_ahead_to_spawn;
                    let c_time = spawner.time;
                    let add_end_time = spawner.time_before_fail_ms as f32;
                    let note_timestamp = note_ref.timestamp;
                    let time_left = note_timestamp - c_time + add_end_time/1000.0;
                    if time_left <= 0.0 {
                        let note_base_ref = note_ref.base();
                        let note_parent = note_base_ref.get_parent();
                        let note_position = note_base_ref.get_position();
                        let destroy_scene = spawner.note_fail_scene.clone().expect("No fail scene for note");
                        drop(spawner);
                        drop(note_base_ref);
                        drop(note_ref);
                        note_parent.expect("No parent for note").remove_child(&note);
                        let mut instantiated = destroy_scene.instantiate().expect("Error instantiating note destruction scene").try_cast::<FailNote>().expect("Scene does not contain node2D");
                        instantiated.bind_mut();
                        instantiated.set_position(note_position);
                        base_ref.add_child(&instantiated);
                        continue
                    }
                    let percent_advance = time_left / std::convert::Into::<f32>::into(ahead);
                    let x_pos = rect_rect.size.x * percent_advance;
                    drop(spawner);
                    let mut note_base_ref = note_ref.base_mut();
                    let mut note_pos = note_base_ref.get_position();
                    if inverted {
                        note_pos.x = rect_rect.size.x - x_pos;
                    } else {
                        note_pos.x = x_pos;
                    }
                    note_base_ref.set_position(note_pos);
                }
            }
        }
    }
}
