use crate::prelude::*;
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Triangle {
    start: Position,
    point1: Position,
    point2: Position,
}
impl Triangle {
    pub fn new(start: Position, point1: Position, point2: Position) -> Self {
        Self {
            start: start.as_ivec2(),
            point1: point1.as_ivec2(),
            point2: point2.as_ivec2(),
        }
    }
}

impl Shape for Triangle {
    // TODO: check for bugs, this is just a quick implementation I thought of.
    fn get_positions(&self) -> HashSet<Position> {
        let mut discovered = HashSet::new();
        for end_point in Line::new(self.point1, self.point2).get_positions() {
            for current_point in Line::new(self.start, end_point) {
                discovered.insert(current_point);
            }
        }
        discovered
    }

    fn contains(&self, position: Position) -> bool { self.get_positions().contains(&position) }

    // FIX: PERF
    fn get_count(&self) -> u32 { self.get_positions().len() as u32 }
}

impl ShapeWithBorder for Triangle {
    fn get_border_positions(&self) -> HashSet<Position> {
        let mut discovered = HashSet::new();
        for point in Line::new(self.start, self.point1) {
            discovered.insert(point);
        }
        for point in Line::new(self.point1, self.point2) {
            discovered.insert(point);
        }
        for point in Line::new(self.point2, self.start) {
            discovered.insert(point);
        }
        discovered
    }

    fn border_contains(&self, position: Position) -> bool {
        self.get_border_positions().contains(&position)
    }

    fn get_border_count(&self) -> usize { self.get_border_positions().len() }
}
