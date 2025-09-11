use godot::prelude::*;

struct RustExtension;

pub mod nodes;
pub mod objects;
pub mod ui;
pub mod step_converter;
pub mod loader;
pub mod gdrust_trinkets;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
