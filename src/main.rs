// src/main.rs

mod app;
mod menu;
mod input;
mod kalkulasi;
mod ui;

use app::App;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inisialisasi terminal
    let mut stdout = io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    // State aplikasi
    let mut app = App::default();

    // Main event loop
    loop {
        terminal.draw(|f| {
            ui::draw_main_ui(f, &mut app);
        })?;

        // Event handling
        if event::poll(std::time::Duration::from_millis(120))? {
            if let Event::Key(key) = event::read()? {
                // Handle exit
                if key.code == KeyCode::Char('q') {
                    break;
                }
                app.handle_key(key);
                if app.should_quit {
                    break;
                }
            }
        }
    }

    // Restore terminal
    crossterm::terminal::disable_raw_mode()?;
    terminal.show_cursor()?;

    Ok(())
}
