use std::time::{Duration, Instant};

use crate::helpers::{incriment_neighbors, reveal_safe_neighbors};
use crossterm::event::KeyCode;
use rand::{thread_rng, Rng};
use tui::{ 
    backend::Backend, 
    layout::Rect, 
    style::{Color, Style}, 
    text::Text, 
    widgets::{Block, Borders, Paragraph}
};

pub(crate) const CELL_WIDTH: u16 = 5;
pub(crate) const CELL_HEIGHT: u16 = 3;
pub(crate) const END_ANIMATION_DELAY: Duration = Duration::from_millis(125);

#[derive(PartialEq)]
pub enum GameState {
    ACTIVE,
    SUCSESS,
    FAILED,
}

#[derive(Clone)]
#[repr(usize)]
pub enum Difficulty { // dictates the percentage of cells that should be mines
    EASY = 12,
    MEDIUM = 16,
    HARD = 21,
}

pub struct Game {
    grid: Vec<Vec<Cell>>,
    cursor_x: usize,
    cursor_y: usize,
    width: usize,
    height: usize,
    show_cursor: bool,
    difficulty_level: Difficulty,
    pub game_state: GameState,
    first_move_made: bool,
    pub game_end_animation_level: usize,
    pub game_time: Instant,
    pub game_start_time: Instant,
    pub game_end_time: Instant,
    pub flags_available: usize,
    pub hidden_cells_remaining: usize,
}

pub struct Cell {
    pub mines_seen: i8,
    pub cell_state: CellState,
}

#[derive(PartialEq)]
pub enum CellState {
    HIDDEN,
    REVEALED,
    FLAGGED,
    REVEALED_AFTER_END,
}

impl Game {
    pub fn new(width: usize, height: usize, difficulty: Difficulty) -> Self {
        let mut grid= Vec::with_capacity(height);
        
        for _ in 0..height {
            let mut row: Vec<Cell> = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(Cell {
                    mines_seen: 0,
                    cell_state: CellState::HIDDEN,
                });
            }
            grid.push(row);
        }

        let mines= width * height * difficulty.to_owned() as usize / 100; // truncate non integer mine count

        Game {
            grid,
            cursor_x: 0,
            cursor_y: 0,
            width,
            height,
            show_cursor: true,
            difficulty_level: difficulty,
            game_state: GameState::ACTIVE,
            first_move_made: false,
            game_end_animation_level: 0,
            game_time: Instant::now(),
            game_start_time: Instant::now(),
            game_end_time: Instant::now(),
            flags_available: mines,
            hidden_cells_remaining: width * height - mines,
        }
    }
    
    pub fn move_cursor(&mut self, direction: KeyCode) {
        match direction {
            KeyCode::Up => {
                if self.cursor_y > 0 { 
                    self.cursor_y -= 1;
                }
            },
            KeyCode::Down => {
                if self.cursor_y < self.height - 1 {
                    self.cursor_y += 1;
                }
            },
            KeyCode::Left => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                }
            },
            KeyCode::Right => {
                if self.cursor_x < self.width - 1 {
                    self.cursor_x += 1;
                }
            },
            _ => {}
        }
    }

    pub fn place_mines(&mut self, centerx: usize, centery: usize){
        let width = self.grid[0].len();
        let height = self.grid.len();
        let mines= self.grid[0].len() * self.grid.len() * self.difficulty_level.to_owned() as usize / 100; // truncate non integer mine count
        let mut rng = thread_rng();
        
        let mut mines_placed = 0;
        while mines_placed < mines {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);

            let distance = (x as isize - centerx as isize).abs() + (y as isize - centery as isize).abs(); // Minimum range of forced safe cells to ensure an area is cleared

            if (self.grid[y][x].mines_seen >= 0 && distance > 2) { // if the mines seen < 0 then it is a mine itself, should not alter
                self.grid[y][x].mines_seen = -9; // TEMP, negative 9 to see if any mines are being touched by mines seen logic 
                incriment_neighbors(x, y, &mut self.grid); 
                mines_placed += 1;
            }
        }
    }
    
    pub fn reveal_cell(&mut self) {
        if !self.first_move_made {
            self.place_mines(self.cursor_x, self.cursor_y);
            self.first_move_made = true;
        }

        let cell: &mut Cell = &mut self.grid[self.cursor_y][self.cursor_x];
        
        if cell.cell_state == CellState::REVEALED || cell.cell_state == CellState::FLAGGED { return } // do not allow for flagged cells to be revealed

        // handle game lose, otherwise decrease left by 1
        cell.cell_state = CellState::REVEALED;
        if cell.mines_seen >= 0 {
            if (cell.mines_seen == 0) { 
                reveal_safe_neighbors(self.cursor_x, self.cursor_y, &mut self.grid);
                self.update_hidden_cells_remaining();
            }
            self.hidden_cells_remaining -= 1;
            
            if self.hidden_cells_remaining <= 0 { 
                self.game_state = GameState::SUCSESS; 
                self.game_end_time = Instant::now();
            }
        } else { // if mines seen is negative it is itself a mine, and thus the game is lost 
            self.game_state = GameState::FAILED;
            self.game_end_time = Instant::now();
        }
    }

    pub fn toggle_flag(&mut self) {
        let cell: &mut Cell = &mut self.grid[self.cursor_y][self.cursor_x];
        
        if cell.cell_state == CellState::REVEALED { return } // do not allow for flagging revealed squares

        cell.cell_state = 
            if cell.cell_state == CellState::HIDDEN { 
                self.flags_available -= 1;
                CellState::FLAGGED 
            } else {
                self.flags_available += 1;
                CellState::HIDDEN 
            }
    }

    
    pub fn update_hidden_cells_remaining(&mut self) {
        let mut count = 0;
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                if self.grid[i][j].cell_state == CellState::HIDDEN && self.grid[i][j].mines_seen >= 0 { count += 1; }
            }
        }
        self.hidden_cells_remaining = count + 1;
    }
}

