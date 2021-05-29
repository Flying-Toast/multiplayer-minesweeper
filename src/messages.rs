#[derive(Debug)]
pub enum OutgoingMessage {
    /// (width, height)
    NewGame(usize, usize),
}

impl OutgoingMessage {
    // encodes to json
    pub fn encode(&self) -> String {
        match self {
            Self::NewGame(width, height) => {
                format!(r#"{{"t":"newgame","width":{},"height":{}}}"#, width, height)
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
