use crate::vec::Vec2ISize as Vec2I;

#[derive(Clone, Copy)]
pub enum GridDirection {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl GridDirection {
    pub fn delta(&self) -> Vec2I {
        use GridDirection::*;
        match self {
            North    => Vec2I( 0, -1),
            South    => Vec2I( 0,  1),
            East     => Vec2I( 1,  0),
            West     => Vec2I(-1,  0),
            NorthEast => North.delta() + East.delta(),
            NorthWest => North.delta() + West.delta(),
            SouthEast => South.delta() + East.delta(),
            SouthWest => South.delta() + West.delta(),
        }
    }
}
