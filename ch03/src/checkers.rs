//! Rusty Checkers module
#![forbid(unsafe_code)]

#[derive(Debug, Default)]
pub(crate) struct Engine {
    pub(crate) current_turn: PieceColor,
    pub(crate) move_count: u32,
    board: [[Option<Piece>; 8]; 8],
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct Move {
    from: Coordinate,
    to: Coordinate,
}

impl Move {
    pub(crate) fn new(from: (u8, u8), to: (u8, u8)) -> Self {
        Self {
            from: Coordinate(from.0, from.1),
            to: Coordinate(to.0, to.1),
        }
    }
}

impl Engine {
    pub(crate) fn new() -> Self {
        Self::default().initialize()
    }

    pub(crate) fn move_piece(&mut self, mv: Move) -> Option<bool> {
        let legal_moves = self.legal_moves();
        if !legal_moves.contains(&mv) {
            return None;
        }

        let crowned = self.should_crowned(mv.to);
        if let Some(mut piece) = self.piece_mut(mv.from).take() {
            if crowned {
                piece.crowned();
            }
            self.board[mv.to.0 as usize][mv.to.1 as usize] = Some(piece);
        } else {
            return None;
        }
        self.advance_turn();
        Some(crowned)
    }

    pub(crate) fn piece(&self, loc: Coordinate) -> Option<&Piece> {
        if loc.0 < 8 && loc.1 < 8 {
            self.board[loc.0 as usize][loc.1 as usize].as_ref()
        } else {
            None
        }
    }

    fn piece_mut(&mut self, loc: Coordinate) -> &mut Option<Piece> {
        &mut self.board[loc.0 as usize][loc.1 as usize]
    }

    fn should_crowned(&self, to: Coordinate) -> bool {
        match self.current_turn {
            PieceColor::White => to.1 == 7,
            PieceColor::Black => to.1 == 0,
        }
    }

    fn advance_turn(&mut self) {
        self.current_turn = match self.current_turn {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        };
    }

    fn legal_moves(&self) -> Vec<Move> {
        let mut moves = vec![];
        for x in 0..8 {
            for y in 0..8 {
                let loc = Coordinate(x as u8, y as u8);
                if let Some(piece) = self.piece(loc) {
                    if piece.color == self.current_turn {
                        moves.extend(self.valid_moves(loc));
                    }
                }
            }
        }
        moves
    }

    fn valid_moves(&self, from: Coordinate) -> impl Iterator<Item = Move> {
        if let Some(piece) = self.piece(from) {
            let mut valid_moves = from
                .jump_targets_iter()
                .filter(|&to| self.valid_jump(piece, from, to))
                .map(|to| Move { from, to })
                .collect::<Vec<_>>();
            let moves = from
                .move_targets_iter()
                .filter(|&to| self.valid_move(to))
                .map(|to| Move { from, to });
            valid_moves.extend(moves);
            valid_moves.into_iter()
        } else {
            vec![].into_iter()
        }
    }

    fn valid_jump(&self, _piece: &Piece, _from: Coordinate, to: Coordinate) -> bool {
        !self.piece(to).is_some()
    }

    fn valid_move(&self, to: Coordinate) -> bool {
        !self.piece(to).is_some()
    }

    fn initialize(mut self) -> Self {
        [1, 3, 5, 7, 0, 2, 4, 6, 1, 3, 5, 7]
            .into_iter()
            .zip([0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2].into_iter())
            .for_each(|(x, y)| {
                self.board[x][y] = Some(Piece::new(PieceColor::White));
            });
        [0, 2, 4, 6, 1, 3, 5, 7, 0, 2, 4, 6]
            .into_iter()
            .zip([5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7].into_iter())
            .for_each(|(x, y)| {
                self.board[x][y] = Some(Piece::new(PieceColor::Black));
            });
        self.current_turn = PieceColor::Black;
        self
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

impl Default for PieceColor {
    fn default() -> Self {
        PieceColor::Black
    }
}

impl From<PieceColor> for i32 {
    fn from(color: PieceColor) -> i32 {
        match color {
            PieceColor::White => 2,
            PieceColor::Black => 1,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Piece {
    color: PieceColor,
    crowned: bool,
}

impl From<&Piece> for i32 {
    fn from(piece: &Piece) -> i32 {
        let mut result = piece.color.into();
        if piece.crowned {
            result += 4;
        }
        result
    }
}

impl Piece {
    fn new(color: PieceColor) -> Self {
        Self {
            color,
            crowned: false,
        }
    }

    fn crowned(&mut self) {
        self.crowned = true;
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct Coordinate(u8, u8);

impl Coordinate {
    pub(crate) fn new(x: u8, y: u8) -> Self {
        Self(x, y)
    }

    fn jump_targets_iter(&self) -> impl Iterator<Item = Self> {
        let Self(x, y) = *self;

        let mut jumps = vec![];
        if y >= 2 {
            jumps.push(Self(x + 2, y - 2));
        }
        jumps.push(Self(x + 2, y + 2));
        if x >= 2 && y >= 2 {
            jumps.push(Self(x - 2, y - 2));
        }
        if x >= 2 {
            jumps.push(Self(x - 2, y + 2));
        }
        jumps.into_iter()
    }

    fn move_targets_iter(&self) -> impl Iterator<Item = Self> {
        let Self(x, y) = *self;

        let mut moves = vec![];
        if x >= 1 {
            moves.push(Self(x - 1, y + 1));
        }
        moves.push(Self(x + 1, y + 1));
        if y >= 1 {
            moves.push(Self(x + 1, y - 1));
        }
        if x >= 1 && y >= 1 {
            moves.push(Self(x - 1, y - 1));
        }
        moves.into_iter()
    }
}
