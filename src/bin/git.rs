use fgit::ui::Tui;

#[allow(dead_code)]
#[allow(unused)]
fn main() -> anyhow::Result<(), anyhow::Error> {
  let ui = Tui::run();
  Ok(())
}
