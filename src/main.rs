mod game;
mod helpers;

use crate::game::{Game, GameState};
use std::{io, time::{Duration, Instant}};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend, layout::Rect, prelude::Backend, style::{Color, Style}, text::Text, widgets::Paragraph, Terminal
};


fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout: io::Stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend: CrosstermBackend<io::Stdout> = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;
    
    let app = Game::new(10, 10, game::Difficulty::MEDIUM); // change to allow for different sizes, based on window size?
    let res= run_app(&mut terminal, app);
    
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

fn run_app<B: tui::backend::Backend>(terminal: &mut Terminal<B>, mut game: Game) -> io::Result<()> {
    let mut key_processed: bool;
    let mut last_key_time = Instant::now();
    let debounce_duration = Duration::from_millis(125);
    
    loop {
        terminal.draw(|f| ui(f, &mut game))?;

        key_processed = true;
        
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                    let current_time = Instant::now();

                if current_time.duration_since(last_key_time) < debounce_duration { continue; }
                
                if game.game_state == GameState::ACTIVE {
                    match key.code {
                        KeyCode::Char('q') => { return Ok(()); }
                        KeyCode::Esc => { return Ok(()); }
                    
                        KeyCode::Up => { game.move_cursor(key.code); }
                        KeyCode::Down => { game.move_cursor(key.code); }
                        KeyCode::Left => { game.move_cursor(key.code); }
                        KeyCode::Right => { game.move_cursor(key.code); }
                    
                        KeyCode::Char('k') => { game.move_cursor(KeyCode::Up); }
                        KeyCode::Char('j') => { game.move_cursor(KeyCode::Down); }
                        KeyCode::Char('h') => { game.move_cursor(KeyCode::Left); }
                        KeyCode::Char('l') => { game.move_cursor(KeyCode::Right); }
                    
                        KeyCode::Char('w') => { game.move_cursor(KeyCode::Up); }
                        KeyCode::Char('s') => { game.move_cursor(KeyCode::Down); }
                        KeyCode::Char('a') => { game.move_cursor(KeyCode::Left); }
                        KeyCode::Char('d') => { game.move_cursor(KeyCode::Right); }
                    
                        KeyCode::Char(' ') => { game.toggle_flag(); }
                        KeyCode::Char('f') => { game.toggle_flag(); }
                        KeyCode::Enter => { game.reveal_cell(); }
                        KeyCode::Char('e') => { game.reveal_cell(); }
                    
                        _ => { key_processed = false; }
                    }
                
                    if key_processed { last_key_time = current_time; }
                } else {
                    match key.code {
                        KeyCode::Char('q') => { return Ok(()); }
                        KeyCode::Esc => { return Ok(()); }

                        KeyCode::Up => { continue; } // keep the movement keys from making the user quit
                        KeyCode::Down => { continue; } // as to not exit the application without them fully
                        KeyCode::Left => { continue; } // realizing that they had lost
                        KeyCode::Right => { continue; }
                    
                        KeyCode::Char('k') => { continue; }
                        KeyCode::Char('j') => { continue; }
                        KeyCode::Char('h') => { continue; }
                        KeyCode::Char('l') => { continue; }
                    
                        KeyCode::Char('w') => { continue; }
                        KeyCode::Char('s') => { continue; }
                        KeyCode::Char('a') => { continue; }
                        KeyCode::Char('d') => { continue; }

                        _ => { return Ok(()); } // any other key should allow the user to quit
                    }
                }
            }
        }
    }
}

fn ui<B: Backend>(frame: &mut tui::Frame<B>, game: &mut Game) {
    let size = frame.size();
    
    let top_left_text = 
        if game.game_state == GameState::ACTIVE { Paragraph::new(Text::raw(format!("{} Flags Left", game.flags_available))).style(Style::default().fg(Color::White)) }
        else { Paragraph::new(Text::raw("Game Over!".to_string())).style(Style::default().fg(Color::White)) };

    frame.render_widget(top_left_text, Rect::new(2, 1, 20, 1));
    
    let right_text_width = 18;
    let right_text_x =
        if size.width > right_text_width + 2 { size.width - right_text_width - 2 }
        else { 0 };
    let top_right_text: Paragraph<'_> = Paragraph::new(Text::raw("press 'q' to quit"))
        .style(Style::default().fg(Color::White));
    frame.render_widget(top_right_text, Rect::new(right_text_x, 1, right_text_width, 1));
    
    game::render_grid(frame, game);
}