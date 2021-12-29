use chess::{app::App, message::Message, ui, utils::debug};
use reqwest::Client;
use serde_json::Value;

use std::sync::Arc;
use tokio::sync::Mutex;

use futures::stream::StreamExt;
use std::sync::mpsc::{self, Receiver};

use std::panic::PanicInfo;

use crossterm::{
    execute,
    style::Print,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
};

fn panic_hook(info: &PanicInfo<'_>) {
    let location = info.location().unwrap();

    let message = match info.payload().downcast_ref::<&'static str>() {
        Some(msg) => *msg,
        None => match info.payload().downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Box<Any>",
        },
    };

    disable_raw_mode().unwrap();

    execute!(
        std::io::stdout(),
        LeaveAlternateScreen,
        Print(format!(
            "thread <unnamed> panicked at '{}', {}\n",
            message, location
        )),
    )
    .unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::panic::set_hook(Box::new(|info| panic_hook(info)));

    let (main_tx, main_rx) = mpsc::channel::<Message>();
    let stream_tx = main_tx.clone();

    let app = App::new(main_tx.clone()).await.unwrap();
    let debug_enabled = *app.config().debug();

    let token = format!("Bearer {}", app.config().token());
    let token_clone = token.clone();

    tokio::spawn(async move {
        let client = Client::new();

        let mut main_event_stream = client
            .get("https://lichess.org/api/stream/event")
            .header("Authorization", token)
            .header("Content-Type", "application/x-ndjson")
            .send()
            .await
            .unwrap()
            .bytes_stream();

        loop {
            let ev = main_event_stream.next().await.unwrap().unwrap();
            let ev_string = String::from_utf8(ev.to_vec()).unwrap();

            if ev_string.len() > 1 {
                if debug_enabled {
                    debug(&format!("main_event_stream: {}", ev_string));
                }

                let json: Value =
                    serde_json::from_str(&ev_string).unwrap_or_else(|_| panic!("{}", ev_string));

                if let Some(p) = json.get("error") {
                    panic!("{}", p);
                }

                match json["type"].as_str().unwrap() {
                    "gameStart" => {
                        let game_id = json["game"]["id"].as_str().unwrap();

                        stream_tx.send(Message::GameStart(game_id.into())).unwrap();
                    }

                    "gameEnd" => {
                        stream_tx.send(Message::GameEnd).unwrap();
                    }
                    _ => (),
                }
            }
        }
    });

    let own_info_tx = main_tx.clone();

    tokio::spawn(async move {
        let client = reqwest::Client::new();

        let res = client
            .get("https://lichess.org/api/account")
            .header("Authorization", token_clone)
            .send()
            .await;

        if let Ok(res) = res {
            let text = res.text().await;

            if let Ok(text) = text {
                let info = serde_json::from_str(&text).unwrap();

                if debug_enabled {
                    debug(&format!("own_info: {}", text));
                }

                own_info_tx.send(Message::GetOwnInfo(info)).unwrap();
            }
        }
    });

    let app = Arc::new(Mutex::new(app));
    let app_clone = app.clone();

    std::thread::spawn(move || {
        event_loop(main_rx, app_clone);
    });

    ui::start(app, main_tx).await?;

    Ok(())
}

#[tokio::main]
async fn event_loop(rx: Receiver<Message>, app: Arc<Mutex<App>>) {
    while let Ok(ev) = rx.recv() {
        let mut app = app.lock().await;

        match ev {
            Message::GameStart(id) => {
                app.init_new_game(id);
                app.state_changed = true;
            }

            Message::GameDataInit(game) => {
                app.start_game(game);
                app.state_changed = true;
            }

            Message::GameStateUpdate(state) => {
                app.update_game_state(state);
                app.state_changed = true;
            }

            Message::NewMessage(msg) => {
                let game = app.game_mut().as_mut().unwrap();
                game.new_message(msg);
            }
            Message::GetOwnInfo(info) => {
                app.set_own_info(info);
            }
            Message::GameEnd => {
                app.end_game();
            }
        }
    }
}
