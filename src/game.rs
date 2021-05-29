use rand::{thread_rng, seq::SliceRandom};
use std::collections::HashSet;

#[derive(Debug)]
pub struct Square {
    is_mine: bool,
    revealed: bool,
    x: usize,
    y: usize,
}

impl Square {
    fn new(is_mine: bool, x: usize, y: usize) -> Self {
        Self {
            is_mine,
            revealed: false,
            x,
            y,
        }
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
    is_first_move: bool,
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
            is_first_move: true,
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

    /// Returns `None` if invalid coords
    pub fn get_square_mut(&mut self, x: usize, y: usize) -> Option<&mut Square> {
        return self.squares.get_mut(y)?.get_mut(x)
    }

    fn square_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize, bool)> {
        // yuck this isn't pretty
        let neighbor_positions = [
            if x > 0 && y > 0 { self.get_square(x - 1, y - 1) } else { None },
            if y > 0 { self.get_square(x, y - 1) } else { None },
            if x < usize::MAX && y > 0 { self.get_square(x + 1, y - 1) } else { None },
            if x > 0 { self.get_square(x - 1, y) } else { None },
            if x < usize::MAX { self.get_square(x + 1, y) } else { None },
            if x > 0 && y < usize::MAX { self.get_square(x - 1, y + 1) } else { None },
            if y < usize::MAX { self.get_square(x, y + 1) } else { None },
            if x < usize::MAX && y < usize::MAX { self.get_square(x + 1, y + 1) } else { None },
        ];
        let mut neighbors = Vec::with_capacity(8);
        for neighbor in neighbor_positions.iter() {
            if let Some(square) = neighbor {
                neighbors.push((square.x, square.y, square.is_mine));
            }
        }
        neighbors
    }

    /// Returns `None` if invalid coords
    pub fn recursive_square_reveal(&mut self, x: usize, y: usize) -> Option<Vec<(usize, usize, SquareContents)>> {
        //TODO: prevent possible DOS by stack overflow if client makes huge field with few mines
        // set a limit on field size to also prevent DOS by allocating tons of memory

        // remove mines on and surrounding the game's first revealed square to prevent loss on first move
        if self.is_first_move {
            self.is_first_move = false;
            let mut mines_to_move = Vec::new();
            let mut invalid_move_targets = HashSet::new();
            invalid_move_targets.insert((x, y));
            if self.get_square(x, y)?.is_mine {
                mines_to_move.push((x, y));
            }
            for neighbor in self.square_neighbors(x, y) {
                if neighbor.2 {
                    mines_to_move.push((neighbor.0, neighbor.1));
                }
                invalid_move_targets.insert((neighbor.0, neighbor.1));
            }

            let mut move_targets: Vec<_> = self.squares
                .iter()
                .flatten()
                .filter(|sq| !sq.is_mine)
                .filter(|sq| !invalid_move_targets.contains(&(sq.x, sq.y)))
                .map(|sq| (sq.x, sq.y))
                .collect();
            move_targets.shuffle(&mut thread_rng());

            for (mine_x, mine_y) in mines_to_move {
                if let Some((move_to_x, move_to_y)) = move_targets.pop() {
                    self.get_square_mut(move_to_x, move_to_y).unwrap().is_mine = true;
                    self.get_square_mut(mine_x, mine_y).unwrap().is_mine = false;
                } else {
                    // minefield is too dense to guarantee safe first move
                    break;
                }
            }
        }

        let square = self.get_square_mut(x, y)?;
        if square.revealed {
            return None;
        }
        square.revealed = true;
        let square = self.get_square(x, y)?;
        if square.is_mine {
            Some(
                vec![(x, y, SquareContents::MineBoom)]
            )
        } else {
            let neighbors = self.square_neighbors(x, y);
            let surr = neighbors.iter()
                .filter(|x| x.2)
                .count() as u8;
            let mut reveals = vec![ (x, y, SquareContents::NumMines(surr)) ];
            if surr == 0 {
                for (neighbor_x, neighbor_y, _) in neighbors {
                    if let Some(mut recursed) = self.recursive_square_reveal(neighbor_x, neighbor_y) {
                        reveals.append(&mut recursed);
                    }
                }
            }

            Some(reveals)
        }
    }
}
