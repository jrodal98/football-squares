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

impl Game {
    // pub fn calculate_payouts(
    //     &self,
    //     score: Score,
    //     event: Event,
    //     coordinates: Vec<Coordinate>,
    // ) -> HashMap<Coordinate, u64> {
    //     let winners = self.board.get_winners(score, event);
    // }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Payout {
    Integer(u32),
    Map(HashMap<Event, u32>),
}

impl Payout {
    fn get_payout(&self, event: Event) -> u32 {
        match self {
            Self::Integer(x) => *x,
            Self::Map(map) => *map.get(&event).unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BoardBlock {
    Board(Board),
    Map(HashMap<Event, Board>),
}

impl BoardBlock {
    pub fn get_winners(&self, score: Score, event: Event) -> Vec<WinningCoordinate> {
        match self {
            Self::Board(board) => board.get_winning_coordinates(score),
            Self::Map(map) => map.get(&event).unwrap().get_winning_coordinates(score),
        }
    }
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

#[derive(Debug, PartialEq, Serialize, Deserialize, Eq, Hash, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum PayoutType {
    DirectHit,
    ThreeWayTouch,
    FiveWayTouch,
    EightWayTouch,
}

impl PayoutType {
    fn from_n(n: usize) -> Self {
        match n {
            1 => Self::DirectHit,
            3 => Self::ThreeWayTouch,
            5 => Self::FiveWayTouch,
            8 => Self::EightWayTouch,
            _ => unreachable!("Only payout type for {} not supported", n),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Board {
    pub afc: [u64; 10],
    pub nfc: [u64; 10],
}

#[derive(Debug)]
pub struct Score {
    afc: u64,
    nfc: u64,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Coordinate {
    x: u64,
    y: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Neighbors {
    coordinates: Vec<Coordinate>,
    payout_type: PayoutType,
}

impl Score {
    pub fn new(afc: u64, nfc: u64) -> Self {
        Self {
            afc: afc % 10,
            nfc: nfc % 10,
        }
    }
}

impl Board {
    pub fn get_winning_coordinates(&self, score: Score) -> Vec<WinningCoordinate> {
        let x = self
            .afc
            .iter()
            .position(|&afc_score| afc_score == score.afc)
            .unwrap()
            .try_into()
            .unwrap();

        let y = self
            .nfc
            .iter()
            .position(|&nfc_score| nfc_score == score.nfc)
            .unwrap()
            .try_into()
            .unwrap();

        let direct_hit = Coordinate { x, y };
        let neighbors = direct_hit.get_neighbors();

        let payout_type = PayoutType::from_n(neighbors.len());

        neighbors.into_iter().fold(
            vec![WinningCoordinate {
                coordinate: direct_hit,
                payout_type: PayoutType::DirectHit,
            }],
            |mut acc, coordinate| {
                acc.push(WinningCoordinate {
                    coordinate,
                    payout_type: payout_type.clone(),
                });
                acc
            },
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct WinningCoordinate {
    pub coordinate: Coordinate,
    pub payout_type: PayoutType,
}

impl Coordinate {
    pub fn new(x: u64, y: u64) -> Self {
        if x > 9 || y > 9 {
            panic!("x and y cannot be greater than 9.")
        }
        Self { x, y }
    }

    pub fn get_neighbors(&self) -> Vec<Coordinate> {
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
