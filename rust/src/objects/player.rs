use godot::{classes::{AnimatedSprite2D, InputEvent}, prelude::*};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Player {
    #[export]
    pub sprite: Option<Gd<AnimatedSprite2D>>,
    pub facing: PlayerDirection,

    pub base: Base<Node2D>
}

#[derive(PartialEq)]
pub enum FiringDirection {
    Up,
    Level
}

#[derive(PartialEq)]
pub enum PlayerDirection {
    Left,
    Right
}

impl Player {
    pub fn shoot(&mut self, firing_direction: FiringDirection) {
        let sprite = self.sprite.as_mut().expect("No sprite");
        match firing_direction {
            FiringDirection::Up => {
                sprite.set_animation("left_up");
            },
            FiringDirection::Level => {
                sprite.set_animation("left");
            },
        }
        sprite.set_frame_and_progress(0, 0.0);
        sprite.play();
    }

    pub fn idle(&mut self) {
        let sprite = self.sprite.as_mut().expect("No sprite");
        sprite.set_animation("idle_2");
        sprite.play();
    }
}

#[godot_api]
impl INode2D for Player {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            sprite: None,
            facing: PlayerDirection::Left,
            base
        }
    }

    fn process(&mut self, _delta: f32) {
        if let Some(ref mut sprite) = &mut self.sprite {
            sprite.set_flip_h(self.facing == PlayerDirection::Right);
            if !sprite.is_playing() {
                self.idle();
            }
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("left") {
            self.facing = PlayerDirection::Left;
            self.shoot(FiringDirection::Level);
        } else if event.is_action_pressed("right") {
            self.facing = PlayerDirection::Right;
            self.shoot(FiringDirection::Level);
        } else if event.is_action_pressed("up_left") {
            self.facing = PlayerDirection::Left;
            self.shoot(FiringDirection::Up);
        } else if event.is_action_pressed("up_right") {
            self.facing = PlayerDirection::Right;
            self.shoot(FiringDirection::Up);
        }
    }
}
