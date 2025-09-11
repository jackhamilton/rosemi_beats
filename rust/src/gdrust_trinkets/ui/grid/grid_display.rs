use crate::gdrust_trinkets::libs::gd_wrapper::GdWrapper;
use crate::gdrust_trinkets::libs::sprite::Rescalable;
use godot::classes::Sprite2D;
use godot::global::MouseButton;
use crate::gdrust_trinkets::ui::grid::grid_cursor_display::Cursor;
use crate::gdrust_trinkets::ui::grid::grid::InsertionError;
use crate::gdrust_trinkets::libs::traits::identifiable::uuid;
use godot::classes::InputEventMouseButton;
use godot::classes::InputEvent;
use godot::classes::IContainer;
use godot::classes::Container;
use crate::gdrust_trinkets::ui::grid::grid_size::UnitSize;
use crate::gdrust_trinkets::ui::grid::grid_container::ObjectContainer;
use crate::gdrust_trinkets::ui::grid::grid_position::GridPosition;
use crate::gdrust_trinkets::ui::grid::grid_item::GridItem;
use crate::gdrust_trinkets::ui::grid::grid::Grid;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, tool, base=Container)]
pub struct GridItemDisplay {
    #[export]
    x_cells: i16,
    #[export]
    y_cells: i16,

    grid: Option<Grid<GdWrapper<GridItem>>>,
    cursor: Option<Gd<Cursor>>,
    pub x_cell_size: i16,
    pub y_cell_size: i16,

    base: Base<Container>
}

#[godot_api]
impl IContainer for GridItemDisplay {
    fn ready(&mut self) {
        self.grid = Some(
            Grid::<GdWrapper<GridItem>>::new(self.x_cells.try_into().unwrap(), self.y_cells.try_into().unwrap())
        );
        self.x_cell_size = self.get_cell_size_x() as i16;
        self.y_cell_size = self.get_cell_size_y() as i16;
    }

    fn gui_input(&mut self, event: Gd<InputEvent>) {
        if let Ok(mouse_event) = event.try_cast::<InputEventMouseButton>() {
            if mouse_event.get_button_index() == MouseButton::LEFT {
                let event_grid_position = Self::convert_position(self.x_cell_size, self.y_cell_size, &mouse_event.get_position());
                if mouse_event.is_pressed() {
                    self.pressed(event_grid_position, mouse_event);
                } else if mouse_event.is_released() || mouse_event.is_canceled() {
                    let cursor = self.cursor.as_ref().expect("No cursor").bind();
                    let cursor_item = cursor.cursor_item.clone();
                    if let Some(item) = cursor_item {
                        let offset_position = cursor.item_offset * Vector2{ x: -1.0, y: -1.0 } + Vector2 { x: 1.0, y: 1.0 };
                        drop(cursor);
                        let mut root_position: Option<Vector2> = None;
                        for child in item.pointer.get_children().iter_shared() {
                            if let Ok(sprite) = child.try_cast::<Sprite2D>() {
                                let position = sprite.to_global(sprite.get_rect().position);
                                // let scale = sprite.get_scale();
                                root_position = Some(position);
                            }
                        }
                        if let Some(root_pos) = root_position {
                            //Sprite found, continue trying to find cursor offset and therefore
                            //the grid position of the square you're dragging
                            let item_local_pos = item.pointer.to_local(root_pos) + Vector2 { x: 1.0, y: 1.0 };
                            let converted_root = offset_position - item_local_pos;
                            let dragged_sq_grid_position = Self::convert_position(self.x_cell_size, self.y_cell_size, &converted_root);
                            let item_grid_position = self.grid.as_ref().expect("Could not lock grid")
                                .get_position(item).expect("Could not get cursor item position");
                            self.released(event_grid_position, item_grid_position.clone(), dragged_sq_grid_position);
                        } else {
                            // No sprite detected
                            let item_grid_position = self.grid.as_ref().expect("Could not lock grid")
                                .get_position(item).expect("Could not get cursor item position");
                            self.released(event_grid_position, item_grid_position.clone(), GridPosition { x: 0, y: 0 });
                        }
                    }
                }
            }
        }
    }
}

impl ObjectContainer for GridItemDisplay {
    fn will_handle(&self, event: Gd<InputEventMouseButton>) -> bool {
        let base = self.base();
        base.get_global_rect().contains_point(event.get_global_position())
    }
}

