#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    User,
    Opponent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hand {
    PAPER    = 0,
    ROCK     = 1,
    SCISSORS = 2,
    UNKNOWN  = 3,
}
