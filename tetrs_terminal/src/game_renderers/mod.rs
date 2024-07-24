pub mod cached;
pub mod immediate;
pub mod naive;

use std::io::{self, Write};

use tetrs_engine::{FeedbackEvent, Game, GameTime};

use crate::terminal_tetrs::{App, GameRunningStats};

pub trait GameScreenRenderer {
    fn render<T>(
        &mut self,
        app: &mut App<T>,
        game: &mut Game,
        action_stats: &mut GameRunningStats,
        new_feedback_events: Vec<(GameTime, FeedbackEvent)>,
        screen_resized: bool,
    ) -> io::Result<()>
    where
        T: Write;
}
