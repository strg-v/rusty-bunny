use crossterm::style::StyledContent;
use rand::Rng;

use crate::player::Player;
use crate::row::TerrainSymbols;

pub struct Log {
    row_width: usize,
    xli: i32,
    xre: i32,
    direction: i32,
    y: usize,
}

impl Log {

    pub fn new(width: usize, xli: i32, length: i32, direction: i32, y: usize) -> Log {

        Log { 
            row_width: width,
            xli: xli, 
            xre: xli + length,
            direction: direction,
            y: y,
        }
    }

    pub fn update_log(&mut self, player: &mut Player)
    {

        let new_xli = self.xli + self.direction;
        let new_xre = self.xre + self.direction;

        let player_y: i32 = player.get_pos().y as i32;
        let player_x: i32 = player.get_pos().x as i32;

        if self.y as i32 == player_y-1 {

            if (player_x >= self.xli) && (player_x <= self.xre) {

                let new_player_x: i32 = player_x + self.direction;

                let new_player_x_usize: usize;

                if new_player_x < 0 {
                    new_player_x_usize = 0;
                } else if new_player_x > self.row_width as i32 {
                    new_player_x_usize = self.row_width;
                } else {
                    new_player_x_usize = new_player_x as usize;
                }

                player.move_to(new_player_x_usize, player_y as usize, self.row_width, self.y+1);
               
            }
        }

        self.xli = new_xli;
        self.xre = new_xre;

    }

    fn get_log_on_map(&self) -> bool {
        if self.direction > 0 {
            return self.xli < self.row_width as i32;
        } else if self.direction < 0 {
            return self.xre > 0;
        }
        else {
            return false;
        }
    }

    fn spawn_new_log(&self) -> Option<Log> {

        let mut rng = rand::rng();
        let distance = rng.random_range(3..5);
        let length = rng.random_range(2..4);

        let new_xli;
        
        if self.direction > 0 {
            if self.xli < distance {
                return None;
            }
    
            new_xli = self.xli - distance - length;

        } else if self.direction < 0 {
            if (self.row_width as i32 - self.xre ) < distance {
                return None;
            }

            new_xli = self.xre + distance;
        } else {
            return None;
        }


        return Option::Some(
            Log::new(self.row_width, new_xli, length, self.direction, self.y)
        );



    }
}

pub struct LogHandler {
    width:usize,
    logs: Vec<Log>,
    direction: i32,
    y: usize,
}

impl LogHandler {
    pub fn new(width: usize, direction: i32, y: usize) -> LogHandler {

        let mut rng = rand::rng();
        let length = rng.random_range(2..4);

        let log = Log::new(width, 0, length, direction, y);
        let mut vector = vec![log];

        let mut idx = 0;

        loop {
            let new_log = vector[idx].spawn_new_log();

            if new_log.is_some() {
                vector.push(new_log.unwrap());
                idx += 1;
            } else {
                break;
            }
        }

        LogHandler { 
            width: width,
            logs: vector,
            direction: direction,
            y: y,
        }
    }

    pub fn update_logs(&mut self, player: &mut Player) {

        // Add new log if enough space on map
        let new_log = self.logs.last().unwrap().spawn_new_log();
        if new_log.is_some() {
            self.logs.push(new_log.unwrap());
        }

        // remove first logs if not on map
        if !self.logs[0].get_log_on_map() {
            self.logs.remove(0);
        }

        for log in  &mut self.logs {
            log.update_log(player);
        }


    }

    pub fn get_logs_in_overlay(&self, overlay: &mut Vec<StyledContent<char>>)
    {
        // move logs in map and put in overlay
        for log in  &self.logs {

            for x in log.xli..log.xre {
                if x >=0 && x < self.width as i32
                {
                    overlay[x as usize] = TerrainSymbols::Board.symbol();
                }
            }
        }
    }
}
