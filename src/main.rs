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
                    let short_bus = p.bus_name().strip_prefix("org.mpris.MediaPlayer2.").unwrap_or(p.bus_name());
                    ListItem::new(format!("{} {} - {} ({} | {})", status, p.artist(), p.title(), p.name(), short_bus))
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
        let selected_bus_name = app.state.selected().and_then(|i| app.players.get(i)).map(|p| p.bus_name().to_string());
        let mut players = backend_impl.players()?;
        
        // Deduplicate by title. If main browser and tab extension report same video, keep tab extension.
        // Or just keep first seen per title.
        let mut unique_players = Vec::new();
        let mut seen_titles = std::collections::HashSet::new();
        
        // Sort to prioritize tab instances (longer bus names or specific patterns) over main instances
        players.sort_by(|a, b| b.bus_name().len().cmp(&a.bus_name().len()));
        
        for p in players {
            let title = p.title();
            if title.is_empty() || title == "Unknown" {
                unique_players.push(p);
            } else if seen_titles.insert(title) {
                unique_players.push(p);
            }
        }
        
        app.players = unique_players;
        app.players.sort_by(|a, b| a.bus_name().cmp(b.bus_name()));
        
        // Restore selection
        if let Some(bus_name) = selected_bus_name {
            if let Some(new_idx) = app.players.iter().position(|p| p.bus_name() == bus_name) {
                app.state.select(Some(new_idx));
            } else {
                app.state.select(if app.players.is_empty() { None } else { Some(0) });
            }
        } else if !app.players.is_empty() && app.state.selected().is_none() {
            app.state.select(Some(0));
        }
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
