use crate::game::{Cell, CellState}; 

pub fn incriment_neighbors(x: usize, y: usize, grid: &mut Vec<Vec<Cell>>) {
    let max_column = grid.len() as i32;
    let max_row = grid[0].len() as i32;
    
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 { continue; } // do not incriment self
            
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            
            if nx >= 0 && nx < max_row && ny >= 0 && ny < max_column { // is neighbor real? (valid grid indexes)
                let neighbor = &mut grid[ny as usize][nx as usize];
                if neighbor.mines_seen < 0 { continue; }
                neighbor.mines_seen += 1;
            }
        }
    }
}

pub fn reveal_safe_neighbors(x: usize, y: usize, grid: &mut Vec<Vec<Cell>>) {
    let max_column = grid.len() as i32;
    let max_row = grid[0].len() as i32;
    
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 { continue; } // do not interact with self
            
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            
            if nx >= 0 && nx < max_row && ny >= 0 && ny < max_column { // is neighbor real? (valid grid indexes)
                let neighbor = &mut grid[ny as usize][nx as usize];
                
                if neighbor.cell_state == CellState::HIDDEN {
                    neighbor.cell_state = CellState::REVEALED;                    
                    if neighbor.mines_seen == 0 {
                        reveal_safe_neighbors(nx as usize, ny as usize, grid); 
                    }
                }
            }
        }
    }
}