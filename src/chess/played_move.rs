use crate::chess::{utils::square_to_idx, CastleKind};

#[derive(Debug, Clone)]
pub struct PlayedMove {
    kind: PlayedMoveKind,
    uci: String,
}

impl PlayedMove {
    pub fn new(kind: PlayedMoveKind, uci: String) -> Self {
        Self { kind, uci }
    }
}

#[derive(Debug, Clone)]
pub enum PlayedMoveKind {
    Castle(CastleKind),
    Promotion,
    Normal,
}
