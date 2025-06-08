use crossterm::{
    cursor::MoveTo, execute, style::{StyledContent}, terminal::{Clear, ClearType}
};
use std::io::{stdout, Write};
use rand::Rng;

use crate::{player::Player, row::{Row, Terrain}};

pub struct World {
    width: usize,
    height: usize,

    map: Vec<Row>,
    tick: u32,
}

impl World {

    pub fn new(width: usize, height: usize) -> Self {
        
        let mut rng = rand::rng();

        let mut map = Vec::new();

        for y in 0..height {

            if y == (height-1) {
                let row = Row::new(y, width, Terrain::GrassNoHedge);
                map.push(row);
                continue;
            } else if y == 0 {
                let row = Row::new(y, width, Terrain::Goal);
                map.push(row);
            }

            let rand_n = rng.random_range(0..100);

            if rand_n > 50 {
                let row = Row::new(y, width, Terrain::Water);
                map.push(row);
            } else {
                let row = Row::new(y, width, Terrain::Grass);
                map.push(row);
            }
        }

        World { width: width, height: height, map: map, tick: 0 }
    }

    pub fn clear(&mut self) {
        let mut stdout = stdout();
        // Clear the terminal and move cursor to top-left
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();
        stdout.flush().unwrap(); // ensure it's printed immediately

        for row in self.map.as_mut_slice() {
            row.clear_overlay();
        }

    }

    pub fn update_world(&mut self, player: &mut Player ) {
        self.tick += 1;

        if self.tick % 7 == 0 {
            for row in &mut self.map {
                row.update_row(player);
            }
        }

        for row in &mut self.map {
            row.fill_overlay();
        }
    }

    pub fn set(&mut self, x: usize, y: usize, c: StyledContent<char>) {
        if y <= self.height && x < self.width {
            self.map[y].set_overlay(x, c);
        }
    }

    pub fn draw(&self, text: &Option<&String>) {

        let mut stdout = stdout();
        let mut i = 0;

        for row in &self.map {
            print!("{i}\t");
            for x in 0..self.width {
                let sym = row.get_x(x);
                print!("{sym}");
            }
            println!("\r");
            i += 1;
        }

        if text.is_some()
        {
            let txt = text.as_ref().unwrap();
            print!("{txt}");
        }

        stdout.flush().unwrap(); // ensure it's printed immediately

    }

    pub fn get_row(&self, y: usize) -> &Row {

        &self.map[y]
    }
}
