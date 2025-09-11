use crate::gdrust_trinkets::libs::gd_wrapper::GdWrapper;
use crate::gdrust_trinkets::ui::grid::grid_display::GridItemDisplay;
use crate::gdrust_trinkets::ui::grid::grid_container::ContainerSystem;
use crate::gdrust_trinkets::ui::grid::grid_item::GridItem;
use godot::prelude::*;
use godot::classes::InputEvent;
use godot::global;
use godot::classes::Sprite2D;
use godot::classes::InputEventMouse;
use godot::classes::InputEventMouseButton;

type EventCallback = (dyn Fn(&mut Gd<GridItem>));

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Cursor {
    pub containers: ContainerSystem<GridItemDisplay>,

    pub cursor_item: Option<GdWrapper<GridItem>>,
    pub item_offset: Vector2,
    pub last_mouse_position: Vector2,
    pub on_cancel: Option<Box<EventCallback>>,

    base: Base<Node2D>
}

#[godot_api]
impl INode2D for Cursor {
    fn ready(&mut self) {
        self.base_mut().set_z_index(100);
    }

    fn process(&mut self, _delta: f64) {
        // Make a sprite for a cursor item if none exists
        if self.cursor_item.is_some() && self.base().get_children().is_empty() {
            let node = self.cursor_item
                .as_ref()
                .expect("Failed to lock cursor item")
                .clone();
            for child in node.pointer.get_children().iter_shared() {
                if let Ok(item) = child.try_cast::<Sprite2D>() {
                    let copy = item.duplicate().expect("Couldn't copy");
                    let mut sprite_ref = copy.cast::<Sprite2D>();
                    sprite_ref.set_visible(true);
                    let mouse_position = self.last_mouse_position;
                    sprite_ref.set_position(mouse_position + self.item_offset);
                    self.base_mut().add_child(&sprite_ref);
                }
            }
        // Remove the cursor item sprite if we have no cursor item
        } else if self.cursor_item.is_none() && !self.base().get_children().is_empty() {
            let mut base = self.base_mut();
            let children = base.get_children();
            for item in children.iter_shared() {
                base.remove_child(&item);
            }
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Ok(mouse_event) = event.clone().try_cast::<InputEventMouseButton>() {
            // We shouldn't handle any events that a container is going to handle
            if self.containers.handled(mouse_event.clone()) { return }
            if mouse_event.get_button_index() == global::MouseButton::LEFT
                && self.cursor_item.is_some()
                && (mouse_event.is_canceled() || mouse_event.is_released()) {
                self.remove_cursor_item();
            }
        // If we have a cursor item, move it
        } else if let Ok(mouse_event) = &event.try_cast::<InputEventMouse>() {
            if self.cursor_item.is_some() {
                let children = self.base().get_children();
                for item in children.iter_shared() {
                    if let Ok(mut sprite) = item.try_cast::<Sprite2D>() {
                        sprite.set_position(mouse_event.get_position() + self.item_offset);
                    }
                }
            }
            self.last_mouse_position = mouse_event.get_position();
        }
    }
}

impl Cursor {
    pub fn set_cursor_item(&mut self, item: GdWrapper<GridItem>, offset: Vector2) {
        self.cursor_item = Some(item);
        self.item_offset = offset;
    }

    pub fn set_remove_callback(&mut self, function: impl Fn(&mut Gd<GridItem>) + 'static) {
        self.on_cancel = Some(Box::new(function));
    }

    pub fn remove_cursor_item(&mut self) {
        if let Some(on_cancel) = &self.on_cancel {
            if let Some(item) = &self.cursor_item {
                on_cancel(&mut item.pointer.clone())
            }
        }
        self.cursor_item = None
    }
}

