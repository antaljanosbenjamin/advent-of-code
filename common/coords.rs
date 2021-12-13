// +--> x
// |
// |
// V
// y

// [x, y]
pub type Coords = [usize; 2];

pub trait CoordsImpl {
    fn new(x: usize, y: usize) -> Self;
    fn x(&self) -> usize;
    fn y(&self) -> usize;

    fn horizontal(&self, other: &Self) -> bool;
    fn vertical(&self, other: &Self) -> bool;

    fn parallel_to_axles(&self, other: &Self) -> bool {
        return self.horizontal(other) || self.vertical(other);
    }
}

impl CoordsImpl for Coords {
    fn new(x: usize, y: usize) -> Self {
        [x, y]
    }

    fn x(&self) -> usize {
        self[0]
    }

    fn y(&self) -> usize {
        self[1]
    }

    fn horizontal(&self, other: &Self) -> bool {
        self.y() == other.y()
    }

    fn vertical(&self, other: &Self) -> bool {
        self.x() == other.x()
    }
}
