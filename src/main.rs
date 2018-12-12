mod netstreams;

use ws::{listen, Message};

fn main() {
    let peer = netstreams::Peer::new();

    listen("127.0.0.1:9001", |ws| {

        println!("Create connection");

        let conn = peer.create_connection(move |msg| {
            ws.send(msg).unwrap();
        }, |stream, metadata| {
            //println!("handle stream: {:?}, {:?}", stream, metadata);

            //stream.on_data(|| {
            //});
        });

        move |msg| {

            match msg {
                Message::Text(_) => {
                    println!("WARNING: text message received. unhandled");
                },
                Message::Binary(v) => {
                    conn.handle_message(&v);
                },
            }

            Ok(())
        }
    }).unwrap()
}
