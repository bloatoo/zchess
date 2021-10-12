use chess::{
    chess::{Board, Square},
    ui::event::*,
};

use std::io::Write;

use crossterm::{
    cursor, queue,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let events = Events::new(1024);
    let mut stdout = std::io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let board = Board::default();

    let sq = Square::new(0, 1);
    let test_moves = board.generate_moves(&sq, &board.piece_at(sq.to_idx()).as_ref().unwrap());

    loop {
        queue!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 7))?;

        for (ref idx, ref piece) in board.pieces().iter().enumerate() {
            if idx % 8 == 0 {
                stdout.execute(cursor::MoveToPreviousLine(1)).unwrap();
            }

            if let Some(_) = test_moves.iter().find(|x| x == &idx) {
                stdout.execute(Print("* ")).unwrap();
                continue;
            }

            stdout
                .execute(match piece {
                    Some(p) => Print(format!("{} ", p.as_ref())),
                    None => Print("e ".into()),
                })
                .unwrap();
        }

        if let Ok(Event::Input(k)) = events.next() {
            match k {
                Key::Char('q') => break,
                _ => (),
            }
        }

        stdout.flush().unwrap();
    }

    exit();
    Ok(())
}

fn exit() {
    let mut stdout = std::io::stdout();
    stdout.execute(LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}
