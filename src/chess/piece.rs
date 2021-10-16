#[derive(Debug, Clone)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone)]
pub struct Piece {
    kind: PieceKind,
    side: Side,
    move_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Side {
    White,
    Black,
}

impl Piece {
    pub fn new(kind: PieceKind, side: Side) -> Self {
        Self {
            kind,
            side,
            move_count: 0,
        }
    }

    pub fn kind(&self) -> &PieceKind {
        &self.kind
    }

    pub fn side(&self) -> &Side {
        &self.side
    }

    pub fn move_count(&self) -> &u32 {
        &self.move_count
    }

    pub fn render(&self, tile_width: usize) -> &str {
        use PieceKind::*;

        let mut piece_str: &str = match self.kind {
            Pawn => "pawn",
            Rook => "rook",
            Bishop => "bishop",
            Knight => "knight",
            Queen => "queen",
            King => "king",
        };

        if piece_str.len() > tile_width {
            piece_str = &piece_str[..tile_width]
        }

        piece_str
    }
}

impl AsRef<str> for Piece {
    fn as_ref(&self) -> &'static str {
        use PieceKind::*;
        match self.kind {
            Pawn => "p",
            Knight => "n",
            Bishop => "b",
            Rook => "r",
            Queen => "q",
            King => "k",
        }
    }
}
