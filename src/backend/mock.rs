use crate::backend::{Backend, Player};
use color_eyre::Result;

pub struct MockBackend;

impl Backend for MockBackend {
    fn players(&self) -> Result<Vec<Box<dyn Player>>> {
        Ok(vec![
            Box::new(MockPlayer {
                name: "Spotify".into(),
                title: "Song A".into(),
                artist: "Artist X".into(),
                playing: true,
            }),
            Box::new(MockPlayer {
                name: "Firefox - YouTube".into(),
                title: "Video B".into(),
                artist: "Creator Y".into(),
                playing: false,
            }),
        ])
    }
}

struct MockPlayer {
    name: String,
    title: String,
    artist: String,
    playing: bool,
}

impl Player for MockPlayer {
    fn name(&self) -> &str { &self.name }
    fn bus_name(&self) -> &str { "mock.bus.name" }
    fn title(&self) -> String { self.title.clone() }
    fn artist(&self) -> String { self.artist.clone() }
    fn is_playing(&self) -> bool { self.playing }
    fn play_pause(&mut self) -> Result<()> { self.playing = !self.playing; Ok(()) }
    fn next(&mut self) -> Result<()> { Ok(()) }
    fn previous(&mut self) -> Result<()> { Ok(()) }
}
