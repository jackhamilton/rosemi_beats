use godot::prelude::*;

#[derive(GodotClass, Debug)]
#[class(base=Node2D)]
pub struct NoteAnimation {
    pub base: Base<Node2D>
}

#[godot_api]
impl INode2D for NoteAnimation {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            base
        }
    }
}
