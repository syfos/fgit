use crossterm::event::KeyCode;
#[allow(dead_code)]
/// Event manager for `Fgit`.
pub struct EventManager {
  mode: ModeType,
  key_pressed: IoSignal,
  pending_prefix: KeyCode,
}

impl EventManager {
  pub fn new() -> Self {
    Self {
      mode: ModeType::Normal,
      key_pressed: IoSignal::None,
      pending_prefix: KeyCode::Null,
    }
  }
}

impl Default for EventManager {
  fn default() -> Self {
    Self::new()
  }
}

#[allow(dead_code)]
pub enum IoSignal {
  Split,
  Quit,
  None,
}

#[allow(dead_code)]
enum ModeType {
  Normal,
  Visual,
  Insert,
}
