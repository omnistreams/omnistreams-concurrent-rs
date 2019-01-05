const MESSAGE_TYPE_CREATE_RECEIVE_STREAM: u8 = 0;
const MESSAGE_TYPE_STREAM_DATA: u8 = 1;
const MESSAGE_TYPE_STREAM_END: u8 = 2;
const MESSAGE_TYPE_TERMINATE_SEND_STREAM: u8 = 3;
const MESSAGE_TYPE_STREAM_ACK: u8 = 4;

use std::collections::HashMap;
use omnistreams_core::{Producer};


pub struct Multiplexer<T, U, V>
    where T: Fn(&[u8]), U: Fn(&[u8]), V: Fn(&mut ReceiveStream<U>, &[u8])
{
    send: T,
    receive_streams: Vec<ReceiveStream<U>>,
    on_stream: V,
    next_stream_id: u8,
}

impl<T, U, V> Multiplexer<T, U, V>
    where T: Fn(&[u8]), U: Fn(&[u8]), V: Fn(&mut ReceiveStream<U>, &[u8])
{
    pub fn new(send: T, on_stream: V) -> Multiplexer<T, U, V> {
        Multiplexer {
            send,
            on_stream,
            receive_streams: Vec::new(),
            next_stream_id: 0,
        }
    }

    pub fn handle_message(&mut self, msg: &[u8]) {

        let message_type = msg[0];
        let stream_id = msg[1];
        let data = &msg[2..];

        match message_type {
            MESSAGE_TYPE_CREATE_RECEIVE_STREAM => {
                println!("create stream {}: {:?}", stream_id, data);
                let request = |num_items| {
                    println!("request called: {}", num_items);
                };

                let mut stream = ReceiveStream::new(request);
                let stream_id = self.next_stream_id;
                self.next_stream_id += 1;
                (self.on_stream)(&mut stream, data);
                self.receive_streams.push(stream);
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
    data_callback: Option<V>,
    upstream_request: fn(u8),
}

impl<V> ReceiveStream<V>
    where V: Fn(&[u8])
{
    pub fn new(upstream_request: fn(u8)) -> ReceiveStream<V> {

        ReceiveStream {
            data_callback: None,
            upstream_request,
        }
    }
}

impl<V> Producer<V> for ReceiveStream<V>
    where V: Fn(&[u8])
{
    fn on_data(&mut self, callback: V) {
        self.data_callback = Some(callback);
    }

    fn request(&self, num_items: u8) {
        (self.upstream_request)(num_items);
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
