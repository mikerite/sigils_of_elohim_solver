#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::precedence)]

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::{self, FromStr};

use FixedPiece::*;
use Piece::*;

// The maximum of the number of pieces that this library can handle
pub const MAX_PIECE_COUNT: usize = 12;

#[derive(Debug)]
pub enum SolveOneError {
    // The number of squares on the board must be a multiple of 4
    InvalidBoardSize,
    // The number of squares must equare 4 * the number of pieces
    InconsistentPieceCount,
    // The number of pieces is greater than `MAX_PIECE_COUNT`
    PieceCountOverLimit,
}

impl Display for SolveOneError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use SolveOneError::*;
        match self {
            InvalidBoardSize => write!(
                f,
                "The total number of squares on the board is not a multiple of four."
            ),
            InconsistentPieceCount => write!(
                f,
                "The total number of squares on the board and the total number of \
                 squares in pieces don't match."
            ),
            PieceCountOverLimit => write!(
                f,
                "This program can handle at most {} tetrominoes.",
                MAX_PIECE_COUNT
            ),
        }
    }
}

impl Error for SolveOneError {}

pub fn solve_one(
    row_count: u32,
    column_count: u32,
    pieces: PieceCollection,
) -> Result<Option<Position>, SolveOneError> {
    let square_count = row_count * column_count;
    if square_count % 4 != 0 {
        return Err(SolveOneError::InvalidBoardSize);
    }
    let piece_count = pieces.count_all();
    if 4 * piece_count != square_count {
        return Err(SolveOneError::InconsistentPieceCount);
    }
    if piece_count > MAX_PIECE_COUNT as u32 {
        return Err(SolveOneError::PieceCountOverLimit);
    }

    let mut solver = Solver::new(Board::new(row_count, column_count), pieces);
    Ok(solver.solve_one())
}

// Pieces are one-sided tetrominos.
// See https://en.wikipedia.org/wiki/Tetromino#One-sided_tetrominoes
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Piece {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

impl Piece {
    // The number of one-sided tetrominos.
    pub const fn count() -> usize {
        7
    }
}

pub struct PieceCollection {
    counts: [u32; Piece::count()],
}

impl PieceCollection {
    fn count(&self, piece: Piece) -> u32 {
        self.counts[piece as usize]
    }

    fn remove(&mut self, piece: Piece) {
        self.counts[piece as usize] -= 1;
    }

    fn add(&mut self, piece: Piece) {
        self.counts[piece as usize] += 1;
    }
    pub fn count_all(&self) -> u32 {
        self.counts.iter().sum()
    }
}

#[derive(Debug)]
pub enum ParsePieceCollectionError {
    UnrecognizedCharacter,
}

impl Display for ParsePieceCollectionError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "The value contains unrecognized characters.")
    }
}

impl Error for ParsePieceCollectionError {}

impl FromStr for PieceCollection {
    type Err = ParsePieceCollectionError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut counts = [0; Piece::count()];

        for c in s.chars() {
            let piece = match c {
                'I' | 'i' => Piece::I,
                'O' | 'o' => Piece::O,
                'T' | 't' => Piece::T,
                'J' | 'j' => Piece::J,
                'L' | 'l' => Piece::L,
                'S' | 's' => Piece::S,
                'Z' | 'z' => Piece::Z,
                _ => return Err(ParsePieceCollectionError::UnrecognizedCharacter),
            };
            counts[piece as usize] += 1;
        }

        Ok(Self { counts })
    }
}

// The fixed tetrominos.
// See https://en.wikipedia.org/wiki/Tetromino#Fixed_tetrominoes
// x1 is the fixed tetromino x in 'standard' position.
// x2 is x1 rotated 90° clockwise, x3 is 180° and x4 is 270°.
#[derive(Clone, Copy)]
enum FixedPiece {
    I1,
    I2,
    O1,
    T1,
    T2,
    T3,
    T4,
    J1,
    J2,
    J3,
    J4,
    L1,
    L2,
    L3,
    L4,
    S1,
    S2,
    Z1,
    Z2,
}

impl FixedPiece {
    // The number of fixed tetrominos.
    const fn count() -> usize {
        19
    }

    // All the fixed tetrominos in an array
    const fn array() -> [Self; Self::count()] {
        [
            I1, I2, O1, T1, T2, T3, T4, J1, J2, J3, J4, L1, L2, L3, L4, S1, S2, Z1, Z2,
        ]
    }
}

