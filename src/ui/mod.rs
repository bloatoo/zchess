use crate::{
    app::App,
    chess::{utils::uci_to_idx, Side, Square},
    message::Message,
    ui::event::*,
    user::User,
    utils::fmt_clock,
};

use std::io::{Stdout, Write};
use std::sync::mpsc::Sender;
use std::sync::Arc;
use tokio::sync::Mutex;

use crossterm::{
    cursor, execute,
    style::{Color, Print, Stylize},
    terminal::{
        self, disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

const H_LINE: &str = "─";
const H_CORNER: &str = "┼";

pub mod event;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UIState {
    Menu,
    Profile(User),
    Seek,
    Game,
}

pub fn draw_seek(stdout: &mut Stdout) -> Result<(), Box<dyn std::error::Error>> {
    let string = "Seeking for a new game...";

    let size = terminal::size().unwrap();

    let center_x = size.0 / 2 - string.len() as u16 / 2;
    let center_y = size.1 / 2;

    execute!(stdout, cursor::MoveTo(center_x, center_y), Print(string))?;
    Ok(())
}

pub fn draw_profile(user: &User, cursor_pos: (u16, u16), stdout: &mut Stdout) {}

pub fn draw_board(
    app: &App,
    cursor_pos: (u16, u16),
    selected_piece: Option<(usize, usize)>,
    stdout: &mut Stdout,
    no_board: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let tile_width = match app.config().tile_width() {
        Some(w) => *w,
        None => 8,
    };

    let tile_height = match app.config().tile_height() {
        Some(h) => *h,
        None => 4,
    };

    let tile_str = format!("│{}", " ".repeat(tile_width));
    let size = terminal::size()?;
    let center = size.0 / 2 - tile_width as u16 * 4 - 2;

    let game = app.game().as_ref().unwrap();

    let board = game.board();

    let (wtime, btime) = match board.played_moves().len() >= 2 {
        true => match board.turn() {
            &Side::White => {
                let wtime =
                    *game.state().wtime() - board.turn_time_taken().elapsed().as_millis() as u64;
                (wtime, *game.state().btime())
            }
            &Side::Black => {
                let btime =
                    *game.state().btime() - board.turn_time_taken().elapsed().as_millis() as u64;
                (*game.state().wtime(), btime)
            }
        },
        false => (*game.state().wtime(), *game.state().btime()),
    };

    let statusline = if game.is_online() {
        let w_player = game.data().white();
        let b_player = game.data().black();

        let names = format!(
            "white: {} ({}) [{}] | black: {} ({}) [{}]",
            w_player.name(),
            w_player.id(),
            fmt_clock(wtime),
            b_player.name(),
            b_player.id(),
            fmt_clock(btime)
        );

        let clock = game.data().clock();

        format!(
            "id: {} | {} | {}+{}",
            game.id(),
            names,
            clock.initial() / 1000 / 60,
            clock.increment() / 1000
        )
    } else {
        format!("white: {} | black: {}", fmt_clock(wtime), fmt_clock(btime))
    };

    let (x, y) = terminal::size().unwrap();

    if no_board {
        execute!(
            stdout,
            cursor::MoveTo(0, y),
            Clear(ClearType::CurrentLine),
            Print(statusline.clone())
        )?;

        return Ok(());
    }

    let center_y = y / 2 - ((tile_height as u16 * 8) as f32 / 2.0).ceil() as u16 - 2;

    // clear terminal and draw statusline
    execute!(
        stdout,
        Clear(ClearType::All),
        cursor::MoveTo(0, y),
        Print(statusline),
    )?;

    // draw chat
    for (idx, msg) in app.game().as_ref().unwrap().messages().iter().enumerate() {
        let msg_string = format!("{}: {}", msg.username(), msg.text());

        execute!(
            stdout,
            cursor::MoveTo(x - x / 5, center_y + idx as u16),
            Print(msg_string)
        )?;
    }

    // print rows
    for i in 1..=8 {
        execute!(
            stdout,
            cursor::MoveTo(
                center - 3,
                center_y + tile_height as u16 * i - tile_height as u16 / 2
            ),
            Print(format!("{}", (9 - i).to_string().with(Color::DarkGrey))),
            cursor::MoveTo(center - 1, 0),
        )?;
    }

    // print first line
    execute!(
        stdout,
        cursor::MoveTo(center - 1, center_y),
        Print(&format!("┬{}", H_LINE.repeat((tile_width as usize + 1) - 1)).repeat(8)),
        Print("┬")
    )?;

    // print
    for i in 0..8 {
        // print tile's vertical lines
        for _ in 0..tile_height {
            execute!(
                stdout,
                cursor::MoveToNextLine(1),
                cursor::MoveToColumn(center),
                Print(&format!("{}", tile_str.repeat(9))),
            )?;
        }

        // print horizontal line
        if i != 7 {
            execute!(
                stdout,
                cursor::MoveToColumn(center),
                Print(
                    &format!(
                        "{}{}",
                        H_CORNER,
                        H_LINE.repeat((tile_width as usize + 1) - 1)
                    )
                    .repeat(8)
                ),
                Print(H_CORNER)
            )?;
        } else {
            execute!(
                stdout,
                cursor::MoveToColumn(center),
                Print(
                    &format!("{}{}", "┴", H_LINE.repeat((tile_width as usize + 1) - 1)).repeat(8)
                ),
                Print("┴")
            )?;
        }
    }

    for (idx, c) in "abcdefgh".split("").enumerate() {
        execute!(
            stdout,
            cursor::MoveTo(
                center + (tile_width as u16 + 1) * idx as u16
                    - (tile_width as f32 / 2.0).ceil() as u16
                    - 1,
                center_y + tile_height as u16 * 8 + 1
            ),
            Print(format!("{}", c.with(Color::DarkGrey)))
        )?;
    }

    let extra_y = match app.config().center_pieces() {
        true => 1,
        false => 0,
    };

    // render pieces
    for i in (0..8).rev() {
        execute!(
            stdout,
            cursor::MoveTo(
                center,
                center_y + (tile_height as u16 * (7 - i)) + 1 + extra_y
            )
        )?;

        for j in 0..8 {
            let idx = match app.check_own_side() {
                Side::White => i * 8 + j,
                Side::Black => 63 - (i * 8 + j),
            } as usize;

            let piece = board.piece_at(idx.into());

            let mut piece_string;

            piece_string = match piece {
                Some(ref p) => {
                    if let Some(r) = app.config().piece_render(p.kind()) {
                        match p.side() {
                            Side::White => r.render_white().clone(),
                            Side::Black => r.render_black().clone(),
                        }
                    } else if tile_width > 4 {
                        p.render(tile_width).to_string()
                    } else {
                        p.render_2c().to_string()
                    }
                }
                None => "".into(),
            };

            let piece_string_raw = piece_string.clone();

            if let Some(ref p) = piece {
                if p.side() == &app.check_own_side() {
                    piece_string = format!("{}", piece_string.with(Color::Cyan));
                }
            }

            let is_selected_sq = match selected_piece {
                Some((_, _)) => {
                    if board.current_generated_moves().contains(&idx) {
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
                if x == idx.x() as usize && y == idx.y() as usize {
                    piece_string = format!("{}", piece_string.bold());
                }
            }

            if cursor_pos.0 == j && cursor_pos.1 == i {
                piece_string = match piece_string.is_empty() {
                    false => format!("{}", piece_string.bold()),
                    true => format!("{}", "*".bold()),
                };
            } else if let Some(_) = selected_piece {
                if board.current_generated_moves().contains(&idx) {
                    piece_string = format!("{}", "*".with(Color::DarkGrey));
                }
            }

            if let Some(mv) = board.played_moves().last() {
                let (src, dest) = uci_to_idx(&mv.uci());

                if src == idx {
                    piece_string += &format!("{}", "*".with(Color::Blue).bold());
                } else if dest == idx {
                    piece_string += &format!("{}", "*".with(Color::Yellow).bold());
                }
            }

            let extra_x = (tile_width as u16 - piece_string_raw.len() as u16) / 2;

            execute!(
                stdout,
                cursor::MoveToColumn(center + 1 + (tile_width as u16 + 1) * j as u16 + extra_x),
                Print(piece_string),
            )?;
        }
    }

    Ok(())
}

pub fn draw_menu(
    app: &App,
    cursor_pos: (u16, u16),
    stdout: &mut Stdout,
) -> Result<(), Box<dyn std::error::Error>> {
    execute!(stdout, Clear(ClearType::All))?;
    let menu_items = vec!["New Lichess game", "Local game"];

    let size = terminal::size().unwrap();

    for (idx, i) in menu_items.iter().enumerate() {
        let center_x = size.0 / 2 - i.len() as u16 / 2;
        let center_y = size.1 / 2 + idx as u16;

        let mut final_string: String = i.to_string();

        if menu_items.len() as u16 - cursor_pos.1 == idx as u16 {
            final_string = format!("{}", final_string.bold());
        }

        execute!(
            stdout,
            cursor::MoveTo(center_x, center_y),
            Print(final_string)
        )?;
    }

    let header_string = match app.own_info() {
        Some(info) => {
            format!("Logged in as: {}", info.username())
        }
        None => String::from("Loading Lichess info..."),
    };

    execute!(
        stdout,
        cursor::MoveTo(size.0 - header_string.len() as u16, 0),
        Clear(ClearType::CurrentLine),
        Print(header_string)
    )?;

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

    let mut size = terminal::size().unwrap();

    loop {
        let mut app = app.lock().await;

        match app.ui_state() {
            UIState::Game => {
                let curr_size = terminal::size().unwrap();

                if app.state_changed || curr_size != size {
                    draw_board(&app, cursor_pos, selected_piece, &mut stdout, false)?;
                    app.state_changed = false;
                    size = curr_size;
                } else {
                    draw_board(&app, cursor_pos, selected_piece, &mut stdout, true)?;
                }
            }

            UIState::Profile(user) => {
                draw_profile(user, cursor_pos, &mut stdout);
            }

            &UIState::Menu => {
                draw_menu(&app, cursor_pos, &mut stdout)?;
            }
            UIState::Seek => {
                draw_seek(&mut stdout)?;
            }
        }

        stdout.flush()?;

        if let Ok(Event::Input(k)) = events.next() {
            app.state_changed = true;
            match k {
                Key::Char('q') => break,
                Key::Char('h') | Key::Left => {
                    if cursor_pos.0 >= 1 {
                        cursor_pos.0 -= 1;
                    }
                }

                Key::Char('a') if app.ui_state() == &UIState::Game => {
                    app.abort_game().await;
                }

                Key::Char('r') if app.ui_state() == &UIState::Game => {
                    app.resign_game().await;
                }

                Key::Char('j') | Key::Down => {
                    if cursor_pos.1 >= 1 {
                        cursor_pos.1 -= 1;
                    }
                }
                Key::Char('k') | Key::Up => {
                    if cursor_pos.1 < 7 {
                        cursor_pos.1 += 1;
                    }
                }
                Key::Char('l') | Key::Right => {
                    if cursor_pos.0 < 7 {
                        cursor_pos.0 += 1;
                    }
                }

                Key::Backspace => {
                    selected_piece = None;
                }

                Key::Enter => {
                    match app.ui_state() {
                        UIState::Menu => match cursor_pos.1 {
                            0 => {
                                if app.own_info().is_some() {
                                    app.seek_for_game().await;
                                } else {
                                }
                            }
                            1 => app.local_game(),
                            _ => (),
                        },

                        UIState::Seek => {}
                        UIState::Profile(_) => {}

                        UIState::Game => {
                            let is_online = app.game().as_ref().unwrap().is_online();

                            let side = if is_online {
                                app.check_own_side()
                            } else {
                                Side::White
                            };

                            let id = app.game().as_ref().unwrap().id().to_string();
                            let token = app.config().token().to_string();
                            let board = app.game_mut().as_mut().unwrap().board_mut();

                            match selected_piece {
                                Some(p) => {
                                    let idx = p.1 * 8 + p.0;

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
                                        let piece = board
                                            .pieces_mut()
                                            .get_mut(idx)
                                            .unwrap()
                                            .as_mut()
                                            .unwrap();
                                        piece.increment_moves();

                                        let piece_side = piece.side().clone();
                                        let turn_time_taken =
                                            board.turn_time_taken().elapsed().as_millis();

                                        board
                                            .submit_move(idx, cursor_idx, id, token, is_online)
                                            .await;
                                        selected_piece = None;
                                        board.set_generated_moves(vec![]);

                                        drop(board);

                                        let game = app.game_mut().as_mut().unwrap();
                                        game.incr_move_count();

                                        let mut new_state = game.state().clone();

                                        let wtime = *game.state().wtime();
                                        let btime = *game.state().btime();

                                        if *game.move_count() >= 3 {
                                            let (wtime, btime) = match piece_side {
                                                Side::White => {
                                                    (wtime - turn_time_taken as u64, btime)
                                                }
                                                Side::Black => {
                                                    (wtime, btime - turn_time_taken as u64)
                                                }
                                            };

                                            new_state.set_btime(btime);
                                            new_state.set_wtime(wtime);

                                            game.set_state(new_state);
                                        }

                                        if !game.is_online() {
                                            game.board_mut().reset_turn_timer();
                                        }
                                    }
                                }
                                None => {
                                    if is_online {
                                        if *board.turn() != side {
                                            continue;
                                        }
                                    }

                                    let idx = match side {
                                        Side::White => (cursor_pos.1 * 8 + cursor_pos.0) as usize,
                                        Side::Black => {
                                            63 - (cursor_pos.1 * 8 + cursor_pos.0) as usize
                                        }
                                    };

                                    if let Some(ref p) = board.piece_at(idx) {
                                        if p.side() == board.turn() {
                                            selected_piece = match side {
                                                Side::White => Some((
                                                    cursor_pos.0 as usize,
                                                    cursor_pos.1 as usize,
                                                )),
                                                Side::Black => {
                                                    let p = 63
                                                        - (cursor_pos.1 * 8 + cursor_pos.0)
                                                            as usize;
                                                    Some((p.x(), p.y()))
                                                }
                                            };

                                            let moves = board.generate_moves(idx, p);

                                            board.set_generated_moves(moves);
                                        }
                                    }
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
