use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GameBlock {
    Board(Game),
    Map(HashMap<String, Game>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Game {
    cost_per_square: u64,
    payout: HashMap<PayoutType, Payout>,
    board: BoardBlock,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Payout {
    Integer(u32),
    Map(HashMap<Event, u32>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BoardBlock {
    Board(Board),
    Map(HashMap<Event, Board>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Event {
    Quarter1,
    Warning1,
    Quarter2,
    Quarter3,
    Warning2,
    Quarter4,
    Final,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum PayoutType {
    DirectHit,
    ThreeWayTouch,
    FiveWayTouch,
    EightWayTouch,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Board {
    afc: [u32; 10],
    nfc: [u32; 10],
}

#[derive(Debug)]
pub struct Score {
    afc: u64,
    nfc: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Coordinate {
    x: u64,
    y: u64,
}

impl Score {
    fn new(afc: u64, nfc: u64) -> Self {
        Self {
            afc: afc % 10,
            nfc: nfc % 10,
        }
    }
}

impl Coordinate {
    pub fn new(x: u64, y: u64) -> Self {
        if x > 9 || y > 9 {
            panic!("x and y cannot be greater than 9.")
        }
        Self { x, y }
    }

    pub fn get_neighbors(&self) -> Vec<Self> {
        match self {
            // three way touch:
            Self { x: 0, y: 0 } => vec![
                Self { x: 0, y: 1 },
                Self { x: 1, y: 0 },
                Self { x: 1, y: 1 },
            ],
            // three way touch:
            Self { x: 0, y: 9 } => vec![
                Self { x: 0, y: 8 },
                Self { x: 1, y: 8 },
                Self { x: 1, y: 9 },
            ],
            // three way touch:
            Self { x: 9, y: 0 } => vec![
                Self { x: 8, y: 0 },
                Self { x: 8, y: 1 },
                Self { x: 9, y: 1 },
            ],
            // three way touch:
            Self { x: 9, y: 9 } => vec![
                Self { x: 8, y: 8 },
                Self { x: 8, y: 9 },
                Self { x: 9, y: 8 },
            ],
            // five way touch:
            Self { x: 0, y } => vec![
                Self { x: 0, y: *y - 1 },
                Self { x: 0, y: *y + 1 },
                Self { x: 1, y: *y - 1 },
                Self { x: 1, y: *y },
                Self { x: 1, y: *y + 1 },
            ],
            Self { x: 9, y } => vec![
                Self { x: 8, y: *y - 1 },
                Self { x: 8, y: *y },
                Self { x: 8, y: *y + 1 },
                Self { x: 9, y: *y - 1 },
                Self { x: 9, y: *y + 1 },
            ],
            // five way touch:
            Self { x, y: 0 } => vec![
                Self { x: *x - 1, y: 0 },
                Self { x: *x - 1, y: 1 },
                Self { x: *x, y: 1 },
                Self { x: *x + 1, y: 1 },
                Self { x: *x + 1, y: 0 },
            ],
            // five way touch:
            Self { x, y: 9 } => vec![
                Self { x: *x - 1, y: 8 },
                Self { x: *x - 1, y: 9 },
                Self { x: *x, y: 8 },
                Self { x: *x + 1, y: 8 },
                Self { x: *x + 1, y: 9 },
            ],

            Self { x, y } => vec![
                Self {
                    x: *x - 1,
                    y: *y - 1,
                },
                Self { x: *x - 1, y: *y },
                Self {
                    x: *x - 1,
                    y: *y + 1,
                },
                Self { x: *x, y: *y - 1 },
                Self { x: *x, y: *y + 1 },
                Self {
                    x: *x + 1,
                    y: *y - 1,
                },
                Self { x: *x + 1, y: *y },
                Self {
                    x: *x + 1,
                    y: *y + 1,
                },
            ],
        }
    }
}
