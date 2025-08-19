use godot::classes::ILine2D;
use godot::{classes::Line2D, prelude::*};

use crate::objects::note::Note;
use crate::ui::spawn_zone::SpawnZone;

#[derive(GodotClass, Debug)]
#[class(base=Line2D, no_init)]
pub struct HoldLine {
    pub pinned_start: Option<Gd<Note>>,
    pub pinned_end: Option<Gd<Note>>,
    pub parent_spawn_zone: Option<Gd<SpawnZone>>,
    pub base: Base<Line2D>
}

#[godot_api]
impl HoldLine {
    fn new(start: Gd<Note>, spawn_zone: Gd<SpawnZone>) -> Gd<HoldLine> {
        Gd::from_init_fn(|base| {
            HoldLine {
                pinned_start: Some(start),
                pinned_end: None,
                parent_spawn_zone: Some(spawn_zone),
                base
            }
        })
    }
}

#[godot_api]
impl ILine2D for HoldLine {
    fn process(&mut self, _delta: f32) {
        let start_position = self.pinned_start.as_ref().expect("No start found").get_global_position();
        let end_position = match &self.pinned_end {
            Some(end) => {
                end.get_global_position()
            }
            None => {
                let spawn = self.parent_spawn_zone.as_ref().expect("No spawn zone found").bind();
                // TODO: May be backwards
                if spawn.inverted {
                    spawn.get_rect().expect("Error getting spawn rect").get_global_position()
                } else {
                    let mut position = spawn.get_rect().expect("Error getting spawn rect").get_global_position();
                    position.x = spawn.get_rect().expect("Error getting spawn rect").get_size().x;
                    position
                }
            }
        };
        let mut mut_ref = self.base_mut();
        mut_ref.clear_points();
        let start_local = mut_ref.to_local(start_position);
        let end_local = mut_ref.to_local(end_position);
        mut_ref.add_point(start_local);
        mut_ref.add_point(end_local);
    }
}
