use crate::game::{ChatMessage, Game, GameState};
use crate::user::User;

pub enum Message {
    GameStart(String), // id
    GameStateUpdate(GameState),
    GameDataInit(Game),
    NewMessage(ChatMessage),
    GetOwnInfo(User),
    GameEnd,
}
