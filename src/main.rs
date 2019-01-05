mod omnistreams_concurrent;

use omnistreams_core::{Producer};
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use serde_json;
use serde_json::Value;
use ws::{listen, Message};

fn main() {

    println!("Started");

    listen("127.0.0.1:9001", |ws| {

        println!("Create mux");

        let mut mux = omnistreams_concurrent::Multiplexer::new(move |msg| {
            ws.send(msg).unwrap();
        }, |stream, md| {

            let metadata: Value = serde_json::from_slice(md).unwrap();
            println!("{:?}", metadata);

            stream.on_data(|msg| {
                let message: Value = serde_json::from_slice(msg).unwrap();
                println!("{:?}", message);
            });

            stream.request(5);
        });

        let (message_sender, message_receiver): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
        thread::spawn(move || {
            for data in message_receiver {
                mux.handle_message(&data);
            }
        });

        move |msg| {

            match msg {
                Message::Text(_) => {
                    println!("WARNING: text message received. unhandled");
                },
                Message::Binary(v) => {
                    message_sender.send(v).unwrap();
                },
            }

            Ok(())
        }
    }).unwrap()
}
