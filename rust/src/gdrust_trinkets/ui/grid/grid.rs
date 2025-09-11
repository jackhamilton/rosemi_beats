use crate::gdrust_trinkets::ui::grid::grid_size::UnitSize;
use crate::gdrust_trinkets::ui::grid::grid_position::GridPosition;
use crate::gdrust_trinkets::libs::traits::identifiable::Identifiable;
use crate::gdrust_trinkets::ui::grid::grid_size::UnitSizable;
use godot::prelude::*;
use std::collections::HashMap;
use std::fmt::Display;

/// Grids are zero-indexed, with the size being exclusive.
/// The i64 refers to an ID, which is garnered from the Identifiable conformance.
pub struct Grid<T> where T: UnitSizable, T: PartialEq, T: Identifiable {
    grid: Vec<Vec<Option<i64>>>,
    items: HashMap<i64, T>,
    positions: HashMap<i64, GridPosition>
}

impl<T> Grid<T> where T: UnitSizable, T: PartialEq, T: Identifiable {
    pub fn new(x: usize, y: usize) -> Grid<T> {
        let mut outer: Vec<Vec<Option<i64>>> = Vec::new();
        for _ in 0..y {
            let mut inner: Vec<Option<i64>> = Vec::new();
            inner.resize(x, None);
            outer.push(inner);
        }
        Grid::<T> {
            grid: outer,
            items: HashMap::new(),
            positions: HashMap::new()
        }
    }

    pub fn can_insert(&self, item: &T, at: &GridPosition) -> Result<(), InsertionError> {
        let size = self.size();
        // Make sure the i7s aren't over 255
        // Make sure we're not trying to insert it at an index greater than the grid's current size
        if at.x >= size.x || at.y >= size.y {
            return Err(InsertionError::OutOfBounds)
        }
        let intended_size = item.size();

        // Make sure it fits
        let max_x = at.x + intended_size.x;
        let max_y = at.y + intended_size.y;
        if max_x > size.x || max_y > size.y {
            return Err(InsertionError::DoesntFit)
        }

        // Check for anything under it
        let mut placement_valid = true;
        let mut conflict: Vec<i64> = Vec::new();
        for i in at.y..max_y {
            for j in at.x..max_x {
                let valid_placement = match self.grid.get(i as usize) {
                    None => return Err(InsertionError::Unknown),
                    Some(arr) => match arr.get(j as usize) {
                        None => return Err(InsertionError::Unknown),
                        Some(optional) => {
                            match optional {
                                Some(value) => {
                                    conflict.push(*value);
                                    false
                                },
                                None => true
                            }
                        }
                    }
                };
                placement_valid &= valid_placement;
            }
        }

        if !placement_valid {
            return Err(InsertionError::Conflicts(conflict))
        }

        // Ensure it's not already present
        for arr in &self.grid {
            if arr.contains(&Some(item.id())) {
                return Err(InsertionError::AlreadyPresent);
            }
        }

        Ok(())
    }

    pub fn insert(&mut self, item: T, at: GridPosition) -> Result<(), InsertionError> {
        let can_insert = self.can_insert(&item, &at);
        if can_insert.is_ok() {
            let intended_size = item.size();

            // Make sure it fits
            let max_x = at.x + intended_size.x;
            let max_y = at.y + intended_size.y;

            // Valid placement, insert the item
            let id = item.id();
            self.items.insert(id, item);
            for i in at.y..max_y {
                for j in at.x..max_x {
                    self.grid[i as usize][j as usize] = Some(id);
                }
            }
            self.positions.insert(id, at);
            Ok(())
        } else {
            can_insert
        }
    }

    pub fn remove(&mut self, at: GridPosition) -> bool where T: Display {
        godot_print!("Removing {}. Before {}", at, self);
        let pos_x = at.x;
        let pos_y = at.y;
        if self.has(at) {
            if let Some(id) = self.grid[pos_y as usize][pos_x as usize] {
                for arr in &mut self.grid {
                    let mut grid_indices = Vec::<usize>::new();
                    for (ind, item) in arr.iter().enumerate() {
                        if *item == Some(id) {
                            grid_indices.push(ind);
                        }
                    }
                    for index in grid_indices {
                        arr[index] = None;
                    }
                }
                self.items.remove(&id);
                self.positions.remove(&id);
            }

            godot_print!("After {}", self);
            true
        } else {
            godot_print!("Failed");
            false
        }
    }

    pub fn has(&self, position: GridPosition) -> bool {
        let id = self.grid[position.y as usize][position.x as usize];
        id.is_some()
    }

    pub fn get(&self, position: GridPosition) -> Option<&T> {
        let id = self.grid[position.y as usize][position.x as usize];
        match id {
            Some(id) => self.items.get(&id),
            _ => None
        }
    }

    pub fn get_mut(&mut self, position: GridPosition) -> Option<&mut T> {
        let id = self.grid[position.y as usize][position.x as usize];
        match id {
            Some(id) => self.items.get_mut(&id),
            _ => None
        }
    }

    pub fn get_position(&self, item: T) -> Option<&GridPosition> {
        let id = item.id();
        self.positions.get(&id)
    }
}

#[derive(Debug, PartialEq)]
pub enum InsertionError {
    AlreadyPresent,
    Conflicts(Vec<i64>),
    DoesntFit,
    OutOfBounds,
    Unknown
}

impl Display for InsertionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            InsertionError::AlreadyPresent => write!(formatter, "(AlreadyPresent)"),
            InsertionError::Conflicts(conflicts) => {
                for conflict in conflicts.iter() {
                    let _ = write!(formatter, "(ConflictError: with {})", conflict);
                }
                Ok(())
            }
            InsertionError::DoesntFit => write!(formatter, "(DoesntFit)"),
            InsertionError::OutOfBounds => write!(formatter, "(OutOfBounds)"),
            InsertionError::Unknown => write!(formatter, "(Unknown)"),
        }
    }
}

impl<T> Display for Grid<T> where T: UnitSizable, T: PartialEq, T: Identifiable, T: Display {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let _ = write!(formatter, "\nGrid: ");
        for arr in &self.grid {
            let _ = writeln!(formatter);
            for item in arr {
                if item.is_some() {
                    let _ = write!(formatter, "{} ", item.expect("Unexpected none"));
                } else {
                    let _ = write!(formatter, "None     ");
                }
            }
        }
        let _ = writeln!(formatter);
        Ok(())
    }
}

/// The grid itself can give you its grid size
impl<T> UnitSizable for Grid<T> where T: UnitSizable, T: PartialEq, T: Identifiable {
    fn size(&self) -> UnitSize {
        let mut x = 0;
        if let Some(item) = &self.grid.first() {
            x = item.len();
        }
        UnitSize {
            x: x as i16,
            y: self.grid.len() as i16
        }
    }
}
