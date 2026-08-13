use crate::{action::EventManager, ui::Tui, watcher::WatchSignals};
use std::sync::Arc;

#[allow(dead_code)]
pub struct App {
  pub tui: Tui,
  pub event_manager: EventManager,
  pub watcher_signal: Arc<WatchSignals>,
}

#[allow(dead_code)]
impl App {
  pub fn new() -> Result<App, Box<dyn std::error::Error>> {
    Ok(App {
      tui: Tui::new(),
      event_manager: EventManager::default(),
      watcher_signal: WatchSignals::spawn()?,
    })
  }
}
