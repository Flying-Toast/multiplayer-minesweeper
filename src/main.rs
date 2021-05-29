mod game;
mod multiplayer;

use simple_websockets::Event;
use multiplayer::{RoomManager, Client};

fn main() {
    let hub = simple_websockets::launch(12345).expect("Failed to start websocket server");
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
            },
        }
    }
}
