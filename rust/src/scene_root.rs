use godot::prelude::*;
use godot::classes::{CollisionShape2D, Control, Node, RectangleShape2D, RigidBody2D, StaticBody2D};
use crate::game_object::GameObject;
use crate::loader::Loader;
use crate::node_spawner::Spawner;
use crate::step_converter::Song;

#[derive(GodotClass, Debug)]
#[class(base=Node)]
pub struct SceneRoot {
    pub game_root: Option<Gd<Node2D>>,
    pub ui_root: Option<Gd<Control>>,
    pub start_menu: Option<Gd<Control>>,
    pub game_ui: Option<Gd<Control>>,
    pub player: Option<Gd<GameObject>>,
    pub player_physics: Option<Gd<RigidBody2D>>,
    pub spawner: Option<Gd<Spawner>>,

    pub base: Base<Node>
}

#[godot_api]
impl SceneRoot {
    #[signal]
    fn start_game();

    fn start(&mut self) {
        let player = self.player.as_mut().expect("Player not found");
        let game_ui = self.game_ui.as_mut().expect("Game UI not found");
        let start_menu = self.start_menu.as_mut().expect("Start menu not found");
        let game_root = self.game_root.as_mut().expect("Game root not found");
        let mut spawner = self.spawner.as_mut().expect("Spawner not found").bind_mut();
        let bounds = &mut game_root.get_node_as::<StaticBody2D>("Bounds");
        let bounds_shape = &mut bounds.get_node_as::<CollisionShape2D>("CollisionShape2D");
        let player_sprite_height = player.bind().get_rect().size.y;
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
        spawner.set_player_base_position(bottom_center);
        spawner.left_x = Some(rect.position.x - 100.0);
        spawner.right_x = Some(rect.size.x + 100.0);
        spawner.player_height = Some(player_sprite_height);
        player.set_position(bottom_center);
        start_menu.set_visible(false);
        player.set_visible(true);
        game_ui.set_visible(true);

        let res = Loader::get_res();
        let mut song_collection: Vec<Vec<Song>> = vec![];
        for item in res {
            let song = Song::from_str(item.text, item.bpm);
            song_collection.push(song);
        }
        spawner.start(song_collection.first().expect("Could not fetch first song").first().expect("Could not fetch first song").clone().rasterize());
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

    fn ready(&mut self) {
        let game_root = self.base().get_node_as::<Node2D>("GameRoot");
        let player = game_root.get_node_as::<GameObject>("Player");
        let player_physics = player.get_node_as::<RigidBody2D>("RigidBody2D");
        let ui_root = self.base().get_node_as::<Control>("UI");
        let start_menu = ui_root.get_node_as::<Control>("StartMenu");
        let game_ui = ui_root.get_node_as::<Control>("GameUI");
        let spawner = game_root.get_node_as::<Spawner>("Spawner");
        self.game_root = Some(game_root);
        self.player = Some(player.clone());
        self.player_physics = Some(player_physics);
        self.ui_root = Some(ui_root.clone());
        self.start_menu = Some(start_menu);
        self.game_ui = Some(game_ui);
        self.spawner = Some(spawner);

        self.signals().start_game().connect_self(Self::start);
    }
}

