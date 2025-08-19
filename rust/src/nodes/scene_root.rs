use crate::objects::game_object::GameObject;
use crate::nodes::node_spawner::Spawner;
use godot::prelude::*;
use godot::classes::{CollisionShape2D, Control, Node, RigidBody2D, StaticBody2D};
use crate::step_converter::Song;

#[derive(GodotClass, Debug)]
#[class(base=Node)]
pub struct SceneRoot {
    #[export]
    pub game_root: Option<Gd<Node2D>>,
    #[export]
    pub ui_root: Option<Gd<Control>>,
    #[export]
    pub start_menu: Option<Gd<Control>>,
    #[export]
    pub game_ui: Option<Gd<Control>>,
    #[export]
    pub player: Option<Gd<GameObject>>,
    #[export]
    pub player_physics: Option<Gd<RigidBody2D>>,
    #[export]
    pub spawner: Option<Gd<Spawner>>,

    pub base: Base<Node>
}

#[godot_api]
impl SceneRoot {
    pub fn start(&mut self, song: Song, song_file: String) {
        let player = self.player.as_mut().expect("Player not found");
        let game_ui = self.game_ui.as_mut().expect("Game UI not found");
        // let start_menu = self.start_menu.as_mut().expect("Start menu not found");
        let game_root = self.game_root.as_mut().expect("Game root not found");
        let mut spawner = self.spawner.as_mut().expect("Spawner not found").bind_mut();
        let bounds = &mut game_root.get_node_as::<StaticBody2D>("Bounds");
        let bounds_shape = &mut bounds.get_node_as::<CollisionShape2D>("CollisionShape2D");
        // let player_sprite_height = player.bind().get_rect().size.y;
        let rect = game_ui.get_rect();
        let bottom_rect = Rect2{
            position: Vector2 {
                x: rect.center().x,
                y: rect.size.y + 3.0
            },
            size: Vector2 {
                x: rect.size.x,
                y: 5.0
            }

        };
        // bounds.set_position(bottom_rect.position);
        // let mutable_shape = &mut bounds_shape.get_shape().expect("No shape found").try_cast::<RectangleShape2D>().expect("Could not cast to rec");
        // mutable_shape.set_size(bottom_rect.size);
        // let bottom_center = Vector2 {
        //     x: bottom_rect.position.x,
        //     y: bottom_rect.position.y - (player_sprite_height / 2.0)
        // };
        spawner.set_player_base_position(player.get_position());
        spawner.left_x = Some(rect.position.x - 100.0);
        spawner.right_x = Some(rect.size.x + 100.0);
        // spawner.player_height = Some(player_sprite_height);
        // player.set_position(bottom_center);
        // start_menu.set_visible(false);
        player.set_visible(true);
        game_ui.set_visible(true);

        spawner.start(song.rasterize(), song_file);
    }
}

#[godot_api]
impl INode for SceneRoot {
    fn init(base: Base<Node>) -> Self {
        Self {
            game_root:None,
            ui_root:None,
            start_menu: None,
            game_ui: None,
            player: None,
            player_physics: None,
            spawner: None,
            base
        }
    }
}

