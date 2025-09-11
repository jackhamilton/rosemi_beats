use crate::gdrust_trinkets::ui::grid::grid_size::UnitSize;
use crate::gdrust_trinkets::libs::traits::identifiable::Identifiable;
use crate::gdrust_trinkets::ui::grid::grid_size::UnitSizable;
use godot::obj::bounds::DeclUser;
use godot::prelude::*;
use std::fmt::Display;

// Holds a reference to a GD shared pointer. Useful to implement traits on Gd<T>.
pub struct GdWrapper<T> where T: GodotClass {
    pub pointer: Gd<T>,
    pub id: i64
}

impl<T> Identifiable for GdWrapper<T> where T: GodotClass {
    fn id(&self) -> i64 {
        self.id
    }
}

impl<T> PartialEq for GdWrapper<T> where T: GodotClass {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> UnitSizable for GdWrapper<T> where T: UnitSizable, T: GodotClass<Declarer = DeclUser> {
    fn size(&self) -> UnitSize {
        return self.pointer.bind().size();
    }
}

impl<T> Clone for GdWrapper<T> where T: UnitSizable, T: GodotClass<Declarer = DeclUser> {
    fn clone(&self) -> Self {
        Self {
            pointer: self.pointer.clone(),
            id: self.id
        }
    }
}

impl<T> Display for GdWrapper<T> where T: UnitSizable, T: GodotClass {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(formatter, "{:0>8}", self.id)
    }
}
