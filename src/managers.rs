use crate::entities::AudioObject;
use std::collections::{HashMap, VecDeque};

pub struct AudioObjectManager {
    pub objects: HashMap<usize, AudioObject>,
    history: VecDeque<AudioObjectManager>,
    current: usize,
    counter: usize,
}

impl AudioObjectManager {
    pub fn get(&mut self, id: usize) -> Option<AudioObject> {
        return self.objects.get(&id).map(|obj| obj.clone());
    }

    pub fn contains(&mut self, id: usize) -> bool {
        return self.objects.contains_key(&id);
    }
}

impl Clone for AudioObjectManager {
    fn clone(&self) -> AudioObjectManager {
        AudioObjectManager {
            history: self.history.clone(),
            current: self.current,
            objects: self.objects.clone(),
            counter: self.counter,
        }
    }
}
