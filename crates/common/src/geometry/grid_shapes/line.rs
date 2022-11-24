use std::ops::Sub;

use super::grid_shape::*;
use crate::prelude::*;
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Line {
    start: IVec2,
    end: IVec2,
}
impl Line {
    pub fn new(start: impl Point2d, end: impl Point2d) -> Self {
        println!("Line from: {:?} to: {:?}", start.as_ivec2(), end.as_ivec2());
        Self {
            start: start.as_ivec2(),
            end: end.as_ivec2(),
        }
    }
}
impl GridShape for Line {
    fn get_count(&self) -> usize { self.get_points().len() }

    fn contains(&self, point: impl Point2d) -> bool { self.get_points().contains(&point.as_ivec2()) }

    fn get_points(&self) -> HashSet<IVec2> {
        let mut discovered = HashSet::new();
        let max_delta = self.start.sub(self.end).abs().max_element();
        for step in 0..=max_delta {
            let percent = if max_delta == 0 { 0.0 } else { step as f32 / max_delta as f32 };
            discovered.insert(self.start.lerp(self.end, percent));
        }
        discovered
    }
}
