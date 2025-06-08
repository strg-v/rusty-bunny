
use crossterm::style::{StyledContent, Stylize, Color};

use crate::{row::{TerrainSymbols}, world::{World}};
use crate::game::{KeyPress};

#[derive(Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Copy, Clone, PartialEq)]
pub enum PlayerState {
    Alive,
    Splash,
    Happy,
}

pub struct Player {
    pos: Position,
    symbol: StyledContent<char>,
    state: PlayerState,
    steps: u32,
}

impl Player {

    pub fn new(symbol: char, x: usize, y: usize) -> Player {
        let pos = Position{
            x: x,
            y: y,
        };

        Player { 
            pos: pos,
            symbol: symbol.white(),
            state: PlayerState::Alive,
            steps: 0,
        }
    }

    pub fn move_to(&mut self, new_x: usize, new_y: usize, max_x: usize, max_y: usize) {

//        self.steps += isize::abs(dx) as u32 + isize::abs(dy) as u32;
        self.steps += 1;

        let x = new_x.clamp(0, max_x-1);
        let y = new_y.clamp(0, max_y);
        self.pos = Position { x: x, y: y};

    }

    pub fn update_state(&mut self, world: &World) {

        let row = world.get_row(self.pos.y);
        let cell = row.get_x(self.pos.x);

        if cell == TerrainSymbols::Water.symbol() {
            self.set_symbol('\u{2205}'.with(Color::Rgb { r: 0, g: 180, b: 255 }));
            self.state = PlayerState::Splash;
        } else if cell == TerrainSymbols::Goal.symbol() {
            self.set_symbol(self.symbol.green());
            self.state = PlayerState::Happy;
        }

    }

    pub fn get_pos(&mut self) -> Position {
        return self.pos;
    }   

    pub fn get_symbol(&self) -> StyledContent<char> {
        self.symbol
    }

    fn set_symbol(&mut self, new_symbol: StyledContent<char>) {
        self.symbol = new_symbol;
    }

    pub fn get_player_state(&self) -> PlayerState {
        self.state
    }

    pub fn get_player_steps(&self) -> u32 {
        return self.steps;
    }
}