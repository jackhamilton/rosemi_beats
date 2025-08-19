use godot::prelude::*;

#[derive(GodotClass, Debug)]
#[class(base=Node)]
pub struct Scorer {
    #[var]
    pub score: i32,
    #[var]
    pub combo: i32,

    pub base: Base<Node>
}

#[godot_api]
impl Scorer {
    #[func]
    pub fn hit(&mut self, delta: f64, max_delta: f64) {
        let good = max_delta / 3.0;
        if delta < good {
            self.score += 300 * self.combo;
        } else {
            self.score += 100 * self.combo;
        }
        self.combo += 1;
    }

    #[func]
    pub fn miss(&mut self) {
        self.combo = 0;
    }
}

#[godot_api]
impl INode for Scorer {
    fn init(base: Base<Node>) -> Self {
        Self {
            score: 0,
            combo: 0,
            base
        }
    }
}
