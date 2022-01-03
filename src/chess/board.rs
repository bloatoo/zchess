use super::{CastleKind, Piece, PieceKind, PlayedMove, PlayedMoveKind, Side};

use crate::chess::utils::{idx_to_square, square_to_idx};

use crate::chess::moves::bishop::generate_bishop_moves;
use crate::chess::moves::king::generate_king_moves;
use crate::chess::moves::knight::generate_knight_moves;
use crate::chess::moves::pawn::generate_pawn_moves;
use crate::chess::moves::queen::generate_queen_moves;
use crate::chess::moves::rook::generate_rook_moves;

use std::time::Instant;

#[allow(unused)]
use crate::utils::debug;

pub trait Square {
    fn x(&self) -> usize;
    fn y(&self) -> usize;
    fn pos(&self) -> (usize, usize);
}

impl Square for usize {
    fn x(&self) -> usize {
        self - self.y() * 8
    }

    fn y(&self) -> usize {
        (*self as f32 / 8.0).floor() as usize
    }

    fn pos(&self) -> (usize, usize) {
        (self.x(), self.y())
    }
}

#[derive(Debug, Clone)]
pub enum SquareColor {
    Light,
    Dark,
}

#[derive(Debug, Clone)]
pub enum Edge {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Clone)]
pub struct Board {
    pieces: Vec<Option<Piece>>,
    en_passant: Option<usize>,
    turn: Side,
    current_generated_moves: Vec<usize>,
    played_moves: Vec<PlayedMove>,
    turn_time_taken: Instant,
}

impl Board {
    pub fn current_generated_moves(&self) -> &Vec<usize> {
        &self.current_generated_moves
    }

    pub fn make_move_str(&mut self, mv: &str) {
        let (src, dest) = mv.split_at(2);
        let (src, dest) = (square_to_idx(src), square_to_idx(dest));

        self.make_move(src, dest);
    }

    // used for reverting played moves
    fn make_move_str_raw(&mut self, mv: &str, promotion: bool) {
        let (src, dest) = mv.split_at(2);
        let (src, dest) = (square_to_idx(src), square_to_idx(dest));

        self.make_move_raw(src, dest, promotion);
    }

    pub fn from_str(fen: &str, turn: Side) -> Self {
        let mut pieces: Vec<Option<Piece>> = vec![];

        for row in fen.split("/") {
            for c in row.split("") {
                use PieceKind::*;
                use Side::*;
                match c {
                    "P" => pieces.push(Some(Piece::new(Pawn, White))),
                    "N" => pieces.push(Some(Piece::new(Knight, White))),
                    "B" => pieces.push(Some(Piece::new(Bishop, White))),
                    "R" => pieces.push(Some(Piece::new(Rook, White))),
                    "Q" => pieces.push(Some(Piece::new(Queen, White))),
                    "K" => pieces.push(Some(Piece::new(King, White))),

                    "p" => pieces.push(Some(Piece::new(Pawn, Black))),
                    "n" => pieces.push(Some(Piece::new(Knight, Black))),
                    "b" => pieces.push(Some(Piece::new(Bishop, Black))),
                    "r" => pieces.push(Some(Piece::new(Rook, Black))),
                    "q" => pieces.push(Some(Piece::new(Queen, Black))),
                    "k" => pieces.push(Some(Piece::new(King, Black))),
                    _ => (),
                }

                if let Ok(res) = c.parse::<usize>() {
                    for _ in 0..res {
                        pieces.push(None);
                    }
                }
            }
        }

        Self {
            pieces,
            turn,
            en_passant: None,
            current_generated_moves: vec![],
            played_moves: vec![],
            turn_time_taken: Instant::now(),
        }
    }

    pub fn turn_time_taken(&self) -> &Instant {
        &self.turn_time_taken
    }

    pub fn reset_turn_timer(&mut self) {
        self.turn_time_taken = Instant::now();
    }

    pub fn revert_move(&mut self) {
        if let Some(mv) = self.played_moves.last() {
            if *mv.kind() == PlayedMoveKind::Promotion {
                let rev_mv = mv.reverse().first().unwrap().clone();
                self.make_move_str_raw(&rev_mv, true);
                return;
            }

            for mv in mv.reverse() {
                self.make_move_str_raw(&mv, false);
            }

            self.swap_turn();

            self.played_moves.pop();
        }
    }

    pub fn played_moves(&self) -> &Vec<PlayedMove> {
        &self.played_moves
    }

    pub fn generate_moves(&self, sq: usize, piece: &Piece) -> Vec<usize> {
        use PieceKind::*;

        let mut moves = match piece.kind() {
            Pawn => generate_pawn_moves(&self, sq, piece),
            Rook => generate_rook_moves(&self, sq, piece),
            Knight => generate_knight_moves(&self, sq, piece),
            Bishop => generate_bishop_moves(&self, sq, piece),
            Queen => generate_queen_moves(&self, sq, piece),
            King => generate_king_moves(&self, sq, piece),
        };

        moves.retain(|m| m < &64);

        if piece.side() == &self.turn {
            let mut board = self.clone();

            for mv in moves.clone().iter() {
                board.make_move(sq, *mv);
                board.swap_turn();

                if board.is_check(piece.side()) {
                    moves.retain(|m| m != mv);
                }

                board.swap_turn();
                board.revert_move();
            }
        }

        moves
    }

    pub fn is_check(&self, side: &Side) -> bool {
        let mut king = None;

        for (idx, piece) in self.pieces.iter().enumerate() {
            if let Some(p) = piece {
                if p.kind() == &PieceKind::King && p.side() == side {
                    king = Some(idx)
                }
            }
        }

        for (idx, piece) in self.pieces.iter().enumerate() {
            if let Some(p) = piece.as_ref() {
                if p.kind() == &PieceKind::King {
                    continue;
                }

                if p.side() == side {
                    continue;
                }

                if p.side() != side && self.generate_moves(idx, p).contains(&king.unwrap()) {
                    return true;
                }
            }
        }

        false
    }

