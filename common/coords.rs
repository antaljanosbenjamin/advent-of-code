use std::ops::Add;

// x is horizontal position
// y is vertical position

// [x, y]
pub type Coords<T = usize> = [T; 2];

pub trait CoordsImpl<T>
where
    Self: Sized,
    T: Add<Output = T>,
{
    fn new(x: T, y: T) -> Self;
    fn x(&self) -> T;
    fn y(&self) -> T;

    fn horizontal(&self, other: &Self) -> bool;
    fn vertical(&self, other: &Self) -> bool;

    fn parallel_to_axles(&self, other: &Self) -> bool {
        return self.horizontal(other) || self.vertical(other);
    }

    fn add(&self, other: &Self) -> Self {
        Self::new(self.x() + other.x(), self.y() + other.y())
    }
}

impl<T> CoordsImpl<T> for Coords<T>
where
    T: Add<Output = T> + Eq + Copy,
{
    fn new(x: T, y: T) -> Self {
        [x, y]
    }

    fn x(&self) -> T {
        self[0]
    }

    fn y(&self) -> T {
        self[1]
    }

    fn horizontal(&self, other: &Self) -> bool {
        self.y() == other.y()
    }

    fn vertical(&self, other: &Self) -> bool {
        self.x() == other.x()
    }
}
