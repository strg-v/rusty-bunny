use crossterm::style::{Color, StyledContent, Stylize};

use std::{vec};
use rand::Rng;
use crate::{log::LogHandler, player::{Player}};


pub enum TerrainSymbols {
    Grass,
    Water,
    Goal,
    Hedge,
    Board,
}

#[derive(PartialEq, Clone)]
pub enum Terrain {
    GrassNoHedge,
    Grass,
    Water,
    Goal,
}

impl TerrainSymbols {
    pub fn symbol(&self) -> StyledContent<char> {
        match self {
            TerrainSymbols::Grass => ','.green().italic(),
            TerrainSymbols::Water => '\u{224b}'.blue(),
            TerrainSymbols::Goal => '$'.yellow(),
            TerrainSymbols::Hedge => '#'.with(Color::Rgb { r: 38, g: 128, b: 37 }),
            TerrainSymbols::Board => '='.with(Color::Rgb { r: 100, g: 60, b: 30 }),
        }
    }
}

pub struct Row {
    y: usize,
    width: usize,
    cells: Vec<StyledContent<char>>,
    overlay: Vec<StyledContent<char>>,
    terrain: Terrain,

    log_handler: Option<LogHandler>,

}

impl Row {
    pub fn new(y: usize, width: usize, terrain: Terrain) -> Row {

        if terrain == Terrain::Grass {
            return Row::new_grass(y, width);
        } else if terrain == Terrain::Water {
            return Row::new_water(y, width);
        }

        let mut symbol = ' '.stylize();

        match terrain {
            Terrain::Goal => symbol = TerrainSymbols::Goal.symbol(),
            Terrain::GrassNoHedge => symbol = TerrainSymbols::Grass.symbol(),
            _ => (),
        }
      
        let vec = vec![symbol; width];
        let c: char = (0).into();
        let overlay: Vec<StyledContent<char>> = vec![c.stylize(); width];

        Row {
            y: y,
            width: width,
            cells: vec,
            terrain: terrain,
            overlay: overlay,
            log_handler: None,
        }
    }

    fn new_grass(y: usize, width: usize) -> Row {

        let mut cells;
        
        if y == 0{
            cells = vec![TerrainSymbols::Grass.symbol(); width];
        } else {

            cells = Vec::new();
            let mut rng = rand::rng();

            for _ in 0..width {
                let rnd_n = rng.random_range(0..100);

                if rnd_n < 25 {
                    cells.push(TerrainSymbols::Hedge.symbol());
                } else {
                    cells.push(TerrainSymbols::Grass.symbol());
                }
            }

        }        

        let c: char = (0).into();
        let overlay: Vec<StyledContent<char>> = vec![c.stylize(); width];

        Row {
            terrain: Terrain::Grass,
            y: y,
            width: width,
            cells: cells,
            overlay: overlay,
            log_handler: None,
        }

    }

    fn new_water(y: usize, width: usize) -> Row {

        let mut rng = rand::rng();
        let direction;
        
        if rng.random_bool(0.5) {
            direction = 1;    
        } else {
            direction = -1;
        }

        let vec = vec![TerrainSymbols::Water.symbol(); width];
        let c: char = (0).into();
        let overlay: Vec<StyledContent<char>> = vec![c.stylize(); width];

        println!("New Water");

        let log_handler = LogHandler::new(width, direction, y);

        Row {
            cells: vec,
            y: y,
            width: width,
            overlay: overlay,
            terrain: Terrain::Water,
            log_handler: Option::Some( log_handler ), 
        }

    }

    pub fn get_x(&self, x: usize) -> StyledContent<char> {

        if x >= self.width {
            panic!("x out of bounds!")
        }

        let c;

        let c0: char = (0).into();

        if self.overlay[x] != c0.stylize() {
            c = self.overlay[x];
            return c;
        }

        c = self.cells[x];

        return c;
    }

    pub fn fill_overlay(&mut self) {
        if self.log_handler.is_some() {
            self.log_handler.as_mut().unwrap().get_logs_in_overlay(&mut self.overlay);
        }
    }

    pub fn clear_overlay(&mut self) {

        let c: char = (0).into();
        let overlay: Vec<StyledContent<char>> = vec![c.stylize(); self.width];

        self.overlay = overlay;
    }

    pub fn update_row(&mut self, player: &mut Player) {
        if let Some(log_hndlr) = self.log_handler.as_mut() {
            log_hndlr.update_logs(player);
        }
    }

    pub fn set_overlay(&mut self, x: usize, symbol: StyledContent<char>) {
        
        if x >= self.width {
            panic!("x out of bounds!")
        }

        self.overlay[x] = symbol;
    }
}