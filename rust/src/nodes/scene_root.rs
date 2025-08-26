use crate::loader::SongMetadata;
use crate::objects::game_object::GameObject;
use crate::nodes::node_spawner::Spawner;
use crate::objects::player::Player;
use crate::save::storage::Storage;
use crate::ui::control_menu::{self, ControlMenu};
use godot::prelude::*;
use godot::classes::{CollisionShape2D, Control, Node, RigidBody2D, StaticBody2D};
use crate::step_converter::{Song, TimedNote};

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
    pub player: Option<Gd<Player>>,
    #[export]
    pub player_physics: Option<Gd<RigidBody2D>>,
    #[export]
    pub spawner: Option<Gd<Spawner>>,
    #[export]
    pub control_menu: Option<Gd<ControlMenu>>,

    pub song: Option<Song>,
    pub metadata: Option<Gd<SongMetadata>>,
    pub song_file: Option<Gd<Resource>>,

    pub base: Base<Node>
}

#[godot_api]
impl SceneRoot {
    #[signal]
    pub fn start_game();

    pub fn start_game_triggered(&mut self) {
        let song = self.song.as_ref().expect("No song");
        let song_file = self.song_file.as_ref().expect("No song file");
        let spawner = self.spawner.as_mut().expect("No spawner");
        let raster = song.clone().rasterize();
        let offset = self.metadata.as_ref().expect("No metadata").bind().offset;
        let notes = raster.iter().map(|item| TimedNote {
            timestamp: item.timestamp + offset,
            line: item.line.clone()
        }).collect();
        spawner.bind_mut().start(
            notes,
            song_file.clone(),
            song.clone().title,
            song.clone().max_combo,
            song.difficulty as i32
        );
    }

    pub fn start(&mut self, song: Song, song_file: Gd<Resource>, metadata: Gd<SongMetadata>) {
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

        if Storage::get_controls_seen() {
            self.control_menu.as_mut().expect("No control menu attached").set_visible(false);
        } else {
            Storage::set_controls_seen(true);
        }

        self.song_file = Some(song_file);
        self.song = Some(song);
        self.metadata = Some(metadata);
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
            control_menu: None,
            song: None,
            metadata: None,
            song_file: None,
            base
        }
    }

    fn enter_tree(&mut self) {
        self.signals().start_game().connect_self(|this| this.start_game_triggered());
    }
}

