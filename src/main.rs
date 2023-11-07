mod constants;
mod entities;
mod managers;
mod utils;

use crate::constants::Coordinate;
use crate::entities::AudioObject;
use crate::managers::AudioObjectManager;

fn main() {
    let mut audio_manager: managers::AudioObjectManager = AudioObjectManager::new();
}
