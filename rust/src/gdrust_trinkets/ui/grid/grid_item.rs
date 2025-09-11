use crate::gdrust_trinkets::ui::grid::grid_size::UnitSizable;
use crate::gdrust_trinkets::ui::grid::grid_size::UnitSize;
use godot::prelude::*;
use godot::classes::Node2D;
use godot::classes::Sprite2D;

#[derive(GodotClass, Debug)]
#[class(no_init, base=Node2D)]
// Should be instantiated via template
pub struct GridItem {
    pub sprite: Option<Gd<Sprite2D>>,
    pub size: UnitSize,
    pub hidden: bool,

    pub base: Base<Node2D>
}

#[godot_api]
impl INode2D for GridItem {
    fn enter_tree(&mut self) {
        let sprite = self.sprite.as_mut().expect("No sprite").clone();
        self.base_mut().add_child(&sprite);
    }
}

#[godot_api]
impl GridItem {
    #[func]
    pub fn make(size_x: i16, size_y: i16, sprite: Gd<Sprite2D>) -> Gd<Self> {
        Gd::from_init_fn(|base| {
            Self {
                sprite: Some(sprite.clone()),
                size: UnitSize { x: size_x, y: size_y },
                hidden: false,
                base
            }
        })
   }

    #[func]
    pub fn copy(&self) -> Gd<Node2D> {
        self.base().clone()
    }

    pub fn set_hidden(&mut self, hidden: bool) {
        if let Some(sprite) = &mut self.sprite { sprite.set_visible(!hidden) }
    }
}

// MARK: Traits
impl UnitSizable for GridItem {
    fn size(&self) -> UnitSize {
        self.size.clone()
    }
}
