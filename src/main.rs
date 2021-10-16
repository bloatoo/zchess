use chess::{
    chess::{Board, Square},
    ui::event::*,
};

const TILE_WIDTH: usize = 8;
const TILE_HEIGHT: usize = 4;
const H_LINE: &str = "─";

use std::{io::Write, panic::PanicInfo};

use crossterm::{
    cursor, execute, queue,
    style::{Color, Print, Stylize},
    terminal::{
        self, disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};

fn panic_hook(info: &PanicInfo<'_>) {
    let location = info.location().unwrap();

    let message = match info.payload().downcast_ref::<&'static str>() {
        Some(msg) => *msg,
        None => match info.payload().downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Box<Any>",
        },
    };

    disable_raw_mode().unwrap();

    execute!(
        std::io::stdout(),
        LeaveAlternateScreen,
        Print(format!(
            "thread <unnamed> panicked at '{}', {}\n",
            message, location
        )),
    )
    .unwrap();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::panic::set_hook(Box::new(|info| panic_hook(info)));

    let events = Events::new(1024);

    let mut stdout = std::io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut cursor_pos = (1, 1);

    let board = Board::default();

    let idx = (cursor_pos.1 * 8 + cursor_pos.0) as usize;

    let tile_str = format!("│{}", " ".repeat(TILE_WIDTH));

    loop {
        let size = terminal::size()?;
        let center = size.0 / 2 - TILE_WIDTH as u16 * 4 - 2;

        // print top vertical line
        queue!(
            stdout,
            Clear(ClearType::All),
            cursor::MoveTo(center - 1, 0),
            Print(&format!("{}", H_LINE.repeat(TILE_WIDTH * 8 + 8 + 1))),
        )?;

        // print
        for _ in 0..8 {
            // print tile's vertical lines
            for _ in 0..TILE_HEIGHT {
                queue!(
                    stdout,
                    cursor::MoveToNextLine(1),
                    cursor::MoveToColumn(center),
                    Print(&format!("{}", tile_str.repeat(9))),
                )?;
            }

            // print horizontal line
            queue!(
                stdout,
                cursor::MoveToColumn(center),
                Print(&format!(
                    "{}",
                    H_LINE.repeat(TILE_WIDTH as usize * 8 + 8 + 1)
                )),
            )?;
        }

        // render pieces
        for i in (0..8).rev() {
            queue!(
                stdout,
                cursor::MoveTo(center, (TILE_HEIGHT as u16 * (7 - i)) + 1)
            )?;

            for j in 0..8 {
                let piece = board.piece_at((i * 8 + j).into());

                let mut piece_string = match piece {
                    Some(ref p) => p.render(TILE_WIDTH).to_string(),
                    None => "".into(),
                };

                if cursor_pos.0 == j && cursor_pos.1 == i {
                    piece_string = match piece_string.is_empty() {
                        false => format!("{}", piece_string.bold()),
                        true => format!("{}", "*".bold()),
                    };
                }

                if let Some(ref p) = board.piece_at(idx) {
                    if board.generate_moves(idx, p).contains(&(i * 8 + j).into()) {
                        piece_string = format!("{}", "*".with(Color::DarkGrey));
                    }
                }

                queue!(
                    stdout,
                    cursor::MoveToColumn(center + 1 + (TILE_WIDTH as u16 + 1) * j as u16),
                    Print(piece_string),
                )?;
            }
        }

        // top to bottom
        /*for row in (0..8).rev() {
            queue!(stdout, cursor::MoveTo(center, TILE_HEIGHT as u16 * row + 1),)?;

            for file in 0..8 {
                queue!(
                    stdout,
                    Print(format!("{}, {}", row, file)),
                    cursor::MoveRight(1)
                )?;
            }
        }*/

        stdout.flush()?;

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
