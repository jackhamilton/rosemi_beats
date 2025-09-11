use crate::gdrust_trinkets::ui::grid::grid_size::UnitSize;
use godot::prelude::Vector2;
use std::fmt::Display;

#[derive(Default, Debug, Clone)]
pub struct GridPosition {
    pub x: i16,
    pub y: i16,
}

impl GridPosition {
    pub fn copy(&self) -> Self {
        GridPosition {
            x: self.x,
            y: self.y
        }
    }
}

impl std::ops::Sub for GridPosition {
    type Output = GridPosition;
    fn sub(self, item: GridPosition) -> GridPosition {
        GridPosition {
            x: self.x - item.x,
            y: self.y - item.y
        }
    }
}

impl std::ops::Mul<UnitSize> for GridPosition {
    type Output = GridPosition;
    fn mul(self, item: UnitSize) -> GridPosition {
        GridPosition {
            x: self.x * item.x,
            y: self.y * item.y
        }
    }
}

impl std::convert::From<GridPosition> for Vector2 {
    fn from(val: GridPosition) -> Self {
        Vector2 {
            x: val.x as f32,
            y: val.y as f32
        }
    }
}

impl Display for GridPosition {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "({}, {})", self.x, self.y)
    }
}
