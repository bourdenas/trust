use std::ops::MulAssign;

use sdl2::rect::Point;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Dirty;

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Id(pub String);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Point);

impl Default for Position {
    fn default() -> Self {
        Position(Point::new(0, 0))
    }
}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    pub resource: String,
    pub frame_index: usize,
    pub scaling: ScalingVec,
}

#[derive(Component, Debug)]
pub struct ScalingVec {
    pub x: f64,
    pub y: f64,
}

impl ScalingVec {
    pub fn new(x: f64, y: f64) -> Self {
        ScalingVec { x, y }
    }
}

impl Default for ScalingVec {
    fn default() -> Self {
        ScalingVec::new(1.0, 1.0)
    }
}

/// Vector position product implemented as [lhs.x * rhs.x, lhs.y * rhs.y].
impl MulAssign for ScalingVec {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl PartialEq for ScalingVec {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
