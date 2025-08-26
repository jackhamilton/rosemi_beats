use crate::nodes::scorer::Scorer;
use crate::ui::finish_menu::FinishMenu;
use crate::ui::spawn_zone::SpawnZone;
use crate::step_converter::NoteType;
use crate::step_converter::TimedNote;
use godot::classes::AudioServer;
use godot::classes::Control;
use godot::{classes::{AudioStreamMp3, AudioStreamPlayer, InputEvent}, prelude::*};

#[derive(GodotClass, Debug)]
#[class(base=Node)]
pub struct Spawner {
    pub player_base_position: Option<Vector2>,
    pub player_height: Option<f32>,
    pub left_x: Option<f32>,
    pub right_x: Option<f32>,
    pub song_title: Option<String>,
    pub song_max_combo: Option<i32>,
    pub song_difficulty: Option<i32>,
    pub song: Option<Vec<TimedNote>>,
    #[var]
    pub time: f32,
    #[var]
    pub playing: bool,
    #[export]
    pub seconds_ahead_to_spawn: u8,
    #[export]
    //unused
    pub note_speed: u8,
    #[export]
    pub time_before_fail_ms: u32,
    #[export]
    pub spawn_zone_one: Option<Gd<SpawnZone>>,
    #[export]
    pub spawn_zone_two: Option<Gd<SpawnZone>>,
    #[export]
    pub spawn_zone_three: Option<Gd<SpawnZone>>,
    #[export]
    pub spawn_zone_four: Option<Gd<SpawnZone>>,
    #[export]
    pub note_scene: Option<Gd<PackedScene>>,
    #[export]
    pub note_fail_scene: Option<Gd<PackedScene>>,
    #[export]
    pub note_success_scene: Option<Gd<PackedScene>>,
    #[export]
    pub audio_stream: Option<Gd<AudioStreamPlayer>>,
    #[export]
    pub scorer: Option<Gd<Scorer>>,
    #[export]
    pub finish_menu: Option<Gd<FinishMenu>>,
    #[export]
    pub settings_gear: Option<Gd<Control>>,

    pub base: Base<Node>
}

#[godot_api]
impl Spawner {
    // Body
    #[func]
    pub fn set_player_base_position(&mut self, position: Vector2) {
        self.player_base_position = Some(position);
    }

    pub fn start(
        &mut self,
        song: Vec<TimedNote>,
        resource: Gd<Resource>,
        song_title: String,
        song_max_combo: i32,
        song_difficulty: i32
    ) {
        self.time = 0.0;
        self.song = Some(song);
        self.playing = true;
        self.song_title = Some(song_title);
        self.song_max_combo = Some(song_max_combo);
        self.song_difficulty = Some(song_difficulty);
        let stream = self.audio_stream.as_mut().expect("No audio stream");
        let audio_stream = resource.try_cast::<AudioStreamMp3>().expect("Not an mp3");
        stream.set_stream(&audio_stream);
        stream.play();
    }

    pub fn get_next_notes(&self) -> Option<TimedNote> {
        if let Some(song) = &self.song {
            song.first().cloned()
        } else {
            None
        }
    }

    pub fn spawn_notes(&mut self, notes: &TimedNote) {
        let line = &notes.line;
        let one = &line.line.0;
        let two = &line.line.1;
        let three = &line.line.2;
        let four = &line.line.3;
        let ref_self = self.to_gd().clone();
        let note_scene = self.note_scene.as_ref().expect("Error unwrapping note scene");
        if let Some(zone_one) = &mut self.spawn_zone_one {
            if *one != NoteType::Empty {
                zone_one.bind_mut().spawn_note(note_scene, notes.timestamp, one.clone(), ref_self.clone());
            }
        }
        if let Some(zone_two) = &mut self.spawn_zone_two {
            if *two != NoteType::Empty {
                zone_two.bind_mut().spawn_note(note_scene, notes.timestamp, two.clone(), ref_self.clone());
            }
        }
        if let Some(zone_three) = &mut self.spawn_zone_three {
            if *three != NoteType::Empty {
                zone_three.bind_mut().spawn_note(note_scene, notes.timestamp, three.clone(), ref_self.clone());
            }
        }
        if let Some(zone_four) = &mut self.spawn_zone_four {
            if *four != NoteType::Empty {
                zone_four.bind_mut().spawn_note(note_scene, notes.timestamp, four.clone(), ref_self);
            }
        }
    }
}

#[godot_api]
impl INode for Spawner {
    fn init(base: Base<Node>) -> Self {
        Self {
            player_height:None,
            player_base_position:None,
            left_x:None,
            right_x:None,
            song:None,
            time:0.0,
            playing:false,
            seconds_ahead_to_spawn: 10,
            note_speed: 1,
            time_before_fail_ms: 100,
            spawn_zone_one: None,
            spawn_zone_two: None,
            spawn_zone_three: None,
            spawn_zone_four: None,
            note_scene: None,
            note_fail_scene: None,
            note_success_scene: None,
            audio_stream: None,
            song_title: None,
            song_max_combo: None,
            song_difficulty: None,
            scorer: None,
            finish_menu: None,
            settings_gear: None,
            base
        }
    }

    fn process(&mut self, _delta: f64) {
        let mut audio_stream = self.audio_stream.as_mut().expect("No valid audio stream").clone();
        let time = audio_stream.get_playback_position() as f64 + AudioServer::singleton().get_time_since_last_mix();
        if self.playing {
            self.time = time as f32;
            if let Some(next_notes) = self.get_next_notes() {
                if next_notes.timestamp < self.time + self.seconds_ahead_to_spawn as f32 {
                    //play
                    self.song.as_mut().expect("Unknown error").remove(0);
                    self.spawn_notes(&next_notes);
                }
            }
        }

        if self.time as f64 > audio_stream.get_stream().expect("No stream").get_length() {
            self.settings_gear.as_mut().expect("No settings gear").set_visible(false);
            let finish = self.finish_menu.as_mut().expect("No finish menu attached");
            finish.set_visible(true);
            let scorer = self.scorer.as_ref().expect("No scorer");
            let score = scorer.bind().score;
            let combo = scorer.bind().max_combo;
            finish.bind_mut().setup(
                true,
                combo >= self.song_max_combo.expect("Song max combo invalid"),
                score.into(),
                combo,
                self.song_title.as_ref().expect("Song title invalid").to_string(),
                self.song_difficulty.expect("Song difficulty invalid")
            );
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_pressed() {
            if event.is_action("left") {
                self.spawn_zone_one.as_mut().expect("Zone not found").bind_mut().process_hit(self.time, self.time_before_fail_ms as f32 / 1000.0, self.note_success_scene.clone());
            } else if event.is_action("up_right") {
                self.spawn_zone_two.as_mut().expect("Zone not found").bind_mut().process_hit(self.time, self.time_before_fail_ms as f32 / 1000.0, self.note_success_scene.clone());
            } else if event.is_action("up_left") {
                self.spawn_zone_three.as_mut().expect("Zone not found").bind_mut().process_hit(self.time, self.time_before_fail_ms as f32 / 1000.0, self.note_success_scene.clone());
            } else if event.is_action("right") {
                self.spawn_zone_four.as_mut().expect("Zone not found").bind_mut().process_hit(self.time, self.time_before_fail_ms as f32 / 1000.0, self.note_success_scene.clone());
            }
        }
    }
}
