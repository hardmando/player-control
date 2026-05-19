mod backend;

use crate::backend::{Backend, Player};
use color_eyre::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};
use std::io;

struct App {
    players: Vec<Box<dyn Player>>,
    state: ListState,
}

impl App {
    fn new(backend: &dyn Backend) -> Result<Self> {
        let players = backend.players()?;
        let mut state = ListState::default();
        if !players.is_empty() {
            state.select(Some(0));
        }
        Ok(Self { players, state })
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.players.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.players.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    #[cfg(target_os = "linux")]
    let backend_impl = backend::mpris::MprisBackend;
    #[cfg(not(target_os = "linux"))]
    let backend_impl = backend::mock::MockBackend;

    let mut app = App::new(&backend_impl)?;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(f.area());

            let items: Vec<ListItem> = app
                .players
                .iter()
                .map(|p| {
                    let status = if p.is_playing() { "▶" } else { "⏸" };
                    ListItem::new(format!("{} {} - {} ({})", status, p.artist(), p.title(), p.name()))
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Players"))
                .highlight_style(
                    Style::default()
                        .bg(Color::Blue)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, chunks[0], &mut app.state);

            let help = Paragraph::new("↑/↓: Nav | Space: Play/Pause | n: Next | p: Prev | q: Quit")
                .block(Block::default().borders(Borders::ALL).title("Controls"));
            f.render_widget(help, chunks[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    KeyCode::Char(' ') => {
                        if let Some(i) = app.state.selected() {
                            app.players[i].play_pause()?;
                        }
                    }
                    KeyCode::Char('n') => {
                        if let Some(i) = app.state.selected() {
                            app.players[i].next()?;
                        }
                    }
                    KeyCode::Char('p') => {
                        if let Some(i) = app.state.selected() {
                            app.players[i].previous()?;
                        }
                    }
                    _ => {}
                }
            }
        }
        
        // Refresh player list/state
        app.players = backend_impl.players()?;
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
