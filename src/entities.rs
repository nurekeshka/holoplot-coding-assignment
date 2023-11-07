use crate::utils::Position;

pub struct AudioObject {
    pub id: usize,
    pub position: Position,
}

impl Clone for AudioObject {
    fn clone(&self) -> AudioObject {
        AudioObject {
            id: self.id,
            position: self.position.clone(),
        }
    }
}
