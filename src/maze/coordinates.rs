use rand::{seq::SliceRandom, Rng};

/// A position in a matrix
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coordinates {
    pub x: i64,
    pub y: i64,
}

impl Coordinates {
    /// Creates a new coordinate based on x and y axis.
    #[must_use]
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    /// Gets a [`Coordinates`] moving the index based on [`Direction`].
    pub fn next(&self, direction: Direction) -> Self {
        Self {
            x: self.x
                + match direction {
                    Direction::East | Direction::Northeast | Direction::Southeast => 1,
                    Direction::West | Direction::Northwest | Direction::Southwest => -1,
                    _ => 0,
                },
            y: self.y
                + match direction {
                    Direction::North | Direction::Northeast | Direction::Northwest => -1,
                    Direction::South | Direction::Southeast | Direction::Southwest => 1,
                    _ => 0,
                },
        }
    }

    /// Calculates the euclidean distance between two points.
    pub fn euclidean_dist(&self, other: &Self) -> f32 {
        f32::sqrt((self.x - other.x).pow(2) as f32 + (self.y - other.y).pow(2) as f32)
    }
}

/// The four cardinal directions
///
/// Also defines convenience functions to work with them.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    North,
    Northeast,
    Northwest,
    East,
    West,
    South,
    Southeast,
    Southwest,
}

impl Direction {
    /// Return the opposite direction of self
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::Northeast => Direction::Southwest,
            Direction::Northwest => Direction::Southeast,
            Direction::Southeast => Direction::Northwest,
            Direction::Southwest => Direction::Northeast,
        }
    }

    /// Generate a list of all collections but in random order
    pub fn gen_random_order(rng: &mut impl Rng) -> [Direction; 4] {
        let mut directions = Self::all();
        directions.shuffle(rng);
        directions
    }

    /// Return all directions as array
    pub fn all() -> [Direction; 4] {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Direction::North => "North",
            Direction::East => "East",
            Direction::South => "South",
            Direction::West => "West",
            Direction::Northeast => "Northeast",
            Direction::Northwest => "Northwest",
            Direction::Southeast => "Southeast",
            Direction::Southwest => "Southwest",
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::coordinates::Direction;

    use super::Coordinates;

    #[test]
    fn test_direction_move() {
        let coordinates = Coordinates::new(1, 1);

        assert_eq!(Coordinates::new(2, 1), coordinates.next(Direction::East));
        assert_eq!(Coordinates::new(0, 1), coordinates.next(Direction::West));
        assert_eq!(Coordinates::new(1, 2), coordinates.next(Direction::South));
        assert_eq!(Coordinates::new(1, 0), coordinates.next(Direction::North));
        assert_eq!(
            Coordinates::new(2, 2),
            coordinates.next(Direction::Southeast)
        );
        assert_eq!(
            Coordinates::new(0, 2),
            coordinates.next(Direction::Southwest)
        );
        assert_eq!(
            Coordinates::new(2, 0),
            coordinates.next(Direction::Northeast)
        );
        assert_eq!(
            Coordinates::new(0, 0),
            coordinates.next(Direction::Northwest)
        );
    }
}
