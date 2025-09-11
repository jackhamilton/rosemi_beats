use godot::prelude::*;
use godot::classes::Sprite2D;

pub trait HasRect {
    fn get_rect(&self) -> Rect2;
}

pub trait Rescalable {
    fn scale_fill(&mut self, x: i16, y: i16);
    fn scale_fit(&mut self, x: i16, y: i16);
}

impl HasRect for Sprite2D {
    fn get_rect(&self) -> Rect2 {
        Rect2 {
            position: self.get_position(),
            size: self.get_rect().size * self.get_scale()
        }
    }
}

impl Rescalable for Sprite2D {
    fn scale_fill(&mut self, x: i16, y: i16) {
        let size = self.get_texture().unwrap().get_size();
        let x_scale = (x as f32) / size.x;
        let y_scale = (y as f32) / size.y;
        self.set_scale(Vector2 { x: x_scale, y: y_scale });
    }

    fn scale_fit(&mut self, x: i16, y: i16) {
        let size = self.get_texture().unwrap().get_size();
        let x_scale = (x as f32) / size.x;
        let y_scale = (y as f32) / size.y;
        let min = std::cmp::min(x_scale as i32, y_scale as i32);
        self.set_scale(Vector2 { x: min as f32, y: min as f32 });
    }
}

