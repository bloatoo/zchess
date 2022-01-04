use crate::chess::PieceKind;
use serde::Deserialize;
use std::collections::HashMap;
use std::{env, fs};
use toml::de::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct PieceRender {
    render_black: String,
    render_white: String,
}

impl PieceRender {
    pub fn render_black(&self) -> &String {
        &self.render_black
    }

    pub fn render_white(&self) -> &String {
        &self.render_white
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pieces: HashMap<String, PieceRender>,
    token: String,
    #[serde(default)]
    debug: bool,
    #[serde(default)]
    center_pieces: bool,
    #[serde(default)]
    dark_square_color: String,
    #[serde(default)]
    light_square_color: String,
    #[serde(default)]
    legal_move_indicator_color: String,
}

impl Config {
    pub fn new() -> Result<Self, Error> {
        let home = env::var("HOME").expect("Failed getting $HOME.");
        let path = format!("{}/.config/zchess.toml", home);
        let data = fs::read_to_string(path).expect("No configuration file found.");
        toml::from_str(&data)
    }

    pub fn center_pieces(&self) -> &bool {
        &self.center_pieces
    }

    pub fn light_square_color(&self) -> &String {
        &self.light_square_color
    }

    pub fn dark_square_color(&self) -> &String {
        &self.dark_square_color
    }

    pub fn pieces(&self) -> &HashMap<String, PieceRender> {
        &self.pieces
    }

    pub fn token(&self) -> &String {
        &self.token
    }

    pub fn piece_render(&self, kind: &PieceKind) -> Option<&PieceRender> {
        use PieceKind::*;

        let idx_str = match kind {
            Pawn => "pawn",
            Knight => "knight",
            Bishop => "bishop",
            Rook => "rook",
            Queen => "queen",
            King => "king",
        };

        self.pieces.get(idx_str.into())
    }

    pub fn legal_move_indicator_color(&self) -> &String {
        &self.legal_move_indicator_color
    }

    pub fn debug(&self) -> &bool {
        &self.debug
    }
}
