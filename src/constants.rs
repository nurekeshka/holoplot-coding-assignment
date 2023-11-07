use crate::utils::Position;

pub enum Coordinate {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Coordinate {
    pub fn position(&self) -> Option<Position> {
        match self {
            Coordinate::A => Some(Position {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            Coordinate::B => Some(Position {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }),
            Coordinate::C => Some(Position {
                x: 2.0,
                y: 2.0,
                z: 2.0,
            }),
            Coordinate::D => Some(Position {
                x: 3.0,
                y: 3.0,
                z: 3.0,
            }),
            Coordinate::E => Some(Position {
                x: 4.0,
                y: 4.0,
                z: 4.0,
            }),
            Coordinate::F => Some(Position {
                x: 5.0,
                y: 5.0,
                z: 5.0,
            }),
            Coordinate::G => Some(Position {
                x: 6.0,
                y: 6.0,
                z: 6.0,
            }),
        }
    }
}
