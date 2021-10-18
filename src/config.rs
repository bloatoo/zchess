use serde::Deserialize;
use std::collections::HashMap;
use std::{env, fs};
use toml::de::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct Piece {
    render_black: String,
    render_white: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pieces: HashMap<String, Piece>,
    token: String,
}

impl Config {
    pub fn new() -> Result<Self, Error> {
        let home = env::var("HOME").unwrap();
        let path = format!("{}/.config/chess.toml", home);
        let data = fs::read_to_string(path).unwrap();
        toml::from_str(&data)
    }

    pub fn pieces(&self) -> &HashMap<String, Piece> {
        &self.pieces
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}
