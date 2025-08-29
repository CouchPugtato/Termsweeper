use crate::game::Cell; 

pub fn incriment_neighbors(x: usize, y: usize, grid: &mut Vec<Vec<Cell>>) {
    let max_column = grid.len() as i32;
    let max_row = grid[0].len() as i32;
    
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 { continue; } // do not incriment self
            
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            
            if nx >= 0 && nx < max_row && ny >= 0 && ny < max_column { // is neighbor real? (valid grid indexes)
                let nx_usize = nx as usize;
                let ny_usize = ny as usize;
                if grid[ny_usize][nx_usize].mines_seen < 0 { continue; }
                grid[ny_usize][nx_usize].mines_seen += 1;

            }
        }
    }
}