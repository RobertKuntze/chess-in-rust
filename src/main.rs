use std::{cmp, fmt, u8::MAX};

fn main() {
    let board: Board = Board { pieces: Vec::new() };
    let mut board: Board = board.initialize();
    let mut white = true;

    loop {
        board.print_board();
        println!("Current Player: {}", if white {"White"} else {"Black"});
        print!("Choose a piece to move: \n");
        let position = get_user_input();
        let piece = board.get_piece(position);
        match piece {
            None => continue,
            Some(p) => {
                if p.white != white {
                    print!("Please choose a piece of the correct color\n");
                    continue;
                }
                let moves = p.get_moves(&board.pieces);
                for m in moves {
                    print!("{:?} ", m);
                }
                print!("\nChoose a possible move:\n");
                let new_position = get_user_input();
                if p.get_moves(&board.pieces).contains(&new_position) {
                    board.move_piece(position, new_position);
                    if board.is_check(white) {
                        print!("Check!\n");
                        return;
                    }
                    white = !white;
                }
            }
        }
    }
}

fn get_user_input() -> Position {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    String::as_pos(&input.trim().to_string())
}

struct Board{
    pieces: Vec<Piece>,
}

impl Board{
    fn initialize(mut self) -> Board {
        let pieces = String::from("RNBQKBNR");
        for i in 0..pieces.len() {
            self.pieces.push(Piece::new(Position(i.try_into().unwrap(), 0), true, pieces.chars().nth(i).unwrap()));
        } for i in 0..8 {
            self.pieces.push(Piece::new(Position(i, 1), true, 'P'));
        } for i in 0..8 {
            self.pieces.push(Piece::new(Position(i, 6), false, 'P'));
        } for i in 0..pieces.len() {
            self.pieces.push(Piece::new(Position(i.try_into().unwrap(), 7), false, pieces.chars().nth(i).unwrap()));
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
                    if piece.position.1 == i.try_into().unwrap() && piece.position.0 == j {
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
        let index = self.pieces.get_index(position);
        match index {
            None => None,
            Some(i) => Some(&self.pieces[i])
        }
    }

    fn move_piece(&mut self, position: Position, new_position: Position) {
        let index = self.pieces.get_index(position);
        let other_index = self.pieces.get_index(new_position);
        match index {
            None => return,
            Some(i) => {
                if other_index.is_some() {
                    self.pieces.remove(other_index.unwrap());
                }
                self.pieces[i].position = new_position;
            }
        }
    }

    fn is_check(&self, white: bool) -> bool {
        let mut king_position = Position(0, 0);
        for piece in &self.pieces {
            if piece.piece == 'K' && piece.white == white {
                king_position = piece.position;
                break;
            }
        }
        for piece in &self.pieces {
            if piece.white == white {
                continue;
            }
            let moves = piece.get_moves(&self.pieces);
            if moves.contains(&king_position) {
                return true;
            }
        }
        false
    }
}


trait VecPiece {
    fn get_piece(&self, position: Position) -> Option<&Piece>;
    fn get_index(&self, position: Position) -> Option<usize>;
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

    fn get_index(&self, position: Position) -> Option<usize> {
        for (i, piece) in self.iter().enumerate() {
            if piece.position == position {
                return Some(i);
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

impl Position {
    fn new(x: char, y: u8) -> Self {
        Position(x as u8 - 97, y - 1)
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

    fn print_piece(&self) {
        print!("{:?} {:?}", self.position, self.piece);
    }

    fn get_moves(&self ,pieces: &Vec<Piece>) -> Vec<Position> {
        let mut moves = Vec::new();
        let mut range = 1;
        let piece: Piece = *self;

        match piece.piece {
            'P' => {
                if (piece.white && piece.position.1 == 1) || (!piece.white && piece.position.1 == 6) {
                    range = 2;
                };

                for i in 1..=range {
                    let new_position = Position(piece.position.0, if piece.white {piece.position.1 + i} else {piece.position.1 - i});
                    if pieces.get_piece(new_position).is_none() {
                        moves.push(new_position);
                    } else {
                        break;
                    }
                }
                if pieces.get_piece(Position(cmp::max((piece.position.0 as i8 - 1) as u8, 0), if piece.white {piece.position.1 + 1} else {piece.position.1 - 1})).is_some() {
                    moves.push(Position(piece.position.0 - 1, if piece.white {piece.position.1 + 1} else {piece.position.1 - 1}));
                } if pieces.get_piece(Position(cmp::min((piece.position.0 as i8 + 1) as u8, 7), if piece.white {piece.position.1 + 1} else {piece.position.1 - 1})).is_some() {
                    moves.push(Position(piece.position.0 + 1, if piece.white {piece.position.1 + 1} else {piece.position.1 - 1}));
                }
            },
            'K' => {
                for i in -1..=1_i8 {
                    for j in -1..=1_i8 {
                        if (i == 0 && j == 0) || (piece.position.0 as i8 + i < 0) || (piece.position.0 as i8 + i > 7) 
                        || (piece.position.1 as i8 + j < 0) || (piece.position.1 as i8 + j > 7) {
                            continue;
                        }
                        let new_position = Position::new((piece.position.0 as i8 + i + 97)as u8 as char,
                        (piece.position.1 as i8 + j + 1) as u8);
                        if pieces.get_piece(new_position).is_none() || pieces.get_piece(new_position).unwrap().white != piece.white {
                            moves.push(new_position);
                        }
                    }
                }
            },
            'Q' => {
                for i in -1..=1_i8 {
                    for j in -1..=1_i8 {
                        if i == 0 && j == 0 {
                            continue;
                        }
                        for k in 1..8 {
                            if (piece.position.0 as i8 + i * k < 0) || (piece.position.0 as i8 + i * k > 7) 
                            || (piece.position.1 as i8 + j * k < 0) || (piece.position.1 as i8 + j * k > 7) {
                                break;
                            }
                            let new_position = Position::new((piece.position.0 as i8 + i * k + 97)as u8 as char,
                            (piece.position.1 as i8 + j * k + 1) as u8);
                            if pieces.get_piece(new_position).is_none() {
                                moves.push(new_position);
                            } else {
                                if pieces.get_piece(new_position).unwrap().white != piece.white {
                                    moves.push(new_position);
                                }
                                break;
                            }
                        }
                    }
                }
            },
            'R' => {
                for i in -1..=1_i8 {
                    for j in -1..=1_i8 {
                        if i.abs() + j.abs() != 1{
                            continue;
                        }
                        for k in 1..8 {
                            if (piece.position.0 as i8 + i * k < 0) || (piece.position.0 as i8 + i * k > 7) 
                            || (piece.position.1 as i8 + j * k < 0) || (piece.position.1 as i8 + j * k > 7) {
                                break;
                            }
                            let new_position = Position::new((piece.position.0 as i8 + i * k + 97)as u8 as char,
                            (piece.position.1 as i8 + j * k + 1) as u8);
                            if pieces.get_piece(new_position).is_none() {
                                moves.push(new_position);
                            } else {
                                if pieces.get_piece(new_position).unwrap().white != piece.white {
                                    moves.push(new_position);
                                }
                                break;
                            }
                        }
                    }
                }
            },
            'B' => {
                for i in -1..=1_i8 {
                    for j in -1..=1_i8 {
                        if i.abs() + j.abs() != 2 {
                            continue;
                        }
                        for k in 1..8 {
                            if (piece.position.0 as i8 + i * k < 0) || (piece.position.0 as i8 + i * k > 7) 
                            || (piece.position.1 as i8 + j * k < 0) || (piece.position.1 as i8 + j * k > 7) {
                                break;
                            }
                            let new_position = Position::new((piece.position.0 as i8 + i * k + 97)as u8 as char,
                            (piece.position.1 as i8 + j * k + 1) as u8);
                            if pieces.get_piece(new_position).is_none() {
                                moves.push(new_position);
                            } else {
                                if pieces.get_piece(new_position).unwrap().white != piece.white {
                                    moves.push(new_position);
                                }
                                break;
                            }
                        }
                    }
                }
            },
            'N' => {
                for i in -2..=2_i8 {
                    for j in -2..=2_i8 {
                        if i.abs() + j.abs() != 3 || i == 0 || j == 0 {
                            continue;
                        }
                        if (piece.position.0 as i8 + i < 0) || (piece.position.0 as i8 + i > 7) 
                        || (piece.position.1 as i8 + j < 0) || (piece.position.1 as i8 + j > 7) {
                            continue;
                        }
                        let new_position = Position::new((piece.position.0 as i8 + i + 97)as u8 as char,
                        (piece.position.1 as i8 + j + 1) as u8);
                        if pieces.get_piece(new_position).is_none() || pieces.get_piece(new_position).unwrap().white != piece.white {
                            moves.push(new_position);
                        }
                    }
                }
            },
            
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

trait AsPosition {
    fn as_pos(string: &str) -> Position;
}

impl AsPosition for String {
    fn as_pos(string: &str) -> Position {
        Position(string.chars().nth(0).unwrap() as u8 - 97, string.chars().nth(1).unwrap() as u8 - 49)
    }
}