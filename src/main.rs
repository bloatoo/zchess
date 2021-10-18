use chess::{app::App, chess::Board, message::Message, ui};
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;

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

    let board = Board::default();
    let app = Arc::new(Mutex::new(App::default()));

    let (main_tx, main_rx) = mpsc::channel::<Message>();

    let stream_tx = main_tx.clone();

    let client = Client::new();

    let mut main_event_stream = client
        .get("https://lichess.org/api/stream/event")
        .header("Authorization", "Bearer abc")
        .header("Content-Type", "application/x-ndjson")
        .send()
        .await
        .unwrap()
        .bytes_stream();

    tokio::spawn(async move {
        loop {
            let ev = main_event_stream.next().await.unwrap().unwrap();
            let ev_string = String::from_utf8(ev.to_vec()).unwrap();

            if ev_string.len() > 1 {
                let json: Value =
                    serde_json::from_str(&ev_string).unwrap_or_else(|_| panic!("{}", ev_string));

                if let Some(p) = json.get("error") {
                    panic!("{}", p);
                }

                match json["type"].as_str().unwrap() {
                    "gameStart" => {}
                    _ => (),
                }
            }
        }
    });

    std::thread::spawn(move || {
        event_loop(main_rx);
    });

    ui::start(app, main_tx)?;

    Ok(())
}

#[tokio::main]
async fn event_loop(rx: Receiver<Message>) {
    while let Ok(_ev) = rx.recv() {}
}
