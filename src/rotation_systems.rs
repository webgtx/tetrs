use crate::game_logic::{Coord, Board, GamePiece, Tetromino, Orientation};

type Offset = (isize,isize);

trait RotationSystem {
    fn rotate_right(board: Board, piece: GamePiece) -> Option<GamePiece>;
    fn rotate_left(board: Board, piece: GamePiece) -> Option<GamePiece>;
    fn rotate_180(board: Board, piece: GamePiece) -> Option<GamePiece>;
}

pub fn add((x0,y0): Coord, (x1,y1): Offset) -> Option<Coord> {
    Some((x0.checked_add_signed(x1)?, y0.checked_add_signed(y1)?))
}

fn find_first_valid_kick(board: Board, piece:GamePiece, right_turns: i32, mut kicks: impl IntoIterator<Item=Offset>) -> Option<GamePiece> {
    let GamePiece(shape, o, pos) = piece;
    kicks.into_iter().find_map(|offset| {
        let new_piece = GamePiece(shape, o.rotate_r(right_turns), add(pos, offset)?);
        if new_piece.fits(board) {
            Some(new_piece)
        } else {
            None
        }
    })
}

struct NintendoRS;
impl RotationSystem for NintendoRS {
    fn rotate_right(board: Board, piece: GamePiece) -> Option<GamePiece> {
        let GamePiece(shape, o, pos) = piece;
        use Orientation::*;
        let kick = match shape {
            Tetromino::O => (0,0), // ⠶ -> ⠶
            Tetromino::I => match o {
                N | S => (2,2), // ⠤⠤  -> ⡇
                E | W => (-2,-2), // ⡇ -> ⠤⠤
            },
            Tetromino::S => match o {
                N | S => (0,0), // ⠴⠂ -> ⠳ // TODO
                E | W => (0,0), // ⠳  -> ⠴⠂
            },
            Tetromino::Z => match o {
                N | S => (0,0), // ⠲⠄ -> ⠞
                E | W => (0,0), // ⠞  -> ⠲⠄
            },
            Tetromino::T => match o {
                N => (0,0), // ⠴⠄ -> ⠗
                E => (0,0), // ⠗  -> ⠲⠂
                S => (0,0), // ⠲⠂ -> ⠺
                W => (0,0), // ⠺  -> ⠴⠄
            },
            Tetromino::L => match o {
                N => (0,0), // ⠤⠆ -> ⠧
                E => (0,0), // ⠧  -> ⠖⠂
                S => (0,0), // ⠖⠂ -> ⠹
                W => (0,0), // ⠹  -> ⠤⠆
            },
            Tetromino::J => match o {
                N => (0,0), // ⠦⠄ -> ⠏
                E => (0,0), // ⠏  -> ⠒⠆
                S => (0,0), // ⠒⠆ -> ⠼
                W => (0,0), // ⠼  -> ⠦⠄
            },
        };
        find_first_valid_kick(board, piece, 1, [kick])
    }

    fn rotate_left(board: Board, piece: GamePiece) -> Option<GamePiece> {
        unimplemented!()
    }
    
    fn rotate_180(board: Board, piece: GamePiece) -> Option<GamePiece> {
        unimplemented!()
    }
}

// The Super Rotation System.
struct SuperRS;
impl RotationSystem for SuperRS {
    fn rotate_right(board: Board, piece: GamePiece) -> Option<GamePiece> {
        unimplemented!()
    }

    fn rotate_left(board: Board, piece: GamePiece) -> Option<GamePiece> {
        unimplemented!()
    }
    
    fn rotate_180(board: Board, piece: GamePiece) -> Option<GamePiece> {
        unimplemented!()
    }
}