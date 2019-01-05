mod omnistreams_concurrent;

use ws::{listen, Message};

fn main() {

    listen("127.0.0.1:9001", |ws| {

        println!("Create mux");

        let mut mux = omnistreams_concurrent::Multiplexer::new(move |msg| {
            ws.send(msg).unwrap();
        }, |stream, _metadata| {
            //println!("handle stream: {:?}, {:?}", stream, metadata);

            stream.on_data(|_msg| {
            });
        });

        mux.handle_message(&[1,2,3]);

        move |msg| {

            match msg {
                Message::Text(_) => {
                    println!("WARNING: text message received. unhandled");
                },
                Message::Binary(v) => {
                    //mux.handle_message(&v);
                },
            }

            Ok(())
        }
    }).unwrap()
}
