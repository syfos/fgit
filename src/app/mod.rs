use std::sync::Arc;

use crate::watcher::WatchSignals;

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Pages {
  HomePage,
  HelpPage,
}

#[allow(dead_code)]
pub struct App {
  pub watcher_signal: Arc<WatchSignals>,
  pub active_page: Pages,
  pub text: String,
}

#[allow(dead_code)]
impl App {
  pub fn new() -> Result<App, Box<dyn std::error::Error>> {
    Ok(App {
      watcher_signal: WatchSignals::spawn()?,
      active_page: Pages::HomePage,
      text: "".to_string(),
    })
  }
}
