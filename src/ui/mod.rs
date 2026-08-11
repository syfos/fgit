use ratatui::{
  DefaultTerminal,
  layout::{Constraint, Rect},
  widgets::{Block, Borders, Paragraph},
};
use std::{error::Error, result::Result};

#[allow(dead_code)]
pub struct Buffer {
  component: Vec<Rect>,
}

#[allow(dead_code)]
pub struct Ui {
  pub buffer: Buffer,
}

#[allow(dead_code)]
impl Ui {
  pub fn run() -> Result<(), Box<dyn Error>> {
    ratatui::run(Self::draw)?;
    Ok(())
  }

  fn split_horizontal(area: Rect) -> Vec<Rect> {
    let constraints = vec![Constraint::Ratio(1, 1)];
    ratatui::layout::Layout::default()
      .direction(ratatui::layout::Direction::Vertical)
      .constraints(constraints)
      .split(area)
      .to_vec()
  }

  fn draw(terminal: &mut DefaultTerminal) -> std::result::Result<(), Box<dyn Error>> {
    loop {
      terminal.draw(|frame| {
        let chunks = Self::split_horizontal(frame.area());
        for (i, chunk) in chunks.iter().enumerate() {
          let borders = if i == 0 {
            Borders::TOP | Borders::BOTTOM
          } else {
            Borders::BOTTOM
          };

          let block = Block::default().borders(borders);

          let paragraph = Paragraph::new(format!("Panel: {i}")).block(block);

          frame.render_widget(paragraph, *chunk);
        }
      })?;
    }
  }
}
