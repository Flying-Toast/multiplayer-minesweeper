use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)]
pub struct Square {
    is_mine: bool,
    x: usize,
    y: usize,
}

impl Square {
    fn new(is_mine: bool, x: usize, y: usize) -> Self {
        Self { is_mine, x, y, }
    }
}

#[derive(Debug)]
pub enum SquareContents {
    NumMines(u8),
    /// revealed square was a mine. Boom!
    MineBoom,
}

#[derive(Debug)]
pub struct Minefield {
    squares: Vec<Vec<Square>>,
    width: usize,
    height: usize,
}

impl Minefield {
    /// Creates a randomized minefield
    fn new(width: usize, height: usize, num_mines: usize) -> Option<Self> {
        let area = width * height;
        if num_mines > area || width == 0 || height == 0 {
            return None;
        }

        let mut all_squares = Vec::with_capacity(area);
        for _ in 0..num_mines {
            all_squares.push(Square::new(true, 0, 0));
        }
        for _ in 0..(area - num_mines) {
            all_squares.push(Square::new(false, 0, 0));
        }
        all_squares.shuffle(&mut thread_rng());

        let mut squares = Vec::with_capacity(height);
        for y in 0..height {
            let mut row = Vec::with_capacity(width);
            for x in 0..width {
                let mut square = all_squares.pop().unwrap();
                square.x = x;
                square.y = y;
                row.push(square);
            }
            squares.push(row);
        }

        Some(Self {
            width,
            height,
            squares,
        })
    }

    pub fn default_field() -> Self {
        Self::new(30, 16, 99).unwrap()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns `None` if invalid coords
    pub fn get_square(&self, x: usize, y: usize) -> Option<&Square> {
        return self.squares.get(y)?.get(x)
    }

    fn square_neighbors(&self, x: usize, y: usize) -> Vec<&Square> {
        let neighbor_positions = vec![
            self.get_square(x - 1, y - 1), self.get_square(x, y - 1), self.get_square(x + 1, y - 1),
            self.get_square(x - 1, y),                                self.get_square(x + 1, y),
            self.get_square(x - 1, y + 1), self.get_square(x, y + 1), self.get_square(x + 1, y + 1),
        ];
        let mut neighbors = Vec::with_capacity(8);
        for neighbor in neighbor_positions {
            if let Some(square) = neighbor {
                neighbors.push(square);
            }
        }
        neighbors
    }

    /// Returns `None` if invalid coords
    pub fn recursive_square_reveal(&self, x: usize, y: usize) -> Option<Vec<(usize, usize, SquareContents)>> {
        //TODO: prevent possible DOS by stack overflow if client makes huge field with few mines
        //TODO: recurse
        let square = self.get_square(x, y)?;
        if square.is_mine {
            Some(
                vec![(x, y, SquareContents::MineBoom)]
            )
        } else {
            Some(vec![
                (
                    x,
                    y,
                    SquareContents::NumMines(self.square_neighbors(x, y).iter().filter(|sq| sq.is_mine).count() as u8),
                )

            ])
        }
    }
}