pub fn render_grid<B: Backend>(frame: &mut tui::Frame<B>, game: &mut Game){
    let size =  frame.size();
    
    let grid_width = CELL_WIDTH * game.width as u16;
    let grid_height = CELL_HEIGHT * game.height as u16;
    
    let grid_x = 
        if size.width > grid_width { (size.width - grid_width)/2 } 
        else { 0 };
    
    let grid_y = 
        if size.height > grid_height { (size.height - grid_height)/2 } 
        else { 0 };

    for y in 0..game.height {
        for x in 0..game.width {
            let cell = &mut game.grid[y][x];
            let cell_x = grid_x + (x as u16 * CELL_WIDTH);
            let cell_y = grid_y + (y as u16 * CELL_HEIGHT);

            if cell_x + CELL_WIDTH > size.width || cell_y + CELL_HEIGHT > size.height { continue; } // if cell is outside of terminal, do not render

            let mut style = Style::default();
            if game.show_cursor && x == game.cursor_x && y == game.cursor_y { style = style.bg(Color::DarkGray); }

            let mut cell_text = 
                if cell.mines_seen < 0 { " ¤".to_string() }
                else { format!(" {}", cell.mines_seen) };
            
            match game.game_state {
                GameState::ACTIVE => {
                    game.game_end_animation_level = 0;
                }
                GameState::FAILED => {
                    if game.game_state != GameState::ACTIVE && Instant::now().duration_since(game.game_time) > END_ANIMATION_DELAY {
                        game.game_time = Instant::now();
                        game.game_end_animation_level += 1;
                    }
                    
                    let distance = (x as isize - game.cursor_x as isize).abs() + (y as isize - game.cursor_y as isize).abs();
        
                    if (distance as usize) < game.game_end_animation_level { 
                        if cell.cell_state == CellState::REVEALED {
                            style = style.bg(Color::Red);
                        } else {
                            cell.cell_state = CellState::REVEALED_AFTER_END;
                            style = style.bg(Color::Red);
                            style = style.fg(Color::Yellow);
                        }
                    }
                }
                GameState::SUCSESS => {
                    if game.game_state != GameState::ACTIVE && Instant::now().duration_since(game.game_time) > END_ANIMATION_DELAY {
                        game.game_time = Instant::now();
                        game.game_end_animation_level += 1;
                    }
                    
                    let distance = (x as isize - game.cursor_x as isize).abs() + (y as isize - game.cursor_y as isize).abs();
        
                    if (distance as usize) < game.game_end_animation_level { 
                        if cell.cell_state == CellState::REVEALED {
                            style = style.bg(Color::Green);
                        } else {
                            cell.cell_state = CellState::REVEALED_AFTER_END;
                            style = style.fg(Color::LightYellow);
                        }
                    }
                }
            }

            match cell.cell_state {
                CellState::HIDDEN => { 
                    cell_text = String::new(); 
                }
                CellState::FLAGGED => { 
                    style = style.bg(Color::Red); 
                    cell_text = " F".to_string();
                }
                CellState::REVEALED => { 
                    if cell.mines_seen < 0 { style = style.bg(Color::Red); }
                    cell_text = format!("{}", cell_text);
                }
                CellState::REVEALED_AFTER_END => { 
                    if cell.mines_seen < 0 { style = style.bg(Color::Red); }
                    cell_text = format!("{}", cell_text);
                }
            }

            if cell.cell_state == CellState::REVEALED {
                match cell.mines_seen {

                    1 => { style = style.fg(Color::Blue); }
                    2 => { style = style.fg(Color::Rgb(61, 179, 143)); }
                    3 => { style = style.fg(Color::LightMagenta); }
                    4 => { style = style.fg(Color::Yellow); }
                    5 => { style = style.fg(Color::Red); }
                    6 => { style = style.fg(Color::Red); }
                    7 => { style = style.fg(Color::Red); }
                    8 => { style = style.fg(Color::Red); }

                    _ => {  style = style.fg(Color::White); }
                }
            }

            frame.render_widget({
                Paragraph::new(Text::raw(cell_text))
                    .block({
                        Block::default()
                            .borders(Borders::ALL)
                            .style(style) 
                        })
                    .style(style) 
                }, 
                Rect::new(cell_x, cell_y, CELL_WIDTH, CELL_HEIGHT)
            );
        }
    }
}
