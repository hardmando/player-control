use crate::backend::{Backend, Player};
use color_eyre::{Result, eyre::eyre};
use mpris::PlayerFinder;

pub struct MprisBackend;

impl Backend for MprisBackend {
    fn players(&self) -> Result<Vec<Box<dyn Player>>> {
        let finder = PlayerFinder::new().map_err(|e| eyre!(e))?;
        let players = finder.find_all().map_err(|e| eyre!(e))?;
        Ok(players
            .into_iter()
            .map(|p| Box::new(MprisPlayer(p)) as Box<dyn Player>)
            .collect())
    }
}

struct MprisPlayer(mpris::Player);

impl Player for MprisPlayer {
    fn name(&self) -> &str { self.0.identity() }
    fn bus_name(&self) -> &str { self.0.bus_name() }
    fn title(&self) -> String {
        self.0.get_metadata()
            .ok()
            .and_then(|m| m.title().map(String::from))
            .unwrap_or_else(|| "Unknown".into())
    }
    fn artist(&self) -> String {
        self.0.get_metadata()
            .ok()
            .and_then(|m| m.artists().and_then(|a| a.first().map(|s| String::from(*s))))
            .unwrap_or_else(|| "Unknown".into())
    }
    fn is_playing(&self) -> bool {
        self.0.get_playback_status().ok() == Some(mpris::PlaybackStatus::Playing)
    }
    fn play_pause(&mut self) -> Result<()> { self.0.play_pause().map_err(|e| eyre!(e)) }
    fn next(&mut self) -> Result<()> { self.0.next().map_err(|e| eyre!(e)) }
    fn previous(&mut self) -> Result<()> { self.0.previous().map_err(|e| eyre!(e)) }
}
