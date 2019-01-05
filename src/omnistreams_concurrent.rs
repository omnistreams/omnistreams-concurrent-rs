const MESSAGE_TYPE_CREATE_RECEIVE_STREAM: u8 = 0;
const MESSAGE_TYPE_STREAM_DATA: u8 = 1;
const MESSAGE_TYPE_STREAM_END: u8 = 2;
const MESSAGE_TYPE_TERMINATE_SEND_STREAM: u8 = 3;
const MESSAGE_TYPE_STREAM_ACK: u8 = 4;

use serde_json;
use serde_json::Value;


pub struct Multiplexer<T, U, V>
    where T: Fn(&[u8]), U: Fn(&[u8]), V: Fn(&mut ReceiveStream<U>, Value)
{
    send: T,
    receive_streams: Vec<ReceiveStream<U>>,
    on_stream: V,
}

impl<T, U, V> Multiplexer<T, U, V>
    where T: Fn(&[u8]), U: Fn(&[u8]), V: Fn(&mut ReceiveStream<U>, Value)
{
    pub fn new(send: T, on_stream: V) -> Multiplexer<T, U, V> {
        Multiplexer {
            send,
            on_stream,
            receive_streams: Vec::new(),
        }
    }

    pub fn handle_message(&mut self, msg: &[u8]) {

        let message_type = msg[0];
        let stream_id = msg[1];
        let data = &msg[2..];

        match message_type {
            MESSAGE_TYPE_CREATE_RECEIVE_STREAM => {
                let metadata: Value = serde_json::from_slice(data).unwrap();
                println!("create stream {}: {:?}", stream_id, metadata);
                let mut stream = ReceiveStream::new();
                self.receive_streams.push(stream);
                //(self.on_stream)(&mut stream, metadata);
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
pub struct ReceiveStream<V>
    where V: Fn(&[u8])
{
    on_data_callback: Option<V>,
}

impl<V> ReceiveStream<V>
    where V: Fn(&[u8])
{
    pub fn new() -> ReceiveStream<V> {
        ReceiveStream {
            on_data_callback: None,
        }
    }

    pub fn on_data(&mut self, callback: V) {
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