    pub fn swap_turn(&mut self) {
        self.turn = match self.turn {
            Side::White => Side::Black,
            Side::Black => Side::White,
        };
    }

    pub fn fen(&self) -> String {
        let mut string = String::new();
        self.pieces.iter().enumerate().for_each(|(idx, piece)| {
            if idx % 8 == 0 {
                string.push('/');
            } else {
                if let Some(p) = piece {
                    string.push_str(&format!("{}", p.as_ref()));
                } else {
                    string.push('e')
                }
            }
        });

        string
    }

    pub async fn submit_move(
        &mut self,
        source: usize,
        dest: usize,
        game_id: String,
        token: String,
        online: bool,
    ) {
        self.make_move(source, dest);
        if online {
            let (src, dest) = (idx_to_square(source), idx_to_square(dest));

            let client = reqwest::Client::new();
            let url = format!(
                "https://lichess.org/api/board/game/{}/move/{}{}",
                game_id, src, dest
            );

            let token = format!("Bearer {}", token);

            client
                .post(url)
                .header("Authorization", token)
                .send()
                .await
                .unwrap();
        }
    }

    pub fn make_move(&mut self, source: usize, dest: usize) {
        let piece = self.piece_at(source).clone().unwrap();

        let (src_str, dest_str) = (idx_to_square(source), idx_to_square(dest));

        if piece.kind() == &PieceKind::King && *piece.move_count() == 0 {
            let idx = dest as isize - source as isize;

            if idx == 2 || idx == -2 {
                let long = idx < 2;

                self.castle(piece.side().clone(), long);
                return;
            }
        }

        if *piece.kind() == PieceKind::Pawn && (dest.y() == 7 || dest.y() == 0) {
            self.promote_piece(source, dest);
            return;
        }

        let mv = PlayedMove::new(PlayedMoveKind::Normal, format!("{}{}", src_str, dest_str));

        self.played_moves.push(mv);

        self.set_piece(dest, Some(piece));
        self.set_piece(source, None);

        self.swap_turn();
    }

    fn make_move_raw(&mut self, source: usize, dest: usize, promotion: bool) {
        let piece = self.piece_at(source).clone().unwrap();

        if promotion {
            let new_piece = Piece::new(PieceKind::Pawn, piece.side().clone());
            self.set_piece(dest, Some(new_piece));
        } else {
            self.set_piece(dest, Some(piece));
        }

        self.set_piece(source, None);
    }

    pub fn castle(&mut self, side: Side, long: bool) {
        let king_idx = match side {
            Side::White => 4,
            Side::Black => 60,
        };

        let rook_idx = match side {
            Side::Black => match long {
                true => 56,
                false => 63,
            },
            Side::White => match long {
                true => 0,
                false => 7,
            },
        };

        let dest_squares: (usize, usize) = match side {
            Side::White => match long {
                true => (2, 3),
                false => (6, 5),
            },
            Side::Black => match long {
                true => (58, 59),
                false => (62, 61),
            },
        };

        let king = self.piece_at(king_idx).clone().unwrap();
        let rook = self.piece_at(rook_idx).clone().unwrap();

        self.set_piece(dest_squares.0, Some(king));
        self.set_piece(king_idx, None);

        self.set_piece(dest_squares.1, Some(rook));
        self.set_piece(rook_idx, None);

        let castle_kind = match (side, long) {
            (Side::White, true) => CastleKind::WhiteLong,
            (Side::White, false) => CastleKind::WhiteShort,
            (Side::Black, true) => CastleKind::BlackLong,
            (Side::Black, false) => CastleKind::BlackShort,
        };

        let mv = PlayedMove::new(
            PlayedMoveKind::Castle(castle_kind),
            format!(
                "{}{}",
                idx_to_square(king_idx),
                idx_to_square(dest_squares.0)
            ),
        );

        self.played_moves.push(mv);

        self.swap_turn();
    }

    pub fn promote_piece(&mut self, source: usize, dest: usize) {
        if let Some(p) = self.piece_at(source) {
            if *p.kind() != PieceKind::Pawn {
                return;
            }

            let new_piece = Piece::new(PieceKind::Queen, p.side().clone());

            drop(p);

            self.set_piece(dest, Some(new_piece));
            self.set_piece(source, None);

            let (src, dst) = (idx_to_square(source), idx_to_square(dest));
            let mv = PlayedMove::new(PlayedMoveKind::Promotion, format!("{}{}q", src, dst));
            self.played_moves.push(mv);

            self.swap_turn();
        }
    }

    fn set_piece(&mut self, dest: usize, piece: Option<Piece>) {
        let p = self.pieces.get_mut(dest).unwrap();
        *p = piece;
    }

    pub fn piece_at(&self, square: usize) -> &Option<Piece> {
        if square > 63 {
            return &None;
        }

        self.pieces.get(square).unwrap()
    }

    pub fn get_row(square: usize) -> usize {
        (square as f32 / 8.0).floor() as usize
    }

    pub fn pieces(&self) -> &Vec<Option<Piece>> {
        &self.pieces
    }

    pub fn pieces_mut(&mut self) -> &mut Vec<Option<Piece>> {
        &mut self.pieces
    }

    pub fn turn(&self) -> &Side {
        &self.turn
    }

    pub fn set_generated_moves(&mut self, moves: Vec<usize>) {
        self.current_generated_moves = moves;
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::from_str("RNBQKBNR/PPPPPPPP/8/8/8/8/pppppppp/rnbqkbnr", Side::White)
    }
}
