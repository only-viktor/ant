mod game;

use crate::game::*;

fn main() {
    let ant = Ant::new(Position::new(1000, 1000));

    println!("Steps count: {}", ant.research());
}
