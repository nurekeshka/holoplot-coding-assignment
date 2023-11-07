pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Clone for Position {
    fn clone(&self) -> Position {
        Position {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}
