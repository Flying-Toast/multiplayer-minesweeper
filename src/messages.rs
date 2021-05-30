use crate::game::SquareContents;

impl SquareContents {
    fn encode(&self) -> &'static str {
        match self {
            Self::NumMines(n) => {
                match n {
                    0 => "0",
                    1 => "1",
                    2 => "2",
                    3 => "3",
                    4 => "4",
                    5 => "5",
                    6 => "6",
                    7 => "7",
                    8 => "8",
                    _ => panic!("Invalid number of surrounding mines"),
                }
            },
            Self::MineBoom => "!",
        }
    }
}

#[derive(Debug)]
pub enum OutgoingMessage<'a> {
    /// (width, height, num_mines)
    NewGame(usize, usize, usize),
    /// Server is revealing square (x, y)
    Reveal(usize, usize, SquareContents),
    /// Tells the client their room id
    RoomCode(&'a str),
    /// Another player flagged/unflagged a mine
    Flag(usize, usize, bool),
    BadBoardParams,
    BadRoomCode,
}

impl OutgoingMessage<'_> {
    // encodes to json
    pub fn encode(&self) -> String {
        match self {
            Self::NewGame(width, height, mines) => {
                format!(r#"{{"t":"newgame","width":{},"height":{},"mines":{}}}"#, width, height, mines)
            },
            Self::Reveal(x, y, contents) => {
                format!(r#"{{"t":"reveal","x":{},"y":{},"content":"{}"}}"#, x, y, contents.encode())
            },
            Self::RoomCode(roomcode) => {
                format!(r#"{{"t":"room","id":"{}"}}"#, roomcode)
            },
            Self::Flag(x, y, on) => {
                format!(r#"{{"t":"flag","x":{},"y":{},"on":{}}}"#, x, y, on)
            },
            Self::BadBoardParams => {
                format!(r#"{{"t":"badboard"}}"#)
            },
            Self::BadRoomCode => {
                format!(r#"{{"t":"badcode"}}"#)
            },
        }
    }
}

#[derive(Debug)]
pub enum IncomingMessage {
    /// Client wants to reveal square (x, y)
    Reveal(usize, usize),
    JoinRoom(String),
    Flag(usize, usize, bool),
    /// Client wants to start a new game
    NewGame(usize, usize, usize),
}

impl IncomingMessage {
    pub fn parse(s: &str) -> Option<Self> {
        let mut lines = s.lines();
        match lines.next()? {
            "reveal" => {
                let x: usize = lines.next()?.parse().ok()?;
                let y: usize = lines.next()?.parse().ok()?;

                Some(Self::Reveal(x, y))
            },
            "join" => {
                Some(Self::JoinRoom(
                    lines.next()?.parse().ok()?,
                ))
            },
            "flag" => {
                Some(Self::Flag(
                    lines.next()?.parse().ok()?,
                    lines.next()?.parse().ok()?,
                    lines.next()?.parse().ok()?,
                ))
            },
            "newgame" => {
                Some(Self::NewGame(
                    lines.next()?.parse().ok()?,
                    lines.next()?.parse().ok()?,
                    lines.next()?.parse().ok()?,
                ))
            },
            _ => None,
        }
    }
}
