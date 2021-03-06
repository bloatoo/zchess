use crate::{
    app::App,
    chess::{
        board::SquareColor,
        utils::{get_square_color, uci_to_idx},
        Side, Square,
    },
    message::Message,
    ui::event::*,
    user::User,
    utils::{fmt_clock, parse_config_hex},
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

const DARK_SQUARE_DEFAULT_COLOR: (u8, u8, u8) = (240, 217, 181);
const LIGHT_SQUARE_DEFAULT_COLOR: (u8, u8, u8) = (181, 136, 99);
const LEGAL_MOVE_INDICATOR_DEFAULT_COLOR: (u8, u8, u8) = (220, 200, 0);

const WHITE_PIECE_DEFAULT_COLOR: (u8, u8, u8) = (0, 0, 0);
const BLACK_PIECE_DEFAULT_COLOR: (u8, u8, u8) = (0, 0, 0);

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
    let size = terminal::size()?;

    let dark_square_color = app.config().dark_square_color();
    let dark_square = parse_config_hex(dark_square_color, DARK_SQUARE_DEFAULT_COLOR);

    let light_square_color = app.config().light_square_color();
    let light_square = parse_config_hex(&light_square_color, LIGHT_SQUARE_DEFAULT_COLOR);

    let legal_move_indicator_color = app.config().legal_move_indicator_color();
    let legal_move_indicator = parse_config_hex(
        legal_move_indicator_color,
        LEGAL_MOVE_INDICATOR_DEFAULT_COLOR,
    );

    let black_piece_color = app.config().black_piece_color();
    let black_piece = parse_config_hex(black_piece_color, BLACK_PIECE_DEFAULT_COLOR);

    let white_piece_color = app.config().white_piece_color();
    let white_piece = parse_config_hex(white_piece_color, WHITE_PIECE_DEFAULT_COLOR);

    let mut tile_width = 8;
    let mut tile_height = 4;

    if !app.small_board() {
        while tile_width * 8 > size.0 as usize / 2 {
            tile_width -= 1;
        }

        while tile_width * 8 < size.0 as usize - (size.0 as f32 / 1.5) as usize {
            tile_width += 1;
        }

        while tile_height * 8 > size.1 as usize - size.1 as usize / 8 {
            tile_height -= 1;
        }

        while tile_height * 8 < (size.1 as f32 * 0.7) as usize {
            tile_height += 1;
        }
    } else {
        tile_width = 4;
        tile_height = 2;
    }

    let tile_str = format!(
        "{}{}",
        " ".repeat(tile_width + 1).on(dark_square),
        " ".repeat(tile_width + 1).on(light_square)
    );

    let tile_str_alt = format!(
        "{}{}",
        " ".repeat(tile_width + 1).on(light_square),
        " ".repeat(tile_width + 1).on(dark_square)
    );

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

        let mut white = format!(
            "white: {} ({}) [{}] | ",
            w_player.name(),
            w_player.rating(),
            fmt_clock(wtime),
        );

        if *board.turn() == Side::White {
            white = format!("{}", white.bold());
        }

        let mut black = format!(
            "black: {} ({}) [{}]",
            b_player.name(),
            b_player.rating(),
            fmt_clock(btime)
        );

        if *board.turn() == Side::Black {
            black = format!("{}", black.bold());
        }

        let clock = game.data().clock();

        format!(
            "id: {} | {}{} | {}+{}",
            game.id(),
            white,
            black,
            clock.initial() / 1000 / 60,
            clock.increment() / 1000
        )
    } else {
        let mut white = format!("white: {} | ", fmt_clock(wtime));

        if *board.turn() == Side::White {
            white = format!("{}", white.bold());
        }

        let mut black = format!("black: {}", fmt_clock(btime));

        if *board.turn() == Side::Black {
            black = format!("{}", black.bold());
        }

        format!("{}{}", white, black)
    };

    let (_, y) = terminal::size().unwrap();

    if no_board {
        execute!(
            stdout,
            cursor::MoveTo(0, y),
            Clear(ClearType::CurrentLine),
            Print(statusline.clone())
        )?;

        return Ok(());
    }

    let center_y = y / 2 - ((tile_height as u16 * 8) as f32 / 2.0).ceil() as u16;

    // clear terminal and draw statusline
    execute!(
        stdout,
        Clear(ClearType::All),
        cursor::MoveTo(0, y),
        Print(statusline),
    )?;

    // draw chat | deprecated, chat will be available on game dashboard
    /*for (idx, msg) in app.game().as_ref().unwrap().messages().iter().enumerate() {
        let msg_string = format!("{}: {}", msg.username(), msg.text());

        execute!(
            stdout,
            cursor::MoveTo(x - x / 5, center_y + idx as u16),
            Print(msg_string)
        )?;
    }*/

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

    // move cursor to center
    execute!(stdout, cursor::MoveTo(center - 1, center_y))?;

    // print
    for i in 0..8 {
        // print tile's vertical lines
        for _ in 0..tile_height {
            let current_row = match i % 2 {
                0 => &tile_str,
                _ => &tile_str_alt,
            };

            execute!(
                stdout,
                cursor::MoveToNextLine(1),
                cursor::MoveToColumn(center),
                Print(&format!("{}", current_row.repeat(4))),
            )?;
        }
    }

    for (idx, c) in "abcdefgh".split("").enumerate() {
        execute!(
            stdout,
            cursor::MoveTo(
                center + (tile_width as u16 + 1) * idx as u16
                    - (tile_width as f32 / 2.0).ceil() as u16
                    - 2,
                center_y + tile_height as u16 * 8 + 1
            ),
            Print(format!("{}", c.with(Color::DarkGrey)))
        )?;
    }

    let extra_y = match app.config().center_pieces() {
        true if tile_height > 1 => (tile_height as f32 / 2.0).floor() as u16 + 1,
        true => 0,
        false => 0,
    };

    // render pieces
    for i in (0..8).rev() {
        execute!(
            stdout,
            cursor::MoveTo(center, center_y + (tile_height as u16 * (7 - i)) + extra_y)
        )?;

        for j in 0..8 {
            let idx = match app.board_display_side() {
                Side::White => i * 8 + j,
                Side::Black => 63 - (i * 8 + j),
            } as usize;

            let color = match get_square_color(idx) {
                SquareColor::Light => light_square,
                SquareColor::Dark => dark_square,
            };

            let piece = board.piece_at(idx.into());

            let mut piece_string;

            piece_string = match piece {
                Some(ref p) => {
                    if let Some(r) = app.config().piece_render(p.kind()) {
                        match p.side() {
                            Side::White => r.render_white().clone(),
                            Side::Black => r.render_black().clone(),
                        }
                    } else if tile_width >= 6 {
                        p.render(tile_width).to_string()
                    } else if *app.small_board() {
                        p.render_char()
                    } else {
                        p.render_char()
                    }
                }
                None => "".into(),
            };

            let piece_string_raw = piece_string.clone();

            if let Some(ref p) = piece {
                piece_string = match p.side() {
                    Side::White => {
                        format!("{}", piece_string.with(white_piece))
                    }
                    Side::Black => {
                        format!("{}", piece_string.with(black_piece))
                    }
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
                    piece_string = format!("{}", "*".with(legal_move_indicator));
                }
            }

            if let Some(mv) = board.played_moves().last() {
                let (src, dest) = uci_to_idx(&mv.uci());

                if src == idx {
                    piece_string += &format!("{}", "*".with(Color::Blue).bold().on(color));
                } else if dest == idx {
                    piece_string += &format!("{}", "*".with(Color::Yellow).bold().on(color));
                }
            }

            let extra_x = if tile_width > 4 {
                (tile_width as u16 - piece_string_raw.len() as u16) / 2
            } else {
                1
            };

            piece_string = piece_string.on(color).to_string();

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
    cursor_pos: &mut (u16, u16),
    stdout: &mut Stdout,
) -> Result<(), Box<dyn std::error::Error>> {
    let menu_items = vec!["New Lichess game", "Local game"];

    if cursor_pos.1 > menu_items.len() as u16 {
        cursor_pos.1 = 0;
    }

    execute!(stdout, Clear(ClearType::All))?;

    let size = terminal::size().unwrap();

    for (idx, i) in menu_items.iter().enumerate() {
        let center_x = size.0 / 2 - i.len() as u16 / 2;
        let center_y = size.1 / 2 + idx as u16;

        let mut final_string: String = i.to_string();

        if cursor_pos.1 == idx as u16 {
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
                draw_menu(&app, &mut cursor_pos, &mut stdout)?;
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
                Key::Char('h') | Key::Left if app.ui_state() == &UIState::Game => {
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

                Key::Char('j') | Key::Down if app.ui_state() == &UIState::Game => {
                    if cursor_pos.1 >= 1 {
                        cursor_pos.1 -= 1;
                    }
                }

                Key::Char('z') if app.ui_state() == &UIState::Game => {
                    app.toggle_small_board();
                }

                Key::Char('j') | Key::Down => {
                    if cursor_pos.1 < 1 {
                        cursor_pos.1 += 1;
                    }
                }

                Key::Char('k') | Key::Up if app.ui_state() == &UIState::Game => {
                    if cursor_pos.1 < 7 {
                        cursor_pos.1 += 1;
                    }
                }

                Key::Char('k') | Key::Up => {
                    if cursor_pos.1 >= 1 {
                        cursor_pos.1 -= 1;
                    }
                }

                Key::Char('f') if app.ui_state() == &UIState::Game => {
                    app.flip_board();
                    let new_cursor_idx = 63 - (cursor_pos.0 + cursor_pos.1 * 8);

                    let new_y = (new_cursor_idx as f32 / 8.0).floor() as u16;
                    let new_x = new_cursor_idx - new_y * 8;

                    cursor_pos = (new_x, new_y);
                }

                Key::Char('l') | Key::Right if app.ui_state() == &UIState::Game => {
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

                            let render_side = app.board_display_side().clone();

                            let id = app.game().as_ref().unwrap().id().to_string();
                            let token = app.config().token().to_string();
                            let board = app.game_mut().as_mut().unwrap().board_mut();

                            match selected_piece {
                                Some(p) => {
                                    let idx = p.1 * 8 + p.0;

                                    let cursor_idx = match render_side {
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
                                                Side::White => (
                                                    wtime - turn_time_taken as u64
                                                        + game.data().clock().increment(),
                                                    btime,
                                                ),
                                                Side::Black => (
                                                    wtime,
                                                    btime - turn_time_taken as u64
                                                        + game.data().clock().increment(),
                                                ),
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

                                    let idx = match render_side {
                                        Side::White => (cursor_pos.1 * 8 + cursor_pos.0) as usize,
                                        Side::Black => {
                                            63 - (cursor_pos.1 * 8 + cursor_pos.0) as usize
                                        }
                                    };

                                    if let Some(ref p) = board.piece_at(idx) {
                                        if p.side() == board.turn() {
                                            selected_piece = match render_side {
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