impl From<usize> for FixedPiece {
    fn from(value: usize) -> Self {
        match value {
            0 => I1,
            1 => I2,
            2 => O1,
            3 => T1,
            4 => T2,
            5 => T3,
            6 => T4,
            7 => J1,
            8 => J2,
            9 => J3,
            10 => J4,
            11 => L1,
            12 => L2,
            13 => L3,
            14 => L4,
            15 => S1,
            16 => S2,
            17 => Z1,
            18 => Z2,
            _ => panic!(),
        }
    }
}

// The shape of the fixed tetrominos when the top left corner of the tetromino
// is positioned at the top left corner of the board. The co-ordinates used
// are (row_index, column_index). The (0, 0) square is not included.
// Some shapes hang over the hang over the board, e.g. J1 and L4.
//
//        ------------------
//        | 0,0 | 0,1 | 0,2
// -------|----------------
// | 1,-1 | 1,0 | 1,1 | 1,2
// -------|----------------
// | 2,-1 | 2,0 | 2,1 | 2,2
//
type PieceShape = [(isize, isize); 3];

// An array of all the shapes
const fn piece_shapes() -> [PieceShape; FixedPiece::count()] {
    [
        [(1, 0), (2, 0), (3, 0)],   // I1
        [(0, 1), (0, 2), (0, 3)],   // I2
        [(0, 1), (1, 1), (1, 0)],   // O1
        [(0, 1), (0, 2), (1, 1)],   // T1
        [(1, 0), (1, -1), (2, 0)],  // T2
        [(1, -1), (1, 0), (1, 1)],  // T3
        [(1, 0), (1, 1), (2, 0)],   // T4
        [(1, 0), (2, -1), (2, 0)],  // J1
        [(1, 0), (1, 1), (1, 2)],   // J2
        [(0, 1), (1, 0), (2, 0)],   // J3
        [(0, 1), (0, 2), (1, 2)],   // J4
        [(1, 0), (2, 0), (2, 1)],   // L1
        [(0, 1), (0, 2), (1, 0)],   // L2
        [(0, 1), (1, 1), (2, 1)],   // L3
        [(1, -2), (1, -1), (1, 0)], // L4
        [(0, 1), (1, -1), (1, 0)],  // S1
        [(1, 0), (1, 1), (2, 1)],   // S2
        [(0, 1), (1, 1), (1, 2)],   // Z1
        [(1, -1), (1, 0), (2, -1)], // Z2
    ]
}

const fn piece_shape(fixed_piece: FixedPiece) -> PieceShape {
    piece_shapes()[fixed_piece as usize]
}

// This array is indexed by the `FixedPiece` enum and maps fixed tetrominoes to tetrominoes
const PIECE_MAP: [Piece; FixedPiece::count()] =
    [I, I, O, T, T, T, T, J, J, J, J, L, L, L, L, S, S, Z, Z];

// A position on a board. Contians a row_count * (col_count + 1) long vector representing
// squares of the board. An empty square is represented by b'.' the end of a row is markef by
// b'\n'. A square occupied by a piece is respresented by b'A', b'B', ... . Squares with the
// same character are occupied by the same piece.
#[derive(Eq, PartialEq)]
pub struct Position {
    squares: Vec<u8>,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        const BOX_CHARS: [char; 16] = [
            ' ',   // 0000
            '?',   // 0001 up
            '?',   // 0010 down
            '│', // 0011
            '?',   // 0100 left
            '┘', // 0101
            '┐', // 0110
            '┤', // 0111
            '?',   // 1000 right
            '└', // 1001
            '┌', // 1010
            '├', // 1011
            '─', // 1100
            '┴', // 1101
            '┬', // 1110
            '┼', // 1111
        ];

        if !f.alternate() {
            return write!(f, "{}", str::from_utf8(&self.squares).unwrap());
        }

        let (column_count, _) = self
            .squares
            .iter()
            .enumerate()
            .find(|(_, &s)| s == b'\n')
            .unwrap();

        let row_count = self.squares.len() / (column_count + 1);

        // Get the element on a `row_count` by `2 * self.column_count` rescaled
        // version of squares. Returns `None` if the coordinates are off the board.
        let get = |row: isize, col: isize| -> Option<u8> {
            if row < 0 || row >= row_count as isize || col < 0 || col >= 2 * column_count as isize {
                return None;
            }

            let index = row * (column_count as isize + 1) + col / 2;
            Some(self.squares[index as usize])
        };

