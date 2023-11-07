mod constants;
mod entities;
mod managers;
mod utils;

use crate::constants::Coordinate;
use crate::managers::AudioObjectManager;

fn main() {
    let mut manager: AudioObjectManager = AudioObjectManager::new();

    let id_1: usize = manager.add(Coordinate::A.position().unwrap()).id;
    let id_2: usize = manager.add(Coordinate::B.position().unwrap()).id;
    manager.move_object(id_1, Coordinate::C.position().unwrap());
    let id_3: usize = manager.add(Coordinate::D.position().unwrap()).id;
    manager.move_object(id_2, Coordinate::E.position().unwrap());
    manager.remove(id_1);
    manager.move_object(id_3, Coordinate::F.position().unwrap());
    manager.move_object(id_3, Coordinate::G.position().unwrap());
    manager.undo();
    manager.undo();

    manager.undo();
    manager.redo();

    println!("{}", manager.to_string());
}
