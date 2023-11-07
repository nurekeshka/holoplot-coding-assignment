use crate::entities::AudioObject;
use crate::utils::Position;
use std::collections::{HashMap, VecDeque};

pub struct AudioObjectManager {
    pub objects: HashMap<usize, AudioObject>,
    history: VecDeque<AudioObjectManager>,
    current: usize,
    counter: usize,
}

impl AudioObjectManager {
    pub fn new() -> AudioObjectManager {
        AudioObjectManager {
            history: VecDeque::new(),
            objects: HashMap::new(),
            counter: 0,
            current: 0,
        }
    }
}

impl AudioObjectManager {
    pub fn add(&mut self, position: Position) -> AudioObject {
        self.save();

        let audio_object = AudioObject::new(self.next_id(), position);
        self.objects.insert(audio_object.id, audio_object.clone());

        return audio_object;
    }

    pub fn remove(&mut self, id: usize) -> Option<AudioObject> {
        self.save();

        return self.objects.remove(&id);
    }
}

impl AudioObjectManager {
    pub fn undo(&mut self) {
        if self.current > 0 {
            self.current -= 1;
            self.restore_current();
        }
    }

    pub fn redo(&mut self) {
        if self.current < self.history.len() - 1 {
            self.current += 1;
            self.restore_current();
        }
    }

    fn save(&mut self) {
        self.history.truncate(self.current);
        self.history.push_back(self.clone());
        self.current += 1;
    }

    fn restore_current(&mut self) {
        if let Some(state) = self.history.get(self.current) {
            let mut clone = state.clone();
            clone.history = self.history.clone();
            *self = clone;
        }
    }
}

impl AudioObjectManager {
    pub fn get(&mut self, id: usize) -> Option<AudioObject> {
        return self.objects.get(&id).map(|obj| obj.clone());
    }

    pub fn contains(&mut self, id: usize) -> bool {
        return self.objects.contains_key(&id);
    }

    fn next_id(&mut self) -> usize {
        self.counter += 1;
        return self.counter;
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
