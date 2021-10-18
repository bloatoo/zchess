use chess::{chess::Board, message::Message, ui};
use reqwest::Client;

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
    ui::start(board)?;

    let (main_tx, main_rx) = mpsc::channel::<Message>();

    let stream_tx = main_tx.clone();

    let client = Client::new();

    tokio::spawn(async move {
        let mut main_event_stream = client
            .get("https://lichess.org/api/stream/event")
            .header("Authorization", "Bearer {}")
            .header("Content-Type", "application/x-ndjson")
            .send()
            .await
            .unwrap()
            .bytes_stream();

        loop {
            /*let ev = main_event_stream.next().await;
            match ev {
                Some(r) => println!("some"),
                None => println!("none"),
            };*/
        }
    });

    std::thread::spawn(move || {
        event_loop(main_rx);
    });

    Ok(())
}

#[tokio::main]
async fn event_loop(rx: Receiver<Message>) {
    while let Ok(_ev) = rx.recv() {}
}
