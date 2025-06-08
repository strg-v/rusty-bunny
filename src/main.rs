mod game;
mod player;
mod row;
mod world;
mod log;

use game::GameControl;
use player::{Player};
use world::World;

fn main() {

    let width = 16;
    let height = 20;
    let tick_time_ms = 100;

    println!("Welcome");

    let mut game = GameControl::new(height, width, tick_time_ms);
    

    game.game_loop();


    println!("Goodbye!");

}
