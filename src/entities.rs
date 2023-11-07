use crate::utils::Position;

pub struct AudioObject {
    pub id: usize,
    pub position: Position,
}

impl AudioObject {
    pub fn new(id: usize, position: Position) -> AudioObject {
        AudioObject { id, position }
    }
}

impl Clone for AudioObject {
    fn clone(&self) -> AudioObject {
        AudioObject {
            id: self.id,
            position: self.position.clone(),
        }
    }
}
