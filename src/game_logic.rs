use crate::{
    tetromino_generators,
    rotation_systems,
};

use std::{
    collections::VecDeque, num::NonZeroU64, time::{Duration, Instant}
};

pub type ButtonChange = ButtonMap<Option<bool>>;
pub type Board = [[Option<TileTypeID>; Game::WIDTH]; Game::HEIGHT]; // NOTE `type Game::Board`... https://github.com/rust-lang/rust/issues/8995
pub type Coord = (usize,usize);
pub type TileTypeID = u32;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Orientation {
    N, E, S, W
}

impl Orientation {
    pub fn rotate_r(&self, right_turns: i32) -> Self {
        use Orientation::*;
        let base = match self {
            N => 0, E => 1, S => 2, W => 3, 
        };
        match (base + right_turns).rem_euclid(4) {
            0 => N, 1 => E, 2 => S, 3 => W, _ => unreachable!()
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Tetromino {
    O, I, S, Z, T, L, J
}

impl TryFrom<usize> for Tetromino {
    type Error = ();

    fn try_from(n: usize) -> Result<Self, Self::Error> {
        use Tetromino::*;
        Ok(match n {
            0 => O, 1 => I, 2 => S, 3 => Z,
            4 => T, 5 => L, 6 => J, _ => Err(())?,
        })
    }
}

pub(crate) struct ActivePiece(pub Tetromino, pub Orientation, pub Coord);

impl ActivePiece {
    // Given a piece, return a list of (x,y) mino positions
    pub fn minos(&self) -> [Coord; 4] {
        let Self(shape, o, (x,y)) = self;
        use Orientation::*;
        match shape {
            Tetromino::O => [(0,0),(1,0),(0,1),(1,1)], // ⠶
            Tetromino::I => match o {
                N | S => [(0,0),(1,0),(2,0),(3,0)], // ⠤⠤
                E | W => [(0,0),(0,1),(0,2),(0,3)], // ⡇
            },
            Tetromino::S => match o {
                N | S => [(0,0),(1,0),(1,1),(2,1)], // ⠴⠂
                E | W => [(1,0),(0,1),(1,1),(0,2)], // ⠳
            },
            Tetromino::Z => match o {
                N | S => [(1,0),(2,0),(0,1),(1,1)], // ⠲⠄
                E | W => [(0,0),(0,1),(1,1),(1,2)], // ⠞
            },
            Tetromino::T => match o {
                N => [(0,0),(1,0),(2,0),(1,1)], // ⠴⠄
                E => [(0,0),(0,1),(1,1),(0,2)], // ⠗
                S => [(1,0),(0,1),(1,1),(2,1)], // ⠲⠂
                W => [(1,0),(0,1),(1,1),(1,2)], // ⠺
            },
            Tetromino::L => match o {
                N => [(0,0),(1,0),(2,0),(2,1)], // ⠤⠆
                E => [(0,0),(1,0),(0,1),(0,2)], // ⠧
                S => [(0,0),(0,1),(1,1),(2,1)], // ⠖⠂
                W => [(1,0),(1,1),(0,2),(1,2)], // ⠹
            },
            Tetromino::J => match o {
                N => [(0,0),(1,0),(2,0),(0,1)], // ⠦⠄
                E => [(0,0),(0,1),(0,2),(1,2)], // ⠏
                S => [(2,0),(0,1),(1,1),(2,1)], // ⠒⠆
                W => [(0,0),(1,0),(1,1),(1,2)], // ⠼
            },
        }.map(|(dx,dy)| (x+dx,y+dy))
    }

    pub(crate) fn fits(&self, board: Board) -> bool {
        self.minos().iter().all(|&(x,y)| x < Game::WIDTH && y < Game::HEIGHT && board[y][x].is_none())
    }
    
}

#[derive(PartialEq, PartialOrd)]
pub enum Stat {
    Lines(u64),
    Level(u64),
    Score(u64),
    Pieces(u64),
    Time(Duration),
}

pub struct Gamemode {
    name: String,
    start_level: u64,
    increase_level: bool,
    mode_limit: Option<Stat>,
    optimize_goal: Stat,
}

impl Gamemode {
    pub const fn custom(name: String, start_level: NonZeroU64, increase_level: bool, mode_limit: Option<Stat>, optimize_goal: Stat) -> Self {
        let start_level = start_level.get();
        Self {
            name,
            start_level,
            increase_level,
            mode_limit,
            optimize_goal,
        }
    }

    pub fn sprint(start_level: NonZeroU64) -> Self {
        let start_level = start_level.get();
        Self {
            name: String::from("Sprint"),
            start_level,
            increase_level: false,
            mode_limit: Some(Stat::Lines(40)),
            optimize_goal: Stat::Time(Duration::ZERO),
        }
    }

    pub fn ultra(start_level: NonZeroU64) -> Self {
        let start_level = start_level.get();
        Self {
            name: String::from("Ultra"),
            start_level,
            increase_level: false,
            mode_limit: Some(Stat::Time(Duration::from_secs(3*60))),
            optimize_goal: Stat::Lines(0),
        }
    }

    pub fn marathon() -> Self {
        Self {
            name: String::from("Marathon"),
            start_level: 1,
            increase_level: true,
            mode_limit: Some(Stat::Level(15)),
            optimize_goal: Stat::Score(0),
        }
    }

    pub fn endless() -> Self {
        Self {
            name: String::from("Endless"),
            start_level: 1,
            increase_level: true,
            mode_limit: None,
            optimize_goal: Stat::Score(0),
        }
    }
    //TODO Gamemode pub fn master() -> Self : 20G gravity mode...
    //TODO Gamemode pub fn increment() -> Self : regain time to keep playing...
    //TODO Gamemode pub fn finesse() -> Self : minimize Finesse(u64) for certain linecount...
}


#[derive(Default, Debug)]
pub struct ButtonMap<T> {
    move_left: T,
    move_right: T,
    rotate_left: T,
    rotate_right: T,
    drop_soft: T,
    drop_hard: T,
    rotate_180: T,
    hold: T,
}

enum GameState {
    Over,
    Fall,
    Clearing,
    //TODO Complete necessary states (keep in mind timing purposes)
}

// Stores the complete game state at a given instant.
pub struct Game {
    // Settings internal
    mode: Gamemode,
    time_started: Instant,
    last_updated: Instant,
    piece_generator: Box<dyn Iterator<Item=Tetromino>>,
    rotate_fn: rotation_systems::RotateFn,
    preview_size: usize,
    //TODO soft_drop_factor=20, lock_delay=0.5s etc.. c.f Notes_Tetrs
    // State
    state: GameState,
    buttons_pressed: ButtonMap<bool>,
    board: Board,
    active_piece: Option<ActivePiece>,
    preview_pieces: VecDeque<Tetromino>,
    // Statistics
    lines_cleared: u64,
    level: u64,
    score: u64,
}

impl Game {
    pub const HEIGHT: usize = 22;
    pub const WIDTH: usize = 10;

    pub fn new(mode: Gamemode) -> Self {
        let time_started = Instant::now();
        let mut generator = tetromino_generators::RecencyProbGen::new();
        let preview_size = 1;
        let preview_pieces = generator.by_ref().take(preview_size).collect();
        Game {
            mode,
            time_started,
            last_updated: time_started,
            piece_generator: Box::new(generator),
            rotate_fn: rotation_systems::rotate_classic,
            preview_size,
            
            state: GameState::Clearing,
            buttons_pressed: ButtonMap::default(),
            board: Default::default(),
            active_piece: None,
            preview_pieces,
            
            lines_cleared: 0,
            level: 0,
            score: 0,
        }
    }

    pub fn get_visuals<'a>(&'a self) -> (&'a Board, Option<[Coord; 4]>, Option<[Coord; 4]>, &VecDeque<Tetromino>) {
        (
            &self.board,
            self.active_piece.as_ref().map(|p| p.minos()),
            self.ghost_piece(),
            &self.preview_pieces,
            // TODO Return current GameState, timeinterval (so we can render e.g. lineclears with intermediate states),
        )
    }

    pub fn get_stats<'a>(&'a self) -> (&'a Gamemode, u64, u64, u64, Instant) {
        (
            &self.mode,
            self.lines_cleared,
            self.level,
            self.score,
            self.time_started,
        )
    }

    fn ghost_piece(&self) -> Option<[Coord; 4]> {
        todo!() // TODO compute ghost piece
    }

    fn level_from_lineclears(lns: u64) {
        todo!() // TODO 10ln / level?
    }

    fn droptime(lvl: u64) -> Duration {
        Duration::from_nanos(match lvl {
            1  => 1000000000,
            2  =>  793000000,
            3  =>  617796000,
            4  =>  472729139,
            5  =>  355196928,
            6  =>  262003550,
            7  =>  189677245,
            8  =>  134734731,
            9  =>   93882249,
            10 =>   64151585,
            11 =>   42976258,
            12 =>   28217678,
            13 =>   18153329,
            14 =>   11439342,
            15 =>    7058616,
            16 =>    4263557,
            17 =>    2520084,
            18 =>    1457139,
            19 =>     823907,
            _ => unimplemented!(),
        })
    }

    fn update(&mut self, interaction: Option<ButtonChange>, up_to: Instant) {
        todo!() // TODO Complete state machine.
        
        // Handle game over: return immediately
        // 
        // Spawn piece
        // Move piece
        // Drop piece
        // Check pattern (lineclear)
        // Update score (B2B?? Combos?? Perfect clears??)
        // Update level
        // Return desired next update

    }
}