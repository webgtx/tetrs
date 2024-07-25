use std::{collections::VecDeque, num::NonZeroU32, time::Duration};

use tetrs_engine::{
    Feedback, FeedbackEvents, Game, GameConfig, GameOver, GameState, Gamemode, InternalEvent, Stat,
    Tetromino,
};

pub fn make_game() -> Game {
    const SPEED_LEVEL: NonZeroU32 = NonZeroU32::MIN.saturating_add(1);
    let mut init = false;
    let mut puzzle_piece_stamp = 0;
    #[allow(non_snake_case)]
    // SAFETY: 255 > 0.
    #[rustfmt::skip]
    let puzzles = [
        /* Puzzle template.
        ("puzzlename", vec![
            b"OOOOOOOOOO",
            b"OOOOOOOOOO",
            b"OOOOOOOOOO",
            b"OOOOOOOOOO",
        ], VecDeque::from([Tetromino::I,])),
        */
        ("Intro", vec![
            b"OOO    OOO",
            b"OOOO  OOOO",
            b"OOOOO OOOO",
            b"OOOOO OOOO",
            ], VecDeque::from([Tetromino::I,Tetromino::L])),
        // I-spins.
        ("1.1 I-spin", vec![
            b"OOOOO OOOO",
            b"OOOOO OOOO",
            b"OOOOO OOOO",
            b"OOOOO OOOO",
            b"OOOO    OO",
            ], VecDeque::from([Tetromino::I,Tetromino::I])),
        ("1.2 I-spin", vec![
            b"OOOOO  OOO",
            b"OOOOO OOOO",
            b"OOOOO OOOO",
            b"OO    OOOO",
            ], VecDeque::from([Tetromino::I,Tetromino::J])),
        ("1.3 I-spin", vec![
            b"OO  O   OO",
            b"OO    OOOO",
            b"OOOO OOOOO",
            b"OOOO OOOOO",
            b"OOOO OOOOO",
            ], VecDeque::from([Tetromino::I,Tetromino::L,Tetromino::O,])),
        ("1.4 I-spin trial", vec![
            b"OOOOO  OOO",
            b"OOO OO OOO",
            b"OOO OO OOO",
            b"OOO     OO",
            b"OOO OOOOOO",
            ], VecDeque::from([Tetromino::I,Tetromino::I,Tetromino::L,])),
        // S/Z-spins.
        ("2.1 S-spin", vec![
            b"OOOO  OOOO",
            b"OOO  OOOOO",
            ], VecDeque::from([Tetromino::S,])),
        ("2.2 S-spins", vec![
            b"OOOO    OO",
            b"OOO    OOO",
            b"OOOOO  OOO",
            b"OOOO  OOOO",
            ], VecDeque::from([Tetromino::S,Tetromino::S,Tetromino::S,])),
        ("2.3 Z-spin galore", vec![
            b"O  OOOOOOO",
            b"OO  OOOOOO",
            b"OOO  OOOOO",
            b"OOOO  OOOO",
            b"OOOOO  OOO",
            b"OOOOOO  OO",
            b"OOOOOOO  O",
            b"OOOOOOOO  ",
            ], VecDeque::from([Tetromino::Z,Tetromino::Z,Tetromino::Z,Tetromino::Z,])),
        ("2.4 SuZ-spin trial", vec![
            b"OOOO  OOOO",
            b"OOO  OOOOO",
            b"OO    OOOO",
            b"OO    OOOO",
            b"OOO    OOO",
            b"OO  OO  OO",
            ], VecDeque::from([Tetromino::S,Tetromino::S,Tetromino::I,Tetromino::I,Tetromino::Z,])),
        // L/J-spins.
        ("3.1 J-spin", vec![
            b"OO     OOO",
            b"OOOOOO OOO",
            b"OOOOO  OOO",
            ], VecDeque::from([Tetromino::J,Tetromino::I,])),
        ("3.2 L/J-spin", vec![
            b"OO      OO",
            b"OO OOOO OO",
            b"OO  OO  OO",
            ], VecDeque::from([Tetromino::J,Tetromino::L,Tetromino::I])),
        ("3.3 L-spin", vec![
            b"OOOOO OOOO",
            b"OOO   OOOO",
            ], VecDeque::from([Tetromino::L,])),
        ("3.4 L/J-spin trial", vec![
            b"O   OO   O",
            b"O O OO O O",
            b"O   OO   O",
            ], VecDeque::from([Tetromino::J,Tetromino::L,Tetromino::J,Tetromino::L,])),
        // L/J-turns.
        ("4.1 L-turn", vec![
            b"OOOO  OOOO",
            b"OOOO  OOOO",
            b"OOOO   OOO",
            b"OOOO OOOOO",
            ], VecDeque::from([Tetromino::L,Tetromino::O,])),
        ("4.2 L-turn", vec![
            b"OOOOO  OOO",
            b"OOO    OOO",
            b"OOOO OOOOO",
            b"OOOO OOOOO",
            ], VecDeque::from([Tetromino::L,Tetromino::O,])),
        ("4.3 77-turn", vec![
            b"OOOO  OOOO",
            b"OOOOO OOOO",
            b"OOO   OOOO",
            b"OOOO OOOOO",
            b"OOOO OOOOO",
            ], VecDeque::from([Tetromino::L,Tetromino::L,])),
        ("4.4 L-turn trial", vec![
            b"OOOO  OOOO",
            b"OOOO  OOOO",
            b"OO     OOO",
            b"OOO  OOOOO",
            b"OOO OOOOOO",
            ], VecDeque::from([Tetromino::L,Tetromino::L,Tetromino::O,])),
        // T-spins.
        ("5.1 T-spin", vec![
            b"OOOO    OO",
            b"OOO   OOOO",
            b"OOOO OOOOO",
            ], VecDeque::from([Tetromino::T,Tetromino::I])),
        ("5.2 T-spin", vec![
            b"OOOO    OO",
            b"OOO   OOOO",
            b"OOOO OOOOO",
            ], VecDeque::from([Tetromino::T,Tetromino::L])),
        ("5.3 T-turn", vec![
            b"OOO   OOOO",
            b"OOOO  OOOO",
            b"OOOO   OOO",
            ], VecDeque::from([Tetromino::T,Tetromino::T])),
        ("5.4 Tetrs T-spin", vec![
            b"OOO  OOOOO",
            b"OOO  OOOOO",
            b"OOOO   OOO",
            b"OOOOO OOOO",
            ], VecDeque::from([Tetromino::T,Tetromino::O])),
        ("5.5 Tetrs T-spin Triple", vec![
            b"OOO  OOOOO",
            b"OOO  OOOOO",
            b"OOOO   OOO",
            b"OOOOO OOOO",
            b"OOOOO  OOO",
            b"OOOOO OOOO",
            ], VecDeque::from([Tetromino::T,Tetromino::J,Tetromino::L])),
    ];
    let total_lines = puzzles
        .iter()
        .map(|(_, puzzle_lines, _)| puzzle_lines.len())
        .sum::<usize>();
    let mut puzzles = puzzles.into_iter();
    let game_modifier = move |upcoming_event: Option<InternalEvent>,
                              config: &mut GameConfig,
                              state: &mut GameState,
                              feedback_events: &mut FeedbackEvents| {
        // Initialize internal game state.
        if !init {
            config.preview_count = 1;
            init = true;
        }
        // Puzzle may have failed.
        let game_piece_stamp = state.pieces_played.iter().sum::<u32>();
        if upcoming_event == Some(InternalEvent::Spawn) && game_piece_stamp == puzzle_piece_stamp {
            // If board is cleared successfully load in next batch.
            if state.board.iter().all(|line| {
                line.iter().all(|cell| cell.is_none()) || line.iter().all(|cell| cell.is_some())
            }) {
                // Load in new puzzle.
                if let Some((puzzle_name, puzzle_lines, puzzle_pieces)) = puzzles.next() {
                    state.consecutive_line_clears = 0;
                    // Game messages.
                    feedback_events.push((
                        state.game_time,
                        Feedback::Message(format!("Puzzle {puzzle_name}")),
                    ));
                    // Queue pieces and lines.
                    puzzle_piece_stamp =
                        game_piece_stamp + u32::try_from(puzzle_pieces.len()).unwrap();
                    state.next_pieces = puzzle_pieces;
                    // Additional piece for consistent end preview.
                    state.next_pieces.push_back(Tetromino::I);
                    for (y, line_template) in puzzle_lines.iter().rev().enumerate() {
                        state.board[y] = line_template.map(|b| {
                            if b == b' ' {
                                None
                            } else {
                                Some(unsafe { NonZeroU32::new_unchecked(255) })
                            }
                        });
                        // Set puzzle limit
                    }
                }
            } else {
                // Otherwise game failed
                state.finished = Some(Err(GameOver::Fail));
            }
        }
    };
    let mut game = Game::with_gamemode(Gamemode::custom(
        "Puzzle".to_string(),
        SPEED_LEVEL,
        false,
        Some(Stat::Lines(total_lines)),
        Stat::Time(Duration::ZERO),
    ));
    game.set_modifier(Some(Box::new(game_modifier)));
    game
}
