use godot::global::godot_print;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Song {
    pub bpm: f32,
    pub difficulty: u8,
    pub beats: Vec<Beat>
}

impl Song {
    pub fn rasterize(self) -> Vec<TimedNote> {
        let mut timed_lines = vec![];
        let mut c_time = 0.0;
        for beat in self.beats {
            // not accounting for how many beats per measure
            let mut partial_beat_time = 60.0/self.bpm;
            let note_count = beat.notes.len() as f32;
            let measure_beats: f32 = 4.0;
            partial_beat_time = (partial_beat_time * measure_beats) / note_count;
            for line in beat.notes {
                timed_lines.push(TimedNote {
                    timestamp: c_time,
                    line
                });
                c_time += partial_beat_time;
            }
        }
        timed_lines
    }

    pub fn from_str(str: &'static str, bpm: f32) -> Vec<Song> {
        // Separate song difficulties
        let songstart_regex = Regex::new(r"(?m)^//.*dance-single.*$").expect("Failed to construct songstart regex");
        let results: Vec<usize> = songstart_regex.find_iter(str).map(|m| m.start()).collect();
        let mut blocks: Vec<(usize, usize)> = vec![];
        let mut last: Option<usize> = None;
        for result in results {
            if let Some(loc) = last {
                blocks.push((loc, result));
            }
            last = Some(result);
        }
        if let Some(loc) = last {
            blocks.push((loc, str.len()));
        }

        // Song separated, parse each block
        let line_num_regex = Regex::new(r"(?m)^([0-9]){4}").expect("Failed to construct line num regex");
        let mut songs: Vec<Song> = vec![];
        for block in blocks {
            let hunk = &str[block.0..block.1];
            let mut difficulty: u8 = u8::MAX;
            if hunk.matches("Beginner").count() > 0 {
                difficulty = 0;
            } else if hunk.matches("Easy").count() > 0 {
                difficulty = 1;
            } else if hunk.matches("Medium").count() > 0 {
                difficulty = 2;
            } else if hunk.matches("Hard").count() > 0 {
                difficulty = 3;
            }
            // Get first beat
            let results: Vec<usize> = line_num_regex.find_iter(hunk).map(|m| m.start()).collect();
            let result: usize = *results.first().expect("No match for start of block");
            let narrow_hunk = &hunk[result..hunk.len()];
            let mut beats: Vec<Beat> = vec![];
            for result in narrow_hunk.split(",") {
                // godot_print!("Parsing measure: {:?}", result);
                let result = result.replace('\r', "");
                let split = result.split("\n");
                let mut lines: Vec<Line> = vec![];
                for item in split.clone() {
                    // godot_print!("Parsing line: {:?}", result);
                    if line_num_regex.is_match(item) {
                        // godot_print!("matched regex");
                        let line = Line {
                            line: (
                                item.chars().nth(0).expect("Expected character").into(),
                                item.chars().nth(1).expect("Expected character").into(),
                                item.chars().nth(2).expect("Expected character").into(),
                                item.chars().nth(3).expect("Expected character").into()
                            )
                        };
                        lines.push(line);
                    }
                }
                beats.push(Beat {
                    notes: lines
                });
            }
            songs.push(Song {
                bpm,
                beats,
                difficulty
            });
        }
        songs
    }
}
#[derive(Debug, Clone)]
pub struct Beat {
    pub notes: Vec<Line>
}

#[derive(Debug, Clone)]
pub struct Line {
    pub line: (NoteType, NoteType, NoteType, NoteType),
}

#[derive(Debug, Clone)]
pub struct TimedNote {
    pub timestamp: f32,
    pub line: Line
}

#[derive(Debug, Clone, PartialEq)]
pub enum NoteType {
    Empty = 0,
    Single = 1,
    HoldStart = 2,
    HoldEnd = 3,
    Mine = 4
}

impl From::<char> for NoteType {
    fn from(value: char) -> Self {
        if let Some(item) = value.to_digit(10) {
            return (item as u8).into();
        }
        NoteType::Empty
    }
}

impl From::<u8> for NoteType {
    fn from(value: u8) -> Self {
        match value {
            1 => NoteType::Single,
            2 => NoteType::HoldStart,
            3 => NoteType::HoldEnd,
            4 => NoteType::Mine,
            _ => NoteType::Empty
        }
    }
}
