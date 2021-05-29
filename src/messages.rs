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
pub enum OutgoingMessage {
    /// (width, height)
    NewGame(usize, usize),
    /// Server is revealing square (x, y)
    Reveal(usize, usize, SquareContents),
}

impl OutgoingMessage {
    // encodes to json
    pub fn encode(&self) -> String {
        match self {
            Self::NewGame(width, height) => {
                format!(r#"{{"t":"newgame","width":{},"height":{}}}"#, width, height)
            },
            Self::Reveal(x, y, contents) => {
                format!(r#"{{"t":"reveal","x":{},"y":{},"content":"{}"}}"#, x, y, contents.encode())
            },
        }
    }
}

#[derive(Debug)]
pub enum IncomingMessage {
    /// Client wants to reveal square (x, y)
    Reveal(usize, usize),
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
            _ => None,
        }
    }
}
