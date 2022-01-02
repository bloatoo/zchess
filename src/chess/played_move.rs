use crate::chess::utils::move_to_uci;
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

    pub fn uci(&self) -> &String {
        &self.uci
    }

    pub fn kind(&self) -> &PlayedMoveKind {
        &self.kind
    }

    pub fn reverse(&self) -> Vec<String> {
        use PlayedMoveKind::*;
        match &self.kind {
            Normal => {
                let (src, dest) = self.uci.split_at(2);

                vec![vec![dest, src].join("")]
            }

            Promotion => {
                let (src, mut dest) = self.uci.split_at(2);
                dest = &dest[..dest.len() - 1];

                vec![vec![dest, src].join("")]
            }

            Castle(kind) => {
                use CastleKind::*;

                match kind {
                    WhiteLong => vec![move_to_uci(2, 4), move_to_uci(3, 0)],
                    WhiteShort => vec![move_to_uci(6, 4), move_to_uci(5, 7)],
                    BlackLong => vec![move_to_uci(58, 60), move_to_uci(59, 56)],
                    BlackShort => vec![move_to_uci(62, 60), move_to_uci(61, 63)],
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayedMoveKind {
    Castle(CastleKind),
    Promotion,
    Normal,
}
