use godot::prelude::*;

struct RustExtension;

mod scene_root;
mod game_object;
mod node_spawner;
mod step_converter;
mod loader;
mod spawn_zone;
mod note;
mod note_animation;
mod scorer;
mod song_load_screen;
mod song_cell;
mod difficulty_button;
mod song_display_screen;

#[cfg(test)]
mod tests;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
