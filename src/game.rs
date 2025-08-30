use crate::helpers::{incriment_neighbors, reveal_safe_neighbors};
use crossterm::event::KeyCode;
use rand::{thread_rng, Rng};
use tui::{ 
    backend::Backend, layout::Rect, style::{Color, Style}, text::Text, widgets::{Block, 
    Borders, 
    Paragraph}};

const CELL_WIDTH: u16 = 5;
const CELL_HEIGHT: u16 = 3;


pub enum GameState {
    ACTIVE,
    SUCSESS,
    FAILED,
}

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
    game_state: GameState,
    pub flags_available: i32,
    hidden_cells_remaining: i32,
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
}

impl Game {
    pub fn new(width: usize, height: usize, difficulty: Difficulty) -> Self {
        let mut rng = thread_rng();
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

        // place mines
        let mines= width * height * difficulty as usize / 100; // truncate non integer mine count
        let mut mines_placed = 0;
        while mines_placed < mines {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);

            if (grid[y][x].mines_seen >= 0) { // if the mines seen < 0 then it is a mine itself, should not alter
                grid[y][x].mines_seen = -9; // TEMP, negative 9 to see if any mines are being touched by mines seen logic 
                incriment_neighbors(x, y, &mut grid); 
                mines_placed += 1;
            }
        }

        Game {
            grid,
            cursor_x: 0,
            cursor_y: 0,
            width,
            height,
            show_cursor: true,
            game_state: GameState::ACTIVE,
            flags_available: mines as i32,
            hidden_cells_remaining: (width * height - mines) as i32,
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
    
    pub fn reveal_cell(&mut self) {
        let cell: &mut Cell = &mut self.grid[self.cursor_y][self.cursor_x];
        
        if cell.cell_state == CellState::REVEALED || cell.cell_state == CellState::FLAGGED { return }; // do not allow for flagged cells to be revealed

        // handle game lose, otherwise decrease left by 1
        cell.cell_state = CellState::REVEALED;
        if cell.mines_seen >= 0 {
            if (cell.mines_seen == 0) { reveal_safe_neighbors(self.cursor_x, self.cursor_y, &mut self.grid); }
            self.hidden_cells_remaining -= 1;
        } else { // if mines seen is negative it is itself a mine, and thus the game is lost 
            self.game_state = GameState::FAILED;
        }
    }

    pub fn toggle_flag(&mut self) {
        let cell: &mut Cell = &mut self.grid[self.cursor_y][self.cursor_x];
        
        if cell.cell_state == CellState::REVEALED { return }; // do not allow for flagging revealed squares
        cell.cell_state = 
            if cell.cell_state == CellState::HIDDEN { 
                self.flags_available -= 1;
                CellState::FLAGGED 
            } else {
                self.flags_available += 1;
                CellState::HIDDEN 
            }
    }
}

pub fn render_grid<B: Backend>(frame: &mut tui::Frame<B>, game: &Game){
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
            let cell = &game.grid[y][x];
            let cell_x = grid_x + (x as u16 * CELL_WIDTH);
            let cell_y = grid_y + (y as u16 * CELL_HEIGHT);
            
            if cell_x + CELL_WIDTH > size.width || cell_y + CELL_HEIGHT > size.height { continue; } // if cell is outside of terminal, do not render
            
            let mut style = Style::default();
            if game.show_cursor && x == game.cursor_x && y == game.cursor_y { style = style.bg(Color::DarkGray); }
            
            let mut cell_text = 
                if cell.mines_seen < 0 { " ¤".to_string() }
                else { format!(" {}", cell.mines_seen) };
            
            match cell.cell_state {
                CellState::HIDDEN => { 
                    cell_text = String::new(); 
                },
                CellState::FLAGGED => { 
                    style = style.bg(Color::Red); 
                    cell_text = " F".to_string();
                },
                CellState::REVEALED => { 
                    if cell.mines_seen < 0 { style = style.bg(Color::Red); }
                    cell_text = format!("{}", cell_text);
                },
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