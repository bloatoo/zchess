use crate::game::Game;

pub enum Message {
    GameStart(String), // id
    GameDataInit(Game),
}
