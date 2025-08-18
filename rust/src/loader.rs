pub struct Loader {}

#[derive(Debug)]
pub struct IncludedSong {
    pub text: &'static str,
    pub bpm: f32
}

impl Loader {
    pub fn get_res() -> Vec<IncludedSong> {
        let files = vec![
            IncludedSong {
                text: include_str!("res/theme.sm"),
                bpm: 120.0
            }
        ];
        files
    }
}
