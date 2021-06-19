use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

use cursive::traits::Scrollable;
use cursive::views::{Dialog, ListView, EditView, TextView, Canvas};
use cursive::traits::{Boxable, Nameable};
use serde_json::json;
use websocket::{OwnedMessage};

mod net;
mod event;

fn main() {
    cursive::logger::init();
    log::set_max_level(log::LevelFilter::Info);
    
    let mut cur = cursive::default();
    cur.set_autorefresh(true);

    cur.add_global_callback(cursive::event::Key::F12, |csv| {
        csv.toggle_debug_console();
    });

    cur.add_layer(Dialog::around(
        ListView::new()
            .child("Nome de Usuário", EditView::new().with_name("username").fixed_width(20))
            .child("IP do servidor", EditView::new().with_name("ip_address").fixed_width(20))
    ).button("Conectar", |csv| {
        let username = csv
        .call_on_name("username", |t: &mut EditView| t.get_content())
        .unwrap();

        let ip_address = csv
        .call_on_name("ip_address", |t: &mut EditView| t.get_content())
        .unwrap();
        
        let opt = ConnectionOptions {
            ip_address: ip_address.to_string(),
            username: username.to_string()
        };

        connection_step(csv, opt);
    }));
    cur.run();
}

#[derive(Clone)]
pub struct ConnectionOptions {
    pub ip_address: String,
    pub username: String
}

pub struct ThreadCalls {
    pub(crate) callback_sink: cursive::CbSink,
    pub messages: Vec<event::Message>
}

pub type ThreadCallsModel = Arc<Mutex<ThreadCalls>>;

fn connection_step(csv: &mut cursive::Cursive, options: ConnectionOptions) {
    let model = Arc::new(Mutex::new(ThreadCalls {
        callback_sink: csv.cb_sink().clone(),
        messages: vec![]
    }));

    let stateres = net::connect(options.clone(), Arc::clone(&model));
    let state = match stateres {
        Ok(stat) => stat,
        Err(err) => {
            csv.add_layer(Dialog::info(err.to_string()));
            return;
        }
    };

    csv.pop_layer();

    let fmt = format!("Conectando como {}", options.username);
    csv.add_layer(Dialog::around(TextView::new(fmt)));

    chat_step(csv, options, state, model);
}

fn chat_step(csv: &mut cursive::Cursive, options: ConnectionOptions, state: net::State, 
        model: ThreadCallsModel) {

    let id = json!({
        "operation": "identity",
        "data": {
            "username": options.username
        }
     });
    csv.pop_layer();
    let _ = state.channel.send(OwnedMessage::Text(id.to_string()));

    csv.add_layer(
        Dialog::around(
            ListView::new()
                    .child("", EditView::new().with_name("inputbox").fixed_width(30))
                    .child("Mensagens", create_canvas(model))
            ).button("Enviar", move |cs| {
                let content = cs
                    .call_on_name("inputbox", |txt: &mut EditView| txt.get_content())
                    .unwrap();

                if content != Rc::new(String::from("")) {
                    let payl = json!({
                        "operation": "message_add",
                        "data": {
                            "content": content.as_str()
                        }
                    });
                    let _ = state.channel.send(OwnedMessage::Text(payl.to_string()));
                }
            }).title("Chat")
        );
}

fn create_canvas(model: ThreadCallsModel) -> impl cursive::view::View {
    Canvas::new(model)
        .with_draw(|mo, printer| {
            let guard = mo.lock().unwrap();
            for (index, message) in guard.messages.iter().enumerate() {
                printer.print(
                    (0, index),
                    &format!("<{}> diz: {}", message.get_author(), message.get_content())
                );
            }
            
    }).with_required_size(|model, _req| {
        let model = model.lock().unwrap();
        let ddd = model.messages.len();
        cursive::Vec2::new(40, ddd)
    }).scrollable()
}