#[godot_api]
impl GridItemDisplay {
    #[func]
    fn insert(&mut self, mut card: Gd<GridItem>, x: i16, y: i16) -> bool {
        let card_x = (x * self.x_cell_size) as f32;
        let card_y = (y * self.y_cell_size) as f32;
        let card_mut = card.bind_mut();
        let mut sprite = card_mut.sprite.clone().expect("Could not get reference to sprite");
        drop(card_mut);
        let size = card.bind().size.clone();
        sprite.scale_fill(self.x_cell_size * size.x, self.y_cell_size * size.y);
        let container = GdWrapper {
            pointer: card.clone(),
            id: uuid()
        };
        let mut base = self.base_mut();
        base.add_child(&card);
        drop(base);
        let sprite_offset = self.center_for_size(size.x as f32, size.y as f32);
        card.set_position(Vector2{x: card_x + sprite_offset.x, y: card_y + sprite_offset.y});
        self.grid.as_mut().unwrap().insert(container, GridPosition {
            x,
            y
        }).is_ok()
    }

    #[func]
    fn set_cursor(&mut self, cursor: Gd<Cursor>) {
        self.cursor = Some(cursor);

        let self_instance = self.base().instance_id();
        let pointer_to_self = Gd::from_instance_id(self_instance);

        // Add self as a container to the cursor
        self.cursor.as_mut().expect("No cursor")
            .bind_mut()
            .containers
            .containers
            .push(pointer_to_self);
    }

    fn pressed(&mut self, position: GridPosition, event: Gd<InputEventMouseButton>) {
        self.set_hidden(position.clone(), true);
        if let Some(item) = self.grid.as_mut().expect("No grid found").get_mut(position.clone()) {
            let cursor = self.cursor.as_mut().expect("Cursor not found!");
            let offset = item.pointer.get_position() - event.get_position();
            let mut cursor = cursor.bind_mut();
            cursor.set_cursor_item(item.clone(), offset);
            cursor.set_remove_callback(move |node| {
                let parent = node.get_parent().expect("Cursor item has no parent");
                if let Ok(mut card_display) = parent.try_cast::<GridItemDisplay>() {
                    card_display.bind_mut().set_hidden(position.clone(), false);
                }
            })
        }
    }

    pub fn released(&mut self, position: GridPosition, previous_item_position: GridPosition, item_grid_offset: GridPosition) {
        let position_adjusted = position - item_grid_offset;
        godot_print!("Released, position: {}, previous: {}", position_adjusted, previous_item_position);
        let mut cursor = self.cursor.as_mut().expect("Cursor not found").bind_mut();
        let grid = self.grid.as_mut().expect("Could not lock grid");
        let mut grid_item = grid.get(previous_item_position.clone()).expect("Could not dereference cursor item").clone();
        let result = grid.can_insert(&grid_item, &position_adjusted);
        if result.is_ok()
        || Err(InsertionError::AlreadyPresent) == result
        || Err(InsertionError::Conflicts(vec![grid_item.id])) == result {
            cursor.on_cancel = None;
            cursor.remove_cursor_item();
            drop(cursor);
            let current_card_position = grid_item.pointer.get_position();
            let shift_offset = position_adjusted.clone() - previous_item_position.clone();
            let cell_size = UnitSize {
                x: self.x_cell_size,
                y: self.y_cell_size
            };
            let full_shift_offset = shift_offset * cell_size;
            grid.remove(previous_item_position.clone());
            grid_item.pointer.set_position(current_card_position + full_shift_offset.into());
            let _ = grid.insert(grid_item, position_adjusted.clone());
            self.set_hidden(position_adjusted, false);
        } else {
            godot_print!("Error: {}", result.expect_err("No error"));
            cursor.on_cancel = None;
            cursor.remove_cursor_item();
            drop(cursor);
            self.set_hidden(previous_item_position, false);
        }
    }

    fn set_hidden(&mut self, position: GridPosition, hidden: bool) {
        if let Some(item) = self.grid.as_mut().expect("No grid found").get_mut(position) {
            let pointer = &mut item.pointer;
            pointer.bind_mut().set_hidden(hidden);
        }
    }

    fn convert_position(x_cell_size: i16, y_cell_size: i16, pos: &Vector2) -> GridPosition {
        let x: f32 = pos.x / x_cell_size as f32;
        let y: f32 = pos.y / y_cell_size as f32;
        GridPosition {
            x: x.floor() as i16,
            y: y.floor() as i16
        }
    }

    fn center_for_size(&self, x: f32, y: f32) -> Vector2 {
        let x_size = self.x_cell_size;
        let y_size = self.y_cell_size;
        Vector2 {x: (x_size as f32 / 2.0) * x, y: (y_size as f32 / 2.0) * y}
    }

    fn get_cell_size_x(&self) -> f32 {
        let size_x: f32 = self.base().get_size().x;
        let cells_x: f32 = self.x_cells.into();
        size_x / cells_x
    }

    fn get_cell_size_y(&self) -> f32 {
        let size_y: f32 = self.base().get_size().y;
        let cells_y: f32 = self.y_cells.into();
        size_y / cells_y
    }
}
