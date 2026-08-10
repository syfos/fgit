use ratatui::layout::Rect;

pub struct Buffer {
  component: Vec<Rect>,
  uid: BufUid,
}

pub struct BufUid;
impl BufUid {
  fn assign() -> Self {
    Self {
      
    }
  }
}
