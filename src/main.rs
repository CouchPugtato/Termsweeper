use std::{io, time::{Duration, Instant}};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::{prelude::thread_rng, Rng};
use tui::{
    backend::CrosstermBackend, 
    Terminal,
    widgets::{Block, Borders, Paragraph},
    layout::Rect,
    style::{Color, Style},
    text::{Span, Line},

};


struct Cell {
    character: char,
    highlight_state: HighlightState,
}

#[derive(PartialEq)]
enum HighlightState {
    None,
    Space,
    Enter,
}

struct App {
    grid: Vec<Vec<Cell>>,
    cursor_x: usize,
    cursor_y: usize,
    width: usize,
    height: usize,
    show_cursor: bool,
}

impl App {
    fn new(width: usize, height: usize) -> Self {
        let mut rng = thread_rng();
        let mut grid= Vec::with_capacity(height);
        
        for _ in 0..height {
            let mut row: Vec<Cell> = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(Cell {
                    character: (rng.gen_range(0..26) + 65) as u8 as char, // random character
                    highlight_state: HighlightState::None,
                });
            }
            grid.push(row);
        }
        
        App {
            grid,
            cursor_x: 0,
            cursor_y: 0,
            width,
            height,
            show_cursor: true,
        }
    }
    
    fn move_cursor(&mut self, direction: KeyCode) {
        match direction {
            KeyCode::Up => {
                if self.cursor_y > 0 { self.cursor_y -= 1;
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
    
    fn toggle_highlight(&mut self, highlight_type: HighlightState) {
        let cell: &mut Cell = &mut self.grid[self.cursor_y][self.cursor_x];
        
        cell.highlight_state = 
            if cell.highlight_state == highlight_type { HighlightState::None } 
            else { highlight_type }
    }
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout: io::Stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend: CrosstermBackend<io::Stdout> = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;
    
    let app: App = App::new(10, 10); // change to allow for different sizes, based on window size?
    let res: Result<(), io::Error> = run_app(&mut terminal, app);
    
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    
    if let Err(err) = res {
        println!("Error: {:?}", err);
    }
    
    Ok(())
}

fn run_app<B: tui::backend::Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let mut key_processed: bool;
    let mut last_key_time = Instant::now();
    let debounce_duration = Duration::from_millis(125);
    
    loop {
        terminal.draw(|f| ui(f, &app))?;
        
        key_processed = true;
        
        if let Event::Key(key) = event::read()? {
            let current_time = Instant::now();

            if current_time.duration_since(last_key_time) < debounce_duration { continue; }
            
            match key.code {
                KeyCode::Char('q') => { return Ok(()); }
                KeyCode::Up => { app.move_cursor(key.code); }
                KeyCode::Down => { app.move_cursor(key.code); }
                KeyCode::Left => { app.move_cursor(key.code); }
                KeyCode::Right => { app.move_cursor(key.code); }
                KeyCode::Enter => { app.toggle_highlight(HighlightState::Enter); }
                KeyCode::Char(' ') => { app.toggle_highlight(HighlightState::Space); }
                _ => { key_processed = false }
            }

            if key_processed { last_key_time = current_time; }
        }
    }
}

fn ui<B: tui::backend::Backend>(frame: &mut tui::Frame<B>, app: &App) {
    let size = frame.size();
    
    let top_left_text = Paragraph::new(Line::from(vec![Span::raw("X flags to place")])) // change to real # of bombs - flags placed
        .style(Style::default().fg(Color::White));
    frame.render_widget(top_left_text, Rect::new(2, 1, 20, 1));
    
    let right_text_width = 18;
    let right_text_x =
        if size.width > right_text_width + 2 { size.width - right_text_width - 2 }
        else { 0 };
    let top_right_text: Paragraph<'_> = Paragraph::new(Line::from(vec![Span::raw("Press 'q' to quit")]))
        .style(Style::default().fg(Color::White));
    frame.render_widget(top_right_text, Rect::new(right_text_x, 1, right_text_width, 1));
    
    let cell_width = 5;
    let cell_height = 3;
    
    let grid_width = cell_width * app.width as u16;
    let grid_height = cell_height * app.height as u16;
    
    let grid_x = 
        if size.width > grid_width { (size.width - grid_width)/2 } 
        else { 0 };
    
    let grid_y = 
        if size.height > grid_height { (size.height - grid_height)/2 } 
        else { 0 };
    
    for y in 0..app.height {
        for x in 0..app.width {
            let cell = &app.grid[y][x];
            let cell_x = grid_x + (x as u16 * cell_width);
            let cell_y = grid_y + (y as u16 * cell_height);
            
            if cell_x + cell_width > size.width || cell_y + cell_height > size.height { continue; } // if cell is outside of terminal, do not render
            
            let cell_area = Rect::new(cell_x, cell_y, cell_width, cell_height);
            
            let mut style = Style::default();
            if app.show_cursor && x == app.cursor_x && y == app.cursor_y { style = style.bg(Color::DarkGray); }
            
            match cell.highlight_state {
                HighlightState::None => {},
                HighlightState::Space => { style = style.bg(Color::Blue); },
                HighlightState::Enter => { style = style.bg(Color::Green); },
            }
            
            let block = Block::default()
                .borders(Borders::ALL)
                .style(style);
            
            let text = Line::from(vec![Span::raw(cell.character.to_string())]);
            
            let paragraph = Paragraph::new(text)
                .block(block)
                .style(style);
            
            frame.render_widget(paragraph, cell_area);
        }
    }
}