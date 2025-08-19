use godot::prelude::*;

struct RustExtension;

pub mod nodes;
pub mod objects;
pub mod ui;
pub mod step_converter;
pub mod loader;

#[cfg(test)]
mod tests;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