        for row in 0..=row_count as isize {
            for col in 0..=2 * column_count as isize {
                let top_left = get(row - 1, col - 1);
                let top_right = get(row - 1, col);
                let bottom_left = get(row, col - 1);
                let bottom_right = get(row, col);

                let up = top_left != top_right;
                let down = bottom_left != bottom_right;
                let left = top_left != bottom_left;
                let right = top_right != bottom_right;

                let char_index = (if up { 1 } else { 0 })
                    + (if down { 2 } else { 0 })
                    + (if left { 4 } else { 0 })
                    + (if right { 8 } else { 0 });

                let c = if char_index > 0 {
                    BOX_CHARS[char_index]
                } else if bottom_right == Some(b'.') {
                    '░'
                } else {
                    ' '
                };

                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Board {
    // The "outer" width; col_count + 1 for the border
    width: usize,
    height: usize,
    bits: u64,
    bitmaps: [u64; FixedPiece::count()],
    stack: [(u64, Piece); MAX_PIECE_COUNT],
    stack_count: usize,
}

impl Board {
    pub fn new(row_count: u32, col_count: u32) -> Self {
        let mut bits = 0_u64;

        let width = col_count as usize + 1;
        let height = row_count as usize;
        let area = width * height;
        bits |= u64::max_value() << area;
        for b in (0..area).skip(width - 1).step_by(width) {
            bits |= 1 << b;
        }

        let mut bitmaps = [1_u64; FixedPiece::count()];
        for (from, to) in piece_shapes().iter().zip(&mut bitmaps) {
            for square in from.iter() {
                *to |= 1 << width as isize * square.0 + square.1
            }
        }

        Self {
            width,
            height,
            bits,
            bitmaps,
            stack: [(0, I); MAX_PIECE_COUNT],
            stack_count: 0,
        }
    }

    fn first_empty_square(&self) -> u32 {
        (self.bits ^ u64::max_value()).trailing_zeros()
    }

    // Returns Ok if the push succeeds and Err if the piece doesn't fit
    fn push(&mut self, fixed_piece: FixedPiece) -> Result<(), ()> {
        debug_assert!(self.stack_count < MAX_PIECE_COUNT);
        let offset = self.first_empty_square();
        let bitmap = self.bitmaps[fixed_piece as usize] << offset;
        if self.bits & bitmap != 0 {
            return Err(());
        }
        self.bits |= bitmap;
        let tetromino_kind = PIECE_MAP[fixed_piece as usize];
        self.stack[self.stack_count] = (bitmap, tetromino_kind);
        self.stack_count += 1;
        Ok(())
    }

    fn pop(&mut self) -> Piece {
        debug_assert!(self.stack_count > 0);
        self.stack_count -= 1;
        let (bitmap, tetromino_kind) = self.stack[self.stack_count];
        self.bits &= !bitmap;
        tetromino_kind
    }

    fn is_complete(&self) -> bool {
        self.bits == u64::max_value()
    }

    fn position(&self) -> Position {
        let mut squares = vec![b'.'; self.width * self.height];

        for (index, &(bitmap, _)) in self.stack[0..self.stack_count].iter().enumerate() {
            let shift = bitmap.trailing_zeros() as usize;
            let bitmap = bitmap >> shift;
            let fixed_piece: FixedPiece = self
                .bitmaps
                .iter()
                .position(|&b| b == bitmap)
                .unwrap()
                .into();
            let shape = piece_shape(fixed_piece);

            let marker = (index + 65) as u8;
            squares[shift] = marker;
            for offset in &shape {
                let offset = self.width as isize * offset.0 + offset.1;
                let index = shift + offset as usize;
                squares[index] = marker;
            }
        }

        for c in squares.iter_mut().skip(self.width - 1).step_by(self.width) {
            *c = b'\n';
        }

        Position { squares }
    }
}

struct Solver {
    board: Board,
    pieces: PieceCollection,
}

impl Solver {
    fn new(board: Board, pieces: PieceCollection) -> Self {
        Self { board, pieces }
    }

    pub fn solve_one(&mut self) -> Option<Position> {
        if self.board.is_complete() {
            return Some(self.board.position());
        }

        for r in &FixedPiece::array() {
            let t = PIECE_MAP[*r as usize];
            if self.pieces.count(t) == 0 {
                continue;
            }
            if self.board.push(*r).is_ok() {
                self.pieces.remove(t);
                let solution = self.solve_one();
                if solution.is_some() {
                    return solution;
                }
                self.board.pop();
                self.pieces.add(t);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::{Board, Solver};

    #[test]
    fn solve_one() {
        let board = Board::new(1, 4);
        let mut solver = Solver::new(board, "I".parse().unwrap());
        let solution = solver.solve_one();
        assert!(solution.is_some());
        assert_eq!(solution.unwrap().to_string(), "AAAA\n");
    }

    mod board {
        use crate::Board;
        use crate::FixedPiece::*;
        use crate::Piece::*;

        #[test]
        fn new() {
            let board = Board::new(5, 4);
            let position = board.position();

            assert_eq!(
                position.to_string(),
                "....\n\
                 ....\n\
                 ....\n\
                 ....\n\
                 ....\n"
            );
        }

        #[test]
        fn push() {
            let mut board = Board::new(4, 4);
            let output = board.push(I1);
            let position = board.position();

            assert!(output.is_ok());
            assert_eq!(
                position.to_string(),
                "A...\n\
                 A...\n\
                 A...\n\
                 A...\n"
            );
        }

        #[test]
        fn push_twice() {
            let mut board = Board::new(4, 4);
            let output1 = board.push(I1);
            let output2 = board.push(I1);
            let position = board.position();

            assert!(output1.is_ok());
            assert!(output2.is_ok());
            assert_eq!(
                position.to_string(),
                "AB..\n\
                 AB..\n\
                 AB..\n\
                 AB..\n"
            );
        }

        #[test]
        fn push_failure() {
            let mut board = Board::new(1, 4);
            let output = board.push(I1);
            let position = board.position();

            assert!(output.is_err());
            assert_eq!(position.to_string(), "....\n");
        }

        #[test]
        fn pop1() {
            let mut board = Board::new(5, 4);
            board.push(O1).unwrap();
            let popped = board.pop();
            let position = board.position();

            assert_eq!(popped, O);
            assert_eq!(
                position.to_string(),
                "....\n\
                 ....\n\
                 ....\n\
                 ....\n\
                 ....\n"
            );
        }

        #[test]
        fn pop2() {
            let mut board = Board::new(5, 4);
            board.push(O1).unwrap();
            board.push(I1).unwrap();
            let popped = board.pop();
            let position = board.position();

            assert_eq!(popped, I);
            assert_eq!(
                position.to_string(),
                "AA..\n\
                 AA..\n\
                 ....\n\
                 ....\n\
                 ....\n"
            );
        }

        #[test]
        fn is_complete() {
            let mut board = Board::new(1, 4);
            assert!(!board.is_complete());
            board.push(I2).unwrap();
            assert!(board.is_complete());
        }
    }

    mod shapes {
        use crate::FixedPiece::*;
        use crate::{Board, FixedPiece};

        // Push fixed pieces onto a blank `row_count` by `col_count` board and
        // return the display
        fn push_output<'a, T: IntoIterator<Item = &'a FixedPiece>>(
            row_count: u32,
            col_count: u32,
            pieces: T,
        ) -> String {
            let mut board = Board::new(row_count, col_count);
            for r in pieces {
                let output = board.push(*r);
                assert!(output.is_ok());
            }
            board.position().to_string()
        }

        #[test]
        fn i1() {
            assert_eq!(
                push_output(5, 4, &[I1]),
                "A...\n\
                 A...\n\
                 A...\n\
                 A...\n\
                 ....\n"
            );
        }

        #[test]
        fn i2() {
            assert_eq!(
                push_output(5, 4, &[I2]),
                "AAAA\n\
                 ....\n\
                 ....\n\
                 ....\n\
                 ....\n"
            );
        }

        #[test]
        fn o1() {
            assert_eq!(
                push_output(5, 4, &[O1]),
                "AA..\n\
                 AA..\n\
                 ....\n\
                 ....\n\
                 ....\n"
            );
        }

        #[test]
        fn t1() {
            assert_eq!(
                push_output(5, 4, &[T1]),
                "AAA.\n\
                 .A..\n\
                 ....\n\
                 ....\n\
                 ....\n"
            );
        }

        #[test]
        fn t2() {
            assert_eq!(
                push_output(4, 5, &[I2, T2]),
                "AAAAB\n\
                 ...BB\n\
                 ....B\n\
                 .....\n"
            );
        }

        #[test]
        fn t3() {
            assert_eq!(
                push_output(4, 6, &[I2, T3]),
                "AAAAB.\n\
                 ...BBB\n\
                 ......\n\
                 ......\n"
            );
        }

        #[test]
        fn t4() {
            assert_eq!(
                push_output(4, 5, &[T4]),
                "A....\n\
                 AA...\n\
                 A....\n\
                 .....\n"
            );
        }

        #[test]
        fn j1() {
            assert_eq!(
                push_output(4, 5, &[I2, J1]),
                "AAAAB\n\
                 ....B\n\
                 ...BB\n\
                 .....\n"
            );
        }

        #[test]
        fn j2() {
            assert_eq!(
                push_output(4, 5, &[J2]),
                "A....\n\
                 AAA..\n\
                 .....\n\
                 .....\n"
            );
        }

        #[test]
        fn j3() {
            assert_eq!(
                push_output(4, 5, &[J3]),
                "AA...\n\
                 A....\n\
                 A....\n\
                 .....\n"
            );
        }

        #[test]
        fn j4() {
            assert_eq!(
                push_output(4, 5, &[J4]),
                "AAA..\n\
                 ..A..\n\
                 .....\n\
                 .....\n"
            );
        }

        #[test]
        fn l1() {
            assert_eq!(
                push_output(4, 5, &[L1]),
                "A....\n\
                 A....\n\
                 AA...\n\
                 .....\n"
            );
        }

        #[test]
        fn l2() {
            assert_eq!(
                push_output(4, 5, &[L2]),
                "AAA..\n\
                 A....\n\
                 .....\n\
                 .....\n"
            );
        }

        #[test]
        fn l3() {
            assert_eq!(
                push_output(4, 5, &[L3]),
                "AA...\n\
                 .A...\n\
                 .A...\n\
                 .....\n"
            );
        }

        #[test]
        fn l4() {
            assert_eq!(
                push_output(4, 5, &[I2, L4]),
                "AAAAB\n\
                 ..BBB\n\
                 .....\n\
                 .....\n"
            );
        }

        #[test]
        fn s1() {
            assert_eq!(
                push_output(4, 6, &[I2, S1]),
                "AAAABB\n\
                 ...BB.\n\
                 ......\n\
                 ......\n"
            );
        }

        #[test]
        fn s2() {
            assert_eq!(
                push_output(4, 5, &[S2]),
                "A....\n\
                 AA...\n\
                 .A...\n\
                 .....\n"
            );
        }

        #[test]
        fn z1() {
            assert_eq!(
                push_output(4, 6, &[Z1]),
                "AA....\n\
                 .AA...\n\
                 ......\n\
                 ......\n"
            );
        }

        #[test]
        fn z2() {
            assert_eq!(
                push_output(4, 5, &[I2, Z2]),
                "AAAAB\n\
                 ...BB\n\
                 ...B.\n\
                 .....\n"
            );
        }
    }

    mod pretty_print {
        use crate::Board;
        use crate::FixedPiece::*;

        #[test]
        fn empty_board() {
            let board = Board::new(4, 5);
            let position = board.position();
            let output = format!("{:#}", position);

            assert_eq!(
                output,
                "┌─────────┐\n\
                 │░░░░░░░░░│\n\
                 │░░░░░░░░░│\n\
                 │░░░░░░░░░│\n\
                 └─────────┘\n"
            );
        }

        #[test]
        fn vertical_border() {
            let mut board = Board::new(4, 5);
            board.push(I1).unwrap();
            let position = board.position();
            let output = format!("{:#}", position);

            assert_eq!(
                output,
                "┌─┬───────┐\n\
                 │ │░░░░░░░│\n\
                 │ │░░░░░░░│\n\
                 │ │░░░░░░░│\n\
                 └─┴───────┘\n"
            );
        }

        #[test]
        fn horizontal_border() {
            let mut board = Board::new(5, 4);
            board.push(I2).unwrap();
            let position = board.position();
            let output = format!("{:#}", position);

            assert_eq!(
                output,
                "┌───────┐\n\
                 ├───────┤\n\
                 │░░░░░░░│\n\
                 │░░░░░░░│\n\
                 │░░░░░░░│\n\
                 └───────┘\n"
            );
        }

        #[test]
        fn corners() {
            let mut board = Board::new(4, 5);
            board.push(Z1).unwrap();
            let position = board.position();
            let output = format!("{:#}", position);

            assert_eq!(
                output,
                "┌───┬─────┐\n\
                 ├─┐ └─┐░░░│\n\
                 │░└───┘░░░│\n\
                 │░░░░░░░░░│\n\
                 └─────────┘\n"
            );
        }
    }
}
