use std::fmt::Display;

use rand::Rng;

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum Play {
    ShortRun,
    LongRun,
    ShortPass,
    LongPass,
    TrickPlay,
    HailMary,
    FieldGoal,
    Punt,
    ExtraPoint,
    TwoPoint,
}

impl Play {
    pub fn get_rand_play() -> Play {
        Play::from(rand::thread_rng().gen_range(0..5))
    }
}

impl Display for Play {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_text = match *self {
            Play::ShortRun => "Short Run",
            Play::LongRun => "Long Run",
            Play::ShortPass => "Short Pass",
            Play::LongPass => "Long Pass",
            Play::TrickPlay => "Trick Play",
            Play::HailMary => "Hail Mary",
            Play::FieldGoal => "Field Goal",
            Play::Punt => "Punt",
            Play::ExtraPoint => "Extra Point",
            Play::TwoPoint => "Two Point",
        };

        write!(f, "{}", display_text)
    }
}

impl From<u8> for Play {
    fn from(num: u8) -> Self {
        match num {
            0 => Play::ShortRun,
            1 => Play::LongRun,
            2 => Play::ShortPass,
            3 => Play::LongPass,
            4 => Play::TrickPlay,
            5 => Play::HailMary,
            6 => Play::FieldGoal,
            7 => Play::Punt,
            8 => Play::ExtraPoint,
            9 => Play::TwoPoint,
            _ => unreachable!("You shouldn't be here! Invalid play number: {}!", num),
        }
    }
}

pub struct PlayCard {
    pub play: Play,
    pub count: u32,
}

impl PlayCard {
    pub fn new(play: Play, ot: bool) -> Self {
        let num = match play {
            Play::TrickPlay => 1,
            Play::HailMary if ot => 2,
            _ => 3,
        };
        Self { play, count: num }
    }
}

impl Default for PlayCard {
    fn default() -> Self {
        Self {
            play: Play::ShortRun,
            count: 3,
        }
    }
}
