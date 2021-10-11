use chess::{chess::Board, ui::event::*};

use crossterm::{
    terminal::{
        disable_raw_mode, enable_raw_mode, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
    },
    ExecutableCommand,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let events = Events::new(1024);
    let mut stdout = std::io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let board = Board::empty();

    loop {
        if let Ok(Event::Input(k)) = events.next() {
            match k {
                Key::Char('q') => break,
                _ => (),
            }
        }
    }

    exit();
    Ok(())
}

fn exit() {
    let mut stdout = std::io::stdout();
    stdout.execute(LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}
