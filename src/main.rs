mod game;
mod multiplayer;
mod messages;

use simple_websockets::{Event, Message};
use multiplayer::{RoomManager, Client};
use messages::{IncomingMessage};

const PORT: u16 = 12345;

fn main() {
    let hub = simple_websockets::launch(PORT).expect("Failed to start websocket server");
    println!("Listening on port {}", PORT);
    let mut rooms = RoomManager::new();

    loop {
        match hub.poll_event() {
            Event::Connect(client_id, responder) => {
                let client = Client::new(client_id, responder);
                rooms.add_client_to_new_room(client);
            },
            Event::Disconnect(client_id) => {
                rooms.remove_client(client_id);
            },
            Event::Message(client_id, message) => {
                match message {
                    Message::Text(s) => {
                        if let Some(msg) = IncomingMessage::parse(&s) {
                            rooms.handle_message(client_id, msg);
                        } else {
                            println!("Couldn't parse message from client {}:\n== start ==\n{}\n== end ==", client_id, s);
                        }
                    },
                    Message::Binary(_) => {
                        println!("Ignoring binary message from client {}", client_id);
                    },
                }
            },
        }
    }
}
