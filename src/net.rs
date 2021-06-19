use websocket::{ClientBuilder, Message, OwnedMessage};
use std::{sync::{mpsc::{Sender, channel}}};
use crate::ThreadCallsModel;
use cursive::Cursive;
use crate::event;
use log::info;

#[derive(Debug)]
pub struct State {
    pub users: Vec<String>,
    pub channel: Sender<OwnedMessage>,
    pub send_thread: std::thread::JoinHandle<()>,
    pub receive_thread: std::thread::JoinHandle<()>,
}

#[allow(unused_must_use, unused_mut)]
pub fn connect(options: crate::ConnectionOptions, thr: ThreadCallsModel) -> websocket::WebSocketResult<State> {
    let addr = format!("ws://{}:9192", options.ip_address);
    let clientres = ClientBuilder::new(&addr)
            .unwrap()
            .connect_insecure();

    let client = match clientres {
        Ok(cl) => cl,
        Err(eror) => {
            return Err(eror);
        }
    };

    let (sx, rx) = channel::<OwnedMessage>();
    let (mut reader, mut writer) = client.split().unwrap();
        
    let writer_clone = sx.clone();
    let writer_thread = std::thread::spawn(move || {
        loop {
            let message = match rx.recv() {
                Ok(m ) => m,
                Err(err) => {
                    println!("{:?}", err);
                    return;
                }
            };
        
            let ss = writer.send_message(&message);
                
            match ss  {
                Ok(()) => (),
                Err(err) => {
                    println!("Send Loop: {:?}", err);
                    let _ = writer.send_message(&Message::close());
                    return;
                }
            }
        }
    });
        
    let reader_thread = std::thread::spawn(move || {
        for msg in reader.incoming_messages() {
            let message = match msg {
                Ok(m) => m,
                Err(err) => {
                    println!("{:?}", err);
                    return;
                }
            };

            info!("Received from WebSocket: {:?}", message);

            match message {
                OwnedMessage::Close(data) => {
                    writer_clone.send(OwnedMessage::Close(data));
                    return;
                }
        
                OwnedMessage::Text(content) => {
                    let ser: event::Payload = serde_json::from_str(content.as_str()).unwrap();
                    poll_event(ser, &thr);                    
                },
        
                _ => {
                    
                }
            }
        }
    });

    Ok(State {
        channel: sx,
        users: vec![],
        send_thread: writer_thread,
        receive_thread: reader_thread
    })
}

fn poll_event(data: event::Payload, model: &ThreadCallsModel) {
    let event = match &(&data).event {
        Some(cont) => cont,
        None => ""
    };

    match event {
        "MESSAGE_CREATE" => {
            let d = data.data.get("data").unwrap().as_object().unwrap();
            let msg = event::Message::new(
            String::from(d.get("content").unwrap().as_str().unwrap()),
            d.get("created_at").unwrap().as_i64().unwrap(),
            String::from(d.get("author").unwrap().as_str().unwrap())
            );

            let mut guard = model.lock().unwrap();
            guard.messages.push(msg);
            drop(guard);
        }

        _ => {

        }
    }

    let guard = model.lock().unwrap();
    guard.callback_sink.send(Box::new(Cursive::noop)).unwrap();
    drop(guard);
}