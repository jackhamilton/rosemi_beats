pub trait UnitSizable {
    fn size(&self) -> UnitSize;
}

#[derive(Default, PartialEq, Debug)]
pub struct UnitSize {
    pub x: i16,
    pub y: i16,
}

impl UnitSize {
    pub const ZERO: UnitSize = UnitSize { x: 0, y: 0 };
    pub const ONE: UnitSize = UnitSize { x: 1, y: 1 };
    pub const TWO: UnitSize = UnitSize { x: 2, y: 2 };
}

impl Clone for UnitSize {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y
        }
    }
}
