use godot::prelude::*;
use godot::classes::Sprite2D;

pub fn scale_fill(mut sprite: Gd<Sprite2D>, x: i16, y: i16) {
    let size = sprite.get_texture().unwrap().get_size();
    let x_scale = (x as f32) / size.x;
    let y_scale = (y as f32) / size.y;
    sprite.set_scale(Vector2 { x: x_scale, y: y_scale });
}

pub fn scale_fit(mut sprite: Gd<Sprite2D>, x: i16, y: i16) {
    let size = sprite.get_texture().unwrap().get_size();
    let x_scale = (x as f32) / size.x;
    let y_scale = (y as f32) / size.y;
    let min = std::cmp::min(x_scale as i32, y_scale as i32);
    sprite.set_scale(Vector2 { x: min as f32, y: min as f32 });
}
