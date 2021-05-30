use std::collections::{HashMap, hash_map::RandomState};
use std::hash::{BuildHasher, Hasher};
use simple_websockets::{Responder, Message};
use crate::game::{Minefield, SquareContents};
use crate::messages::{OutgoingMessage, IncomingMessage};

pub type RoomId = u32;
pub type ClientId = u64;

#[derive(Debug)]
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

#[derive(Debug)]
struct GameRoom {
    clients: HashMap<ClientId, Client>,
    field: Minefield,
    is_game_over: bool,
    room_id: RoomId,
    room_code: String,
}

impl GameRoom {
    fn new(field: Minefield, room_id: RoomId, room_code: String) -> Self {
        Self {
            clients: HashMap::new(),
            field,
            is_game_over: false,
            room_id,
            room_code,
        }
    }

    fn add_client(&mut self, client: Client) {
        client.responder.send(Message::Text(
            OutgoingMessage::NewGame(self.field.width(), self.field.height()).encode()
        ));
        client.responder.send(Message::Text(
            OutgoingMessage::RoomCode(&self.room_code).encode()
        ));
        for square in self.field.all_squares() {
            if square.revealed() {
                let num_mines = self.field.square_neighbors(square.x(), square.y())
                    .iter()
                    .filter(|(_, _, is_mine)| *is_mine)
                    .count() as u8;
                client.responder.send(Message::Text(
                    OutgoingMessage::Reveal(square.x(), square.y(), SquareContents::NumMines(num_mines)).encode()
                ));
            }

            if square.flagged() {
                client.responder.send(Message::Text(
                    OutgoingMessage::Flag(square.x(), square.y(), true).encode()
                ));
            }
        }
        self.clients.insert(client.id, client);
    }

    fn remove_client(&mut self, client_id: ClientId) -> Option<Client> {
        self.clients.remove(&client_id)
    }

    fn is_empty(&self) -> bool {
        self.clients.is_empty()
    }

    pub fn broadcast_message(&self, message: OutgoingMessage) {
        for client in self.clients.values() {
            client.responder.send(Message::Text(message.encode()));
        }
    }

    /// Reveals square to all clients
    fn reveal_square(&mut self, x: usize, y: usize) {
        if self.is_game_over {
            return;
        }

        if let Some(squares) = self.field.recursive_square_reveal(x, y) {
            for (x, y, contents) in squares {
                if let SquareContents::MineBoom = contents {
                    self.is_game_over = true;
                }
                self.broadcast_message(
                    OutgoingMessage::Reveal(x, y, contents)
                );
            }
        }
    }
}

#[derive(Debug)]
pub struct RoomManager {
    rooms: HashMap<RoomId, GameRoom>,
    next_room_id: RoomId,
    /// Maps a client to the room they're in
    client_map: HashMap<ClientId, RoomId>,
    random_state: RandomState,
    code_map: HashMap<String, RoomId>,
}

impl RoomManager {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            next_room_id: 0,
            client_map: HashMap::new(),
            random_state: RandomState::new(),
            code_map: HashMap::new(),
        }
    }

    fn gen_room_id(&mut self) -> RoomId {
        let id = self.next_room_id;
        self.next_room_id = self.next_room_id.wrapping_add(1);
        id
    }

    fn room_id_to_code(&self, id: RoomId) -> String {
        let mut hasher = self.random_state.build_hasher();
        hasher.write_u32(id);

        let mut s = format!("{:X}", hasher.finish())
            .replace("0", "G")
            .replace("1", "H")
            .replace("2", "Q")
            .replace("3", "J")
            .replace("4", "K")
            .replace("5", "L")
            .replace("6", "M")
            .replace("7", "N")
            .replace("8", "O")
            .replace("9", "P");
        s.truncate(6);

        s
    }

    pub fn add_client_to_new_room(&mut self, client: Client) {
        let roomid = self.gen_room_id();
        let room = GameRoom::new(Minefield::default_field(), roomid, self.room_id_to_code(roomid));
        self.code_map.insert(self.room_id_to_code(roomid), roomid);
        self.rooms.insert(roomid, room);
        self.add_client_to_room(client, roomid);
    }

    fn add_client_to_room(&mut self, client: Client, room_id: RoomId) {
        self.client_map.insert(client.id, room_id);
        self.rooms.get_mut(&room_id).unwrap().add_client(client);
    }

    pub fn remove_client(&mut self, client_id: ClientId) -> Option<Client> {
        if let Some(roomid) = self.client_map.remove(&client_id) {
            let room = self.rooms.get_mut(&roomid).expect("Invalid RoomId in client_map");

            let removed = room.remove_client(client_id).expect("Client was not in room given by client_map");
            if room.is_empty() {
                self.rooms.remove(&roomid);
                self.code_map.remove(&self.room_id_to_code(roomid));
            }
            Some(removed)
        } else {
            None
        }
    }

    pub fn handle_message(&mut self, client_id: ClientId, message: IncomingMessage) {
        let client_roomid = *self.client_map.get(&client_id).expect("Client is not in a room");
        let room = self.rooms.get_mut(&client_roomid).expect("Invalid RoomId in client_map");
        match message {
            IncomingMessage::Reveal(x, y) => room.reveal_square(x, y),
            IncomingMessage::JoinRoom(room_code) => {
                if let Some(&room_id) = self.code_map.get(&room_code) {
                    if *self.client_map.get(&client_id).unwrap() != room_id {
                        let client = self.remove_client(client_id).unwrap();
                        self.add_client_to_room(client, room_id);
                    }
                }
            },
            IncomingMessage::Flag(x, y, on) => {
                if x < room.field.width() && y < room.field.height() {
                    room.field.get_square_mut(x, y).unwrap().set_flagged(on);
                    room.broadcast_message(
                        OutgoingMessage::Flag(x, y, on)
                    );
                }
            },
        }
    }
}
