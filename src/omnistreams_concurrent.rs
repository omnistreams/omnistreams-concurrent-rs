const MESSAGE_TYPE_CREATE_RECEIVE_STREAM: u8 = 0;
const MESSAGE_TYPE_STREAM_DATA: u8 = 1;
const MESSAGE_TYPE_STREAM_END: u8 = 2;
const MESSAGE_TYPE_TERMINATE_SEND_STREAM: u8 = 3;
const MESSAGE_TYPE_STREAM_REQUEST_DATA: u8 = 4;
const MESSAGE_TYPE_CONTROL_MESSAGE: u8 = 5;

use std::collections::HashMap;
use std::rc::Rc;
use omnistreams_core::{Producer};

pub struct Multiplexer {
    send: Rc<Fn(&[u8])>,
    conduit_callback: Box<Fn(&mut ReceiveStream, &[u8])>,
    next_stream_id: u8,
    receive_streams: HashMap<u8, ReceiveStream>,
}

impl Multiplexer {
    pub fn new() -> Multiplexer {
        Multiplexer {
            send: Rc::new(|msg| {}),
            conduit_callback: Box::new(|mut producer, metadata| {}),
            next_stream_id: 0,
            receive_streams: HashMap::new(),
        }
    }

    pub fn set_send_handler<C: 'static + Fn(&[u8])>(&mut self, callback: C) {
        //self.send = Box::new(callback);
        self.send = Rc::new(callback);
    }

    pub fn handle_message(&mut self, msg: &[u8]) {

        let message_type = msg[0];
        let stream_id = msg[1];
        let data = &msg[2..];

        match message_type {
            MESSAGE_TYPE_CREATE_RECEIVE_STREAM => {
                println!("create stream {}: {:?}", stream_id, data);

                let send = self.send.clone();

                let request = move |num_items| {
                    println!("request called: {}", num_items);
                    send(&[MESSAGE_TYPE_STREAM_REQUEST_DATA, stream_id, num_items]);
                };

                let mut producer = ReceiveStream {
                    data_callback: Box::new(|msg| {}),
                    upstream_request: Box::new(request),
                };
                let stream_id = self.next_stream_id;
                self.next_stream_id += 1;
                (self.conduit_callback)(&mut producer, data);
                self.receive_streams.insert(stream_id, producer);
            },
            MESSAGE_TYPE_STREAM_DATA => {
                if let Some(producer) = self.receive_streams.get(&stream_id) {
                    (producer.data_callback)(data);
                }
            },
            MESSAGE_TYPE_STREAM_END => {
            },
            MESSAGE_TYPE_TERMINATE_SEND_STREAM=> {
            },
            MESSAGE_TYPE_STREAM_REQUEST_DATA => {
            },
            _ => {
            },
        }
    }

    pub fn on_conduit<C: 'static + Fn(&mut ReceiveStream, &[u8])>(&mut self, callback: C) {
        self.conduit_callback = Box::new(callback);
    }
}

pub struct ReceiveStream {
    data_callback: Box<Fn(&[u8])>,
    upstream_request: Box<Fn(u8)>,
}

impl ReceiveStream {
    pub fn on_data<C: 'static + Fn(&[u8])>(&mut self, callback: C) {
        self.data_callback = Box::new(callback);
    }

    pub fn request(&mut self, num_items: u8) {
        println!("requested: {}", num_items);
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
