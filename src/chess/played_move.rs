use crate::chess::CastleKind;

#[derive(Debug, Clone)]
pub struct PlayedMove {
    kind: PlayedMoveKind,
    uci: String,
}

impl PlayedMove {
    pub fn new(kind: PlayedMoveKind, uci: String) -> Self {
        Self { kind, uci }
    }

    pub fn reverse(&self) -> String {
        let (src, mut dest) = self.uci.split_at(2);

        if dest.len() > 2 {
            dest = &dest[..dest.len() - 1];
        }

        vec![src, dest].join("")
    }
}

#[derive(Debug, Clone)]
pub enum PlayedMoveKind {
    Castle(CastleKind),
    Promotion,
    Normal,
}
