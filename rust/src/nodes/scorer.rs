use godot::{classes::{AudioStream, AudioStreamPlaybackPolyphonic, AudioStreamPlayer, AudioStreamPolyphonic}, prelude::*};

#[derive(GodotClass, Debug)]
#[class(base=Node)]
pub struct Scorer {
    #[var]
    pub score: i32,
    #[var]
    pub combo: i32,
    #[var]
    pub max_combo: i32,
    #[export]
    pub hit_audio_stream: Option<Gd<AudioStreamPlayer>>,
    #[export]
    pub hit_audio_resource: Option<Gd<AudioStream>>,

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
        if self.combo > self.max_combo {
            self.max_combo = self.combo;
        }

        let stream = self.hit_audio_stream.as_mut().expect("Error: no attached audio stream");
        let playback = stream.get_stream_playback().expect("Could not retrieve playback");
        let mut polyphonic = playback.try_cast::<AudioStreamPlaybackPolyphonic>().expect("Could not retrieve polyphonic audio");
        let resource = self.hit_audio_resource.as_ref().expect("No hit audio resource provided");
        polyphonic.play_stream_ex(resource).bus("SFX").done();
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
            max_combo: 0,
            hit_audio_stream: None,
            hit_audio_resource: None,
            base
        }
    }

    fn enter_tree(&mut self) {
        let stream = self.hit_audio_stream.as_mut().expect("Error: no attached audio stream");
        stream.set_bus("SFX");
        stream.set_max_polyphony(32);
        stream.set_stream(&AudioStreamPolyphonic::new_gd());
    }
}
