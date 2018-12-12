const MESSAGE_TYPE_CREATE_RECEIVE_STREAM: u8 = 0;
const MESSAGE_TYPE_STREAM_DATA: u8 = 1;
const MESSAGE_TYPE_STREAM_END: u8 = 2;
const MESSAGE_TYPE_TERMINATE_SEND_STREAM: u8 = 3;
const MESSAGE_TYPE_STREAM_ACK: u8 = 4;

use serde_json;
use serde_json::Value;

pub struct Peer {
}

impl Peer {
    pub fn new() -> Peer {
        Peer {
        }
    }

    pub fn create_connection<T, U>(&self, send: T, on_stream: U) -> Connection<T, U> 
        where T: Fn(&[u8]), U: Fn(ReceiveStream<T>, Value)
    {
        Connection::new(send, on_stream)
    }
}


pub struct Connection<T, U>
    where T: Fn(&[u8]), U: Fn(ReceiveStream<T>, Value)
{
    send: T,
    on_stream: U,
    receive_streams: Vec<ReceiveStream<T>>,
}

impl<T, U> Connection<T, U>
    where T: Fn(&[u8]), U: Fn(ReceiveStream<T>, Value)
{
    pub fn new(send: T, on_stream: U) -> Connection<T, U> {
        Connection {
            send,
            on_stream,
            receive_streams: Vec::new(),
        }
    }

    pub fn handle_message(&self, msg: &[u8]) {

        let message_type = msg[0];
        let stream_id = msg[1];
        let data = &msg[2..];

        match message_type {
            MESSAGE_TYPE_CREATE_RECEIVE_STREAM => {
                let metadata: Value = serde_json::from_slice(data).unwrap();
                println!("create stream {}: {:?}", stream_id, metadata);
                let stream = ReceiveStream::new();
                //self.receive_streams.push(stream);
                (self.on_stream)(stream, metadata);
            },
            MESSAGE_TYPE_STREAM_DATA => {
                println!("data for stream {}: {:?}", stream_id, data);
            },
            MESSAGE_TYPE_STREAM_END => {
            },
            MESSAGE_TYPE_TERMINATE_SEND_STREAM=> {
            },
            MESSAGE_TYPE_STREAM_ACK => {
            },
            _ => {
            },
        }
        //(self.send)(msg);
    }
}

#[derive(Debug)]
pub struct ReceiveStream<T>
    where T: Fn(&[u8])
{
    on_data_callback: Option<T>,
}

impl<T> ReceiveStream<T>
    where T: Fn(&[u8])
{
    pub fn new() -> ReceiveStream<T> {
        ReceiveStream {
            on_data_callback: None,
        }
    }

    pub fn on_data(&mut self, callback: T) {
        self.on_data_callback = Some(callback);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
