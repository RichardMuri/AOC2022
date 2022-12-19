use std::fmt;
use std::str::FromStr;

// Define our error types. These may be customized for our error handling cases.
// Now we will be able to write our own errors, defer to an underlying error
// implementation, or do something in between.
#[derive(Debug, Clone)]
pub struct RpsError;

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for RpsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown rock, paper, scissors encoding.")
    }
}

pub enum RpsResult {
    Win,
    Lose,
    Draw,
}

impl FromStr for RpsResult {
    type Err = RpsError;

    fn from_str(input: &str) -> Result<RpsResult, Self::Err> {
        match input {
            "X" => Ok(RpsResult::Lose),
            "Y" => Ok(RpsResult::Draw),
            "Z" => Ok(RpsResult::Win),
            _ => Err(RpsError),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

trait Beats {
    fn beats(&self) -> Self;
}

impl Beats for RPS {
    fn beats(&self) -> Self {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }
}

trait WinLoseDraw {
    fn winlosedraw(&self, opponent: &RPS) -> RPS;
}

impl WinLoseDraw for RpsResult {
    fn winlosedraw(&self, opponent: &RPS) -> RPS {
        match self {
            Self::Win => opponent.beats().beats(),
            Self::Lose => opponent.beats(),
            Self::Draw => opponent.clone(),
        }
    }
}

impl RPS {
    fn points(&self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
            _ => 0,
        }
    }

    pub fn play(me: &RPS, opponent: &RPS) -> i32 {
        let self_beats = me.beats();
        let opponent_beats = opponent.beats();

        if self_beats == *opponent {
            me.points() + 6
        } else if opponent_beats == *me {
            me.points()
        } else {
            me.points() + 3
        }
    }

    pub fn alternate_score(opponent: &RPS, game_result: &RpsResult) -> i32 {
        let me = game_result.winlosedraw(opponent);
        RPS::play(&me, opponent)
    }
}

impl FromStr for RPS {
    type Err = RpsError;

    fn from_str(input: &str) -> Result<RPS, Self::Err> {
        match input {
            "A" => Ok(RPS::Rock),
            "X" => Ok(RPS::Rock),
            "B" => Ok(RPS::Paper),
            "Y" => Ok(RPS::Paper),
            "C" => Ok(RPS::Scissors),
            "Z" => Ok(RPS::Scissors),
            _ => Err(RpsError),
        }
    }
}

impl fmt::Display for RPS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RPS::Rock => write!(f, "Rock"),
            RPS::Paper => write!(f, "Paper"),
            RPS::Scissors => write!(f, "Scissors"),
            _ => write!(f, "?"),
        }
    }
}
