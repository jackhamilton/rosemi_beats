use godot::{classes::{Sprite2D}, prelude::*};

#[derive(GodotClass, Debug)]
#[class(base=Node2D)]
pub struct GameObject {
    pub sprite: Option<Gd<Sprite2D>>,

    pub base: Base<Node2D>
}

impl GameObject {
    pub fn get_rect(&self) -> Rect2 {
        let sprite = self.sprite.as_ref().expect("Could not get size: invalid sprite.");
        Rect2 {
            position: sprite.get_position(),
            size: sprite.get_rect().size * sprite.get_scale()
        }
    }
}

#[godot_api]
impl INode2D for GameObject {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            sprite: None,
            base
        }
    }

    fn enter_tree(&mut self) {
        if let Some(physics_body) = self.base().get_node_or_null("RigidBody2D") {
            let sprite = physics_body.get_node_as::<Sprite2D>("Sprite2D");
            self.sprite = Some(sprite);
        } else {
            let sprite = self.base().get_node_as::<Sprite2D>("Sprite2D");
            self.sprite = Some(sprite);
        }
    }
}
