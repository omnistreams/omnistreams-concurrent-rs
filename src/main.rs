mod omnistreams_concurrent;

use omnistreams_core::{Producer};
use serde_json;
use serde_json::Value;
use ws::{listen, Handler, Sender, Result, Message, CloseCode};

use std::thread;
use std::sync::mpsc::{Sender as ChannelSender, Receiver as ChannelReceiver};
use std::sync::mpsc;

struct Server {
    out: Sender,
    tx: ChannelSender<Vec<u8>>,
}

impl Handler for Server {

    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Echo the message back
        self.out.send(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        // The WebSocket protocol allows for a utf8 reason for the closing state after the
        // close code. WS-RS will attempt to interpret this data as a utf8 description of the
        // reason for closing the connection. I many cases, `reason` will be an empty string.
        // So, you may not normally want to display `reason` to the user,
        // but let's assume that we know that `reason` is human-readable.
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}

fn main() {
    listen("127.0.0.1:9001", |out| {
    println!("Create mux");

    let (tx, rx): (ChannelSender<Vec<u8>>, ChannelReceiver<Vec<u8>>) = mpsc::channel();

    let sender = out.clone();

    thread::spawn(move || {
        let mut mux = omnistreams_concurrent::Multiplexer::new();

        mux.on_conduit(|mut producer, metadata| {
            producer.on_data(|data| {
                println!("got data: {:?}", data);
                //producer.request(1);
            });

            producer.request(10);
        });

        mux.set_send_handler(move |msg| {
            sender.send(msg).unwrap();
        });

        for msg in rx {
            mux.handle_message(&msg);
        }
    });

    move |msg| {

        match msg {
            Message::Text(_) => {
                println!("WARNING: text message received. unhandled");
            },
            Message::Binary(v) => {
                tx.send(v).unwrap();
            },
        }

        Ok(())
    }

  }).unwrap()
} 
/*
mod omnistreams_concurrent;

use omnistreams_core::{Producer};
//use std::thread;
//use std::sync::mpsc::{Sender, Receiver};
//use std::sync::mpsc;
use serde_json;
use serde_json::Value;
use ws::{listen, Handler, Message, Sender};

struct Server {
    out: Sender,
}

impl Handler for Server {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Echo the message back
        self.out.send(msg)
    }
}

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

        //let (message_sender, message_receiver): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
        //thread::spawn(move || {
        //    for data in message_receiver {
        //        mux.handle_message(&data);
        //    }
        //});

        move |msg| {

            match msg {
                Message::Text(_) => {
                    println!("WARNING: text message received. unhandled");
                },
                Message::Binary(v) => {
                    //message_sender.send(v).unwrap();
                },
            }

            Ok(())
        }
    }).unwrap()
}
*/
