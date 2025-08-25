use godot::prelude::*;

struct RustExtension;

pub mod libs;
pub mod nodes;
pub mod objects;
pub mod ui;
pub mod step_converter;
pub mod loader;
pub mod save;

#[cfg(test)]
mod tests;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
