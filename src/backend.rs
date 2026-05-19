use color_eyre::Result;

pub trait Player {
    fn name(&self) -> &str;
    fn title(&self) -> String;
    fn artist(&self) -> String;
    fn is_playing(&self) -> bool;
    fn play_pause(&mut self) -> Result<()>;
    fn next(&mut self) -> Result<()>;
    fn previous(&mut self) -> Result<()>;
}

pub trait Backend {
    fn players(&self) -> Result<Vec<Box<dyn Player>>>;
}

#[cfg(target_os = "linux")]
pub mod mpris;
#[cfg(not(target_os = "linux"))]
pub mod mock;
