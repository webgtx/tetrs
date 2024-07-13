use crate::backend::game::{ActivePiece, Board, Orientation, Tetromino};

pub type RotateFn = fn(piece: ActivePiece, board: Board, right_turns: i32) -> Option<ActivePiece>;

#[allow(dead_code)]
pub fn rotate_dummy(mut piece: ActivePiece, board: Board, right_turns: i32) -> Option<ActivePiece> {
    piece.orientation = piece.orientation.rotate_r(right_turns);
    piece.fits_at(board, (0, 0))
}

#[allow(dead_code)]
pub fn rotate_classic(
    mut piece: ActivePiece,
    board: Board,
    right_turns: i32,
) -> Option<ActivePiece> {
    let right = match right_turns.rem_euclid(4) {
        // No rotation occurred.
        0 => return Some(piece),
        // One right rotation.
        1 => true,
        // Classic didn't define 180 rotation, just check if the "default" 180 rotation fits.
        2 => {
            piece.orientation = piece.orientation.rotate_r(right_turns);
            return piece.fits_at(board, (0, 0));
        }
        // One left rotation.
        3 => false,
        _ => unreachable!(),
    };
    use Orientation::*;
    #[rustfmt::skip]
    let offset = match piece.shape {
        Tetromino::O => (0, 0), // ⠶
        Tetromino::I => match piece.orientation {
            N | S => (2, -1), // ⠤⠤ -> ⡇
            E | W => (-2, 1), // ⡇  -> ⠤⠤
        },
        Tetromino::S | Tetromino::Z => match piece.orientation {
            N | S => (1, 0),  // ⠴⠂ -> ⠳  // ⠲⠄ -> ⠞
            E | W => (-1, 0), // ⠳  -> ⠴⠂ // ⠞  -> ⠲⠄
        },
        Tetromino::T | Tetromino::L | Tetromino::J => match piece.orientation {
            N => if right { (1, -1) } else { (-1, 1) } // ⠴⠄ <-> ⠗  // ⠤⠆ <-> ⠧  // ⠦⠄ <-> ⠏
            E => if right { (-1, 0) } else { (1, 0) }  // ⠗  <-> ⠲⠂ // ⠧  <-> ⠖⠂ // ⠏  <-> ⠒⠆
            S => (0, 0),                               // ⠲⠂ <-> ⠺  // ⠖⠂ <-> ⠹  // ⠒⠆ <-> ⠼
            W => if right { (0, 1) } else { (0, -1) }  // ⠺  <-> ⠴⠄ // ⠹  <-> ⠤⠆ // ⠼  <-> ⠦⠄
        },
    };
    piece.orientation = piece.orientation.rotate_r(right_turns);
    piece.fits_at(board, offset)
}

/* TODO: Improve and implement the 'Okay' Rotation System.
pub fn rotate_okay(piece: ActivePiece, board: Board, right_turns: i32) -> Option<ActivePiece> {
    let ActivePiece(shape, o, pos) = piece;
    let r = match right_turns.rem_euclid(4) {
        0 => return Some(piece),
        1 => true,
        2 => return first_valid_kick(board, ActivePiece(shape, o.rotate_r(2), pos), 2, std::iter::once((0,0))),
        3 => false,
        _ => unreachable!()
    };
    use Orientation::*;
    #[rustfmt::skip]
    let kicks = match shape {
        Tetromino::O => (0,0), // ⠶ -> ⠶
        Tetromino::I => match o {
            N | S => (2,2), // ⠤⠤ -> ⡇
            E | W => (-2,-2), // ⡇  -> ⠤⠤
        },
        Tetromino::S => match o {
            N | S => (1,1), // ⠴⠂ -> ⠳
            E | W => (-1,-1), // ⠳  -> ⠴⠂
        },
        Tetromino::Z => match o {
            N | S => (1,1), // ⠲⠄ -> ⠞
            E | W => (-1,-1), // ⠞  -> ⠲⠄
        },
        Tetromino::T => match o {
            N => if r {(1,0)} else {(-1,0)}, // ⠴⠄ -> ⠗
            E => if r {(-1,-1)} else {(1,1)}, // ⠗  -> ⠲⠂
            S => if r {(0,1)} else {(0,-1)}, // ⠲⠂ -> ⠺
            W => (0,0), // ⠺  -> ⠴⠄
        },
        Tetromino::L => match o {
            N => if r {(1,0)} else {(-1,0)}, // ⠤⠆ -> ⠧
            E => if r {(-1,-1)} else {(1,1)}, // ⠧  -> ⠖⠂
            S => if r {(0,1)} else {(0,-1)}, // ⠖⠂ -> ⠹
            W => (0,0), // ⠹  -> ⠤⠆
        },
        Tetromino::J => match o {
            N => if r {(1,0)} else {(-1,0)}, // ⠦⠄ -> ⠏
            E => if r {(-1,-1)} else {(1,1)}, // ⠏  -> ⠒⠆
            S => if r {(0,1)} else {(0,-1)}, // ⠒⠆ -> ⠼
            W => (0,0), // ⠼  -> ⠦⠄
        },
    };
    first_valid_kick(board, piece, 1, kicks)
}*/

/* TODO: Implement the Super Rotation System.
pub fn rotate_super(board: Board, piece: ActivePiece, right_turns: i32) -> Option<ActivePiece> {}
*/

/* TODO: Implement the Arika Rotation System.
pub fn rotate_arika(board: Board, piece: ActivePiece, right_turns: i32) -> Option<ActivePiece> {}
*/
