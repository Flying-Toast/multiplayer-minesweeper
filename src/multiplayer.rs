use std::collections::HashMap;
use simple_websockets::{Responder, Message};
use crate::game::{Minefield};

type RoomId = u32;
type ClientId = u64;

enum OutgoingMessage {
    /// (width, height)
    NewGame(usize, usize),
}

impl OutgoingMessage {
    // encodes to json
    fn encode(&self) -> String {
        match self {
            Self::NewGame(width, height) => {
                format!(r#"{{"t":"newgame","width":{},"height":{}}}"#, width, height)
            },
        }
    }
}

pub struct Client {
    responder: Responder,
    id: ClientId,
}

impl Client {
    pub fn new(id: ClientId, responder: Responder) -> Self {
        Self {
            responder,
            id,
        }
    }
}

struct GameRoom {
    clients: HashMap<ClientId, Client>,
    field: Minefield,
}

impl GameRoom {
    fn new(field: Minefield) -> Self {
        Self {
            clients: HashMap::new(),
            field,
        }
    }

    fn add_client(&mut self, client: Client) {
        client.responder.send(Message::Text(
            OutgoingMessage::NewGame(self.field.width(), self.field.height()).encode()
        ));
        //TODO: broadcast new client to room (if other clients are in room)
        self.clients.insert(client.id, client);
    }

    fn remove_client(&mut self, client_id: ClientId) -> Option<Client> {
        self.clients.remove(&client_id)
        //TODO: broadcast removal
    }

    fn is_empty(&self) -> bool {
        self.clients.is_empty()
    }
}

pub struct RoomManager {
    rooms: HashMap<RoomId, GameRoom>,
    next_room_id: RoomId,
    /// Maps a client to the room they're in
    client_map: HashMap<ClientId, RoomId>,
}

impl RoomManager {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            next_room_id: 0,
            client_map: HashMap::new(),
        }
    }

    fn gen_room_id(&mut self) -> RoomId {
        let id = self.next_room_id;
        self.next_room_id = self.next_room_id.wrapping_add(1);
        id
    }

    pub fn add_client_to_new_room(&mut self, client: Client) {
        let roomid = self.gen_room_id();
        let mut room = GameRoom::new(Minefield::default_field());
        self.client_map.insert(client.id, roomid);
        room.add_client(client);
        self.rooms.insert(roomid, room);
    }

    pub fn remove_client(&mut self, client_id: ClientId) -> Option<Client> {
        if let Some(roomid) = self.client_map.remove(&client_id) {
            let room = self.rooms.get_mut(&roomid).expect("Invalid RoomId in client_map");

            let removed = room.remove_client(client_id).expect("Client was not in room given by client_map");
            if room.is_empty() {
                self.rooms.remove(&roomid);
            }
            Some(removed)
        } else {
            None
        }
    }
}
