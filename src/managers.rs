use crate::entities::AudioObject;
use std::collections::{HashMap, VecDeque};

pub struct AudioObjectManager {
    pub objects: HashMap<usize, AudioObject>,
    history: VecDeque<AudioObjectManager>,
    current: usize,
    counter: usize,
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
