use crate::watcher::WatchSignals;
use std::sync::Arc;

#[allow(dead_code)]
pub struct App {
  // ui: Ui,
  pub watcher_signal: Arc<WatchSignals>,
}

#[allow(dead_code)]
impl App {
  pub fn new() -> Result<App, Box<dyn std::error::Error>> {
    Ok(App {
      watcher_signal: WatchSignals::spawn()?,
    })
  }
}
