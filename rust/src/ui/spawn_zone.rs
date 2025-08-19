use crate::ui::animations::note_animation::NoteAnimation;
use crate::objects::note::Note;
use crate::{nodes::node_spawner::Spawner, nodes::scorer::Scorer, step_converter::NoteType};
use godot::{classes::{ColorRect}, global::absf, prelude::*};

#[derive(GodotClass, Debug)]
#[class(base=Node2D)]
pub struct SpawnZone {
    #[export]
    pub rect: Option<Gd<ColorRect>>,
    #[export]
    pub inverted: bool,
    #[export]
    pub spawner: Option<Gd<Spawner>>,
    #[export]
    pub scorer: Option<Gd<Scorer>>,
    #[var]
    pub line_x: f32,

    pub base: Base<Node2D>
}

#[godot_api]
impl SpawnZone {
    #[func]
    pub fn process_hit(&mut self, time: f32, max_time: f32, success_scene: Option<Gd<PackedScene>>) {
        let children = self.base().get_children();
        let mut nearest_note: Option<Gd<Note>> = None;
        for child in children.iter_shared() {
            if child.is_instance_valid() {
                if let Ok(note) = child.try_cast::<Note>() {
                    match nearest_note {
                        None => nearest_note = Some(note),
                        Some(ref prev) => {
                            if prev.bind().timestamp > note.bind().timestamp {
                                nearest_note = Some(note)
                            }
                        }
                    }
                }
            }
        }
        if let Some(mut nearest) = nearest_note {
            let note_timer = nearest.bind().timestamp;
            let diff = absf((note_timer - time).into());
            if diff < max_time as f64 {
                // Within timing window
                // Free note
                let nearest_position = nearest.bind().base().get_position();
                nearest.bind_mut().base_mut().queue_free();
                // Spawn successful scene
                let mut instance = success_scene.as_ref().expect("No success scene for notes connected to spawner").instantiate().expect("Failed to instantiate scene").try_cast::<NoteAnimation>().expect("No note animation in note animation");
                instance.bind_mut().base_mut().set_position(nearest_position);
                self.base_mut().add_child(&instance);
                // Notify scoring
                self.scorer.as_mut().expect("Error getting scorer").bind_mut().hit(diff, max_time as f64);
            }
        }
    }

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
            line_x: 0.0,
            spawner: None,
            scorer: None,
            base
        }
    }

    fn enter_tree(&mut self) {
        let rect_ref = self.rect.as_mut().expect("No reference to base rect");
        let rect_rect = rect_ref.get_rect();
        let inverted = self.inverted;
        let spawner = self.spawner.as_ref().expect("Note has no spawner reference").bind();
        let add_end_time = spawner.time_before_fail_ms as f32;
        let total_time = spawner.seconds_ahead_to_spawn as f32 + (spawner.time_before_fail_ms as f32 / 1000.0);
        let percent_advance = (add_end_time/1000.0) / total_time;
        let x = rect_rect.size.x * percent_advance;
        if inverted {
            self.line_x = rect_rect.size.x - x;
        } else {
            self.line_x = x;
        }
    }

    fn process(&mut self, _delta: f64) {
        let rect_ref = self.rect.as_mut().expect("No reference to base rect");
        // I know
        let rect_rect = rect_ref.get_rect();
        let inverted = self.inverted;
        let base_ref = self.base_mut();
        let children = base_ref.get_children();
        drop(base_ref);
        for child in children.iter_shared() {
            if let Ok(mut note) = child.try_cast::<Note>() {
                if note.is_instance_valid() {
                    let mut base_ref = self.base_mut();
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
                        let mut instantiated = destroy_scene.instantiate().expect("Error instantiating note destruction scene").try_cast::<NoteAnimation>().expect("Scene does not contain node2D");
                        instantiated.bind_mut();
                        instantiated.set_position(note_position);
                        base_ref.add_child(&instantiated);
                        drop(base_ref);
                        self.scorer.as_mut().expect("Error getting scorer").bind_mut().miss();
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
