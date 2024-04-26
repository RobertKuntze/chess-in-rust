use std::fmt;

fn main() {
    let board: Board = Board { pieces: Vec::new() };
    let board: Board = board.initialize();

    board.print_board();
    println!("{:?}", board.get_piece(Position(1, 1)).unwrap().get_moves(&board.pieces));
}

struct Board{
    pieces: Vec<Piece>,
}

impl Board{
    fn initialize(mut self) -> Board {
        let pieces = String::from("RkBQKBkR");
        for i in 0..pieces.len() {
            self.pieces.push(Piece::new(Position(0, i.try_into().unwrap()), true, pieces.chars().nth(i).unwrap()));
        } for i in 0..8 {
            self.pieces.push(Piece::new(Position(1, i), true, 'P'));
        } for i in 0..8 {
            self.pieces.push(Piece::new(Position(6, i), false, 'p'));
        } for i in 0..pieces.len() {
            self.pieces.push(Piece::new(Position(7, i.try_into().unwrap()), false, pieces.chars().nth(i).unwrap()));
        }
        self
    }

    fn print_board(&self) {
        let file = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let rank = ['1', '2', '3', '4', '5', '6', '7', '8'];
        for i in 0..8 {
            print!("{} ", rank[i]);
            for j in 0..8 {
                let mut found = false;
                for piece in &self.pieces {
                    if piece.position.0 == i.try_into().unwrap() && piece.position.1 == j {
                        print!("{:?} ", piece);
                        found = true;
                        break;
                    }
                }
                if !found {
                    print!(". ");
                }
            }
            print!("\n");
        }
    }

    fn get_piece(&self, position: Position) -> Option<&Piece> {
        for piece in &self.pieces {
            if piece.position == position {
                return Some(piece);
            }
        }
        None
    }
}


trait VecPiece {
    fn get_piece(&self, position: Position) -> Option<&Piece>;
}

impl VecPiece for Vec<Piece> {
    fn get_piece(&self, position: Position) -> Option<&Piece> {
        for piece in self {
            if piece.position == position {
                return Some(piece);
            }
        }
        None
    }
}

impl fmt::Debug for Board{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.pieces)
    }
}

#[derive(Copy, Clone, PartialEq)]
struct Position(u8, u8);

impl fmt::Debug for Position{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{}{}", (self.0 + 97) as char, self.1 + 1)
    }
}

#[derive(Copy, Clone, PartialEq)]
struct Piece{
    position : Position,
    white : bool,
    piece : char,
}

impl Piece{
    fn new(position: Position, white: bool, piece: char) -> Self {
        Piece {position, white, piece}
    }

    fn move_piece(&mut self, position: Position) {
        self.position = position;
    }

    fn get_moves(&self,pieces: &Vec<Piece>) -> Vec<Position> {
        let mut moves = Vec::new();
        let mut range = 1;
        match self.piece {
            'P' => {
                if self.white && self.position.0 == 1 {
                    range = 2;
                }

                for i in 1..=range {
                    let new_position = Position(self.position.0, self.position.1 + i);
                    if pieces.get_piece(new_position).is_none() {
                        moves.push(new_position);
                    } else {
                        break;
                    }
                }
            }
            _ => {}
        }
        moves
    }
}

impl fmt::Debug for Piece{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.piece)
    }
}

