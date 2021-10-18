use crate::{
    app::App,
    chess::{Side, Square},
    message::Message,
    ui::event::*,
};

use std::io::{Stdout, Write};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use tokio::sync::Mutex;

use crossterm::{
    cursor, execute, queue,
    style::{Color, Print, Stylize},
    terminal::{
        self, disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

const TILE_WIDTH: usize = 8;
const TILE_HEIGHT: usize = 4;
const H_LINE: &str = "─";

pub mod event;

pub enum UIState {
    Menu,
    Game,
}

pub fn draw_board(
    app: &App,
    cursor_pos: (u16, u16),
    selected_piece: Option<(usize, usize)>,
    stdout: &mut Stdout,
) -> Result<(), Box<dyn std::error::Error>> {
    let tile_str = format!("│{}", " ".repeat(TILE_WIDTH));
    let size = terminal::size()?;
    let center = size.0 / 2 - TILE_WIDTH as u16 * 4 - 2;

    let game = app.game().as_ref().unwrap();

    let board = game.board();

    let w_player = game.data().white();
    let b_player = game.data().black();

    let names = format!(
        "white: {} ({}) | black: {} ({})",
        w_player.name(),
        w_player.id(),
        b_player.name(),
        b_player.id()
    );

    let clock = game.data().clock();

    let statusline = format!(
        "id: {} | {} | {}+{}",
        game.id(),
        names,
        clock.initial(),
        clock.increment()
    );

    // print top vertical line
    let (_, y) = terminal::size().unwrap();

    queue!(
        stdout,
        Clear(ClearType::All),
        cursor::MoveTo(0, y),
        Print(statusline),
        cursor::MoveTo(center - 1, 0),
        Print(&format!("{}", H_LINE.repeat(TILE_WIDTH * 8 + 8 + 1))),
    )?;

    // print rows
    for i in 1..=8 {
        execute!(
            stdout,
            cursor::MoveTo(center - 3, TILE_HEIGHT as u16 * i - TILE_HEIGHT as u16 / 2),
            Print(9 - i),
            cursor::MoveTo(center - 1, 0),
        )?;
    }
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

    for (idx, c) in "abcdefgh".split("").enumerate() {
        queue!(
            stdout,
            cursor::MoveTo(
                center + (TILE_WIDTH as u16 + 1) * idx as u16
                    - (TILE_WIDTH as f32 / 2.0).ceil() as u16
                    - 1,
                TILE_HEIGHT as u16 * 8 + 1
            ),
            Print(c)
        )?;
    }

    // render pieces
    for i in (0..8).rev() {
        queue!(
            stdout,
            cursor::MoveTo(center, (TILE_HEIGHT as u16 * (7 - i)) + 1)
        )?;

        for j in 0..8 {
            let idx = match app.check_own_side() {
                Side::White => i * 8 + j,
                Side::Black => 63 - (i * 8 + j),
            };

            let piece = board.piece_at(idx.into());

            let mut piece_string;

            piece_string = match piece {
                Some(ref p) => {
                    if TILE_WIDTH > 4 {
                        p.render(TILE_WIDTH).to_string()
                    } else {
                        p.render_2c().to_string()
                    }
                }
                None => "".into(),
            };

            let is_selected_sq = match selected_piece {
                Some((_, _)) => {
                    if board
                        .current_generated_moves()
                        .contains(&(i * 8 + j).into())
                    {
                        true
                    } else {
                        false
                    }
                }

                None => false,
            };

            if is_selected_sq {
                piece_string = "*".into()
            }

            if let Some((x, y)) = selected_piece {
                if x == j as usize && y == i as usize {
                    piece_string = format!("{}", piece_string.bold());
                }
            }

            if cursor_pos.0 == j && cursor_pos.1 == i {
                piece_string = match piece_string.is_empty() {
                    false => format!("{}", piece_string.bold()),
                    true => format!("{}", "*".bold()),
                };
            } else if let Some(_) = selected_piece {
                if board
                    .current_generated_moves()
                    .contains(&(i * 8 + j).into())
                {
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

    Ok(())
}

pub async fn start(
    app: Arc<Mutex<App>>,
    main_tx: Sender<Message>,
) -> Result<(), Box<dyn std::error::Error>> {
    let events = Events::new(1024);

    let mut stdout = std::io::stdout();
    execute!(stdout, cursor::Hide, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut cursor_pos = (0, 0);
    let mut selected_piece: Option<(usize, usize)> = None;

    loop {
        let mut app = app.lock().await;

        match app.ui_state() {
            UIState::Game => {
                draw_board(&app, cursor_pos, selected_piece, &mut stdout)?;
            }

            &UIState::Menu => {
                /*app.start_new_game("123");
                draw_board(
                    app.game().as_ref().unwrap(),
                    cursor_pos,
                    selected_piece,
                    &mut stdout,
                )?;*/
            }
        }

        stdout.flush()?;

        if let Ok(Event::Input(k)) = events.next() {
            match k {
                Key::Ctrl('s') => {
                    app.seek_for_game().await;
                }
                Key::Char('q') => break,
                Key::Char('h') => {
                    if cursor_pos.0 >= 1 {
                        cursor_pos.0 -= 1;
                    }
                }
                Key::Char('j') => {
                    if cursor_pos.1 >= 1 {
                        cursor_pos.1 -= 1;
                    }
                }
                Key::Char('k') => {
                    if cursor_pos.1 < 7 {
                        cursor_pos.1 += 1;
                    }
                }
                Key::Char('l') => {
                    if cursor_pos.0 < 7 {
                        cursor_pos.0 += 1;
                    }
                }

                Key::Backspace => {
                    selected_piece = None;
                }

                /*Key::Ctrl('g') => {
                    app.start_new_game("abc");
                }*/
                Key::Enter => {
                    let side = app.check_own_side();
                    let board = app.game_mut().as_mut().unwrap().board_mut();

                    match selected_piece {
                        Some(p) => {
                            let idx = match side {
                                Side::White => p.1 * 8 + p.0,
                                Side::Black => 63 - (p.1 * 8 + p.0),
                            };

                            match board.piece_at(idx) {
                                Some(ref piece) => {
                                    let cursor_idx = match side {
                                        Side::White => (cursor_pos.1 * 8 + cursor_pos.0) as usize,
                                        Side::Black => {
                                            63 - ((cursor_pos.1 * 8 + cursor_pos.0) as usize)
                                        }
                                    };

                                    if board /*.generate_moves(idx, &piece)*/
                                        .current_generated_moves()
                                        .contains(&cursor_idx)
                                    {
                                        drop(piece);

                                        let piece = board
                                            .pieces_mut()
                                            .get_mut(idx)
                                            .unwrap()
                                            .as_mut()
                                            .unwrap();
                                        piece.increment_moves();

                                        board.make_move(idx, cursor_idx);
                                        selected_piece = None;
                                        board.set_generated_moves(vec![]);
                                    }
                                }
                                _ => (),
                            }
                        }
                        None => {
                            let idx = match side {
                                Side::White => (cursor_pos.1 * 8 + cursor_pos.0) as usize,
                                Side::Black => 63 - (cursor_pos.1 * 8 + cursor_pos.0) as usize,
                            };

                            if let Some(ref p) = board.piece_at(idx) {
                                if p.side() == board.turn() {
                                    selected_piece = match side {
                                        Side::White => {
                                            Some((cursor_pos.0 as usize, cursor_pos.1 as usize))
                                        }
                                        Side::Black => {
                                            let p = 63 - (cursor_pos.1 * 8 + cursor_pos.0) as usize;
                                            Some((p.x(), p.y()))
                                        }
                                    };

                                    let moves = board.generate_moves(idx, p);

                                    let moves = match side {
                                        Side::White => moves,
                                        Side::Black => moves.iter().map(|x| 63 - x).collect(),
                                    };

                                    board.set_generated_moves(moves);
                                }
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }

    stop();

    Ok(())
}

fn stop() {
    let mut stdout = std::io::stdout();
    execute!(stdout, LeaveAlternateScreen, cursor::Show).unwrap();
    disable_raw_mode().unwrap();
}
