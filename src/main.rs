use std::io::Result;
mod app;
mod tui;
mod utils;
mod widgets;
use crate::app::app::App;
use crate::tui::Tui;

fn main() -> Result<()> {
    let mut app = App::new();
    let mut tui = Tui::new()?;
    tui.enter()?;

    app.run(&mut tui)?;

    tui.restore()?;
    Ok(())
}
