use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use std::{thread::sleep, time::{Duration, Instant}};
use crate::{player::{PlayerState}, Player};
use crate::World;
use crate::row::TerrainSymbols;

enum GameState {
    INIT,
    PLAY,
    WON,
    LOST,
    END,
}

#[derive(PartialEq)]
pub enum KeyPress {
    Up,
    Down,
    Left,
    Right,
    Esc,
    None,
    Space,
}

pub struct GameControl {
    height: usize,
    width: usize,
    tick_time_ms: u32,

    player: Player,
    world: World,

    state: GameState,
}

impl GameControl {

    pub fn new(height: usize, width: usize, tick_time_ms: u32) -> GameControl {

        GameControl { 
            height: height,
            width: width,
            tick_time_ms: tick_time_ms,

            player: Player::new('\u{eeed}', 0, height),
            world: World::new(width, height),

            state: GameState::INIT,
        }
    }

    pub fn game_loop(&mut self) {

        let mut message: String = String::from("");
        self.init_game();

        loop {
            match self.state {
                GameState::INIT => {

                    self.player = Player::new('\u{eeed}', 0, self.height);
                    self.world = World::new(self.width, self.height);

                    self.state = GameState::PLAY;
                },
                GameState::PLAY => {

                    if self.player.get_player_state() == PlayerState::Splash {
                        self.state = GameState::LOST;
                    } else if self.player.get_player_state() == PlayerState::Happy {
                        self.state = GameState::WON;
                    }

                    self.world.clear();

                    // Logs can move player
                    self.world.update_world(&mut self.player);

                    self.player.update_state(&self.world);

                    // Place player in overlay at its position in world
                    self.world.set(self.player.get_pos().x, self.player.get_pos().y, self.player.get_symbol());
                    
                    message = format!("Steps: {}", self.player.get_player_steps());

                    self.world.draw(&Some(&message));

                    self.handle_key_during_play();

                },
                GameState::WON => {
                    message = format!("Steps: {}\r\nYou Won! \n\rPress Space to restart, or ESC to end game.\r\n", self.player.get_player_steps());
                    self.state = GameState::END;
                },
                GameState::LOST => {
                    message = format!("Steps: {}\r\nYou Lost! \n\rPress Space to restart, or ESC to end game.\r\n", self.player.get_player_steps());
                    self.state = GameState::END;
                },
                GameState::END => {
                    self.world.clear();
                    self.world.update_world(&mut self.player);
                    self.world.set(self.player.get_pos().x, self.player.get_pos().y, self.player.get_symbol());
                    self.world.draw(&Some(&message));

                    let pressed_key = self.get_keypress();

                    if pressed_key == KeyPress::Esc {
                        break;
                    } else if pressed_key == KeyPress::Space {
                        self.state = GameState::INIT;
                    }

                }
            }
        }
        self.deinit_game();
    }

    fn handle_key_during_play(&mut self) {

        let pressed_key = self.get_keypress();

        if pressed_key == KeyPress::Esc {
            self.state = GameState::LOST;
        } else if pressed_key != KeyPress::None {

            let dx: i32;
            let dy: i32;

            match pressed_key {
                KeyPress::Up => {dx = 0; dy = -1},
                KeyPress::Right => {dx = 1; dy = 0},
                KeyPress::Down => {dx = 0; dy = 1},
                KeyPress::Left => {dx = -1; dy = 0},
                _ => {dx = 0; dy = 0},
            }

            let new_x = (self.player.get_pos().x as i32 + dx).clamp(0, (self.width-1) as i32);
            let new_y = (self.player.get_pos().y as i32 + dy).clamp(0, (self.height) as i32);

            let row = self.world.get_row(new_y as usize);
            let cell = row.get_x(new_x as usize);

            if cell == TerrainSymbols::Hedge.symbol() {
                return;
            }

            self.player.move_to(new_x as usize, new_y as usize, self.width, self.height);
        }
    }

    fn init_game(&self) {
        enable_raw_mode().unwrap();
    }

    fn deinit_game(&self) {
        disable_raw_mode().unwrap();
    }

    fn get_keypress(&self) -> KeyPress {
        
        let start_time = Instant::now();

        let mut key_code: KeyPress = KeyPress::None;

        if event::poll(Duration::from_millis(self.tick_time_ms.into())).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Esc => return KeyPress::Esc,
                    KeyCode::Up | KeyCode::Char('w') => key_code = KeyPress::Up,
                    KeyCode::Right | KeyCode::Char('d') => key_code =  KeyPress::Right,
                    KeyCode::Down | KeyCode::Char('s') => key_code =  KeyPress::Down,
                    KeyCode::Left | KeyCode::Char('a') => key_code =  KeyPress::Left,
                    KeyCode::Char(' ') => key_code =  KeyPress::Space,
                    _ => key_code = KeyPress::None,
                }
            }
        }

        let dur = Instant::now() - start_time;

        if dur < Duration::from_millis(self.tick_time_ms.into()) {
            sleep(Duration::from_millis(self.tick_time_ms.into()) - dur);
        }

        return key_code;
    }

}