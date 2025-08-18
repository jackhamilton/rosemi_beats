use godot::prelude::*;

struct RustExtension;

mod scene_root;
mod game_object;
mod node_spawner;
mod step_converter;
mod loader;
mod spawn_zone;
mod note;
mod fail_note;

#[cfg(test)]
mod tests;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
