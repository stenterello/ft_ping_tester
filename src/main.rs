mod app;
mod traits;
mod tui;
mod utils;
mod widgets;

use crate::app::app::App;
use crate::tui::Tui;
use std::io::Result;

fn main() -> Result<()> {
    let mut app = App::new()?;
    let mut tui = Tui::new()?;
    tui.enter()?;

    match app.run(&mut tui) {
        Ok(()) => tui.restore()?,
        Err(e) => {
            tui.restore()?;
            eprintln!("{}", e);
            return Err(e);
        }
    }

    Ok(())
}
