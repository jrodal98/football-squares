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

#[derive(Debug, PartialEq, Eq)]
pub struct Coordinate {
    x: u64,
    y: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct WinningCoordinates {
    direct_hit: Coordinate,
    neighbors: Neighbors,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Neighbors {
    coordinates: Vec<Coordinate>,
    payout_type: PayoutType,
}

impl Neighbors {
    pub fn new(coordinates: Vec<Coordinate>) -> Self {
        let payout_type = PayoutType::from_n(coordinates.len());
        Self {
            coordinates,
            payout_type,
        }
    }
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
    pub fn get_winning_coordinates(&self, score: Score) -> WinningCoordinates {
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

        WinningCoordinates {
            direct_hit,
            neighbors,
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

    pub fn get_neighbors(&self) -> Neighbors {
        match self {
            // three way touch:
            Self { x: 0, y: 0 } => Neighbors::new(vec![
                Self { x: 0, y: 1 },
                Self { x: 1, y: 0 },
                Self { x: 1, y: 1 },
            ]),
            // three way touch:
            Self { x: 0, y: 9 } => Neighbors::new(vec![
                Self { x: 0, y: 8 },
                Self { x: 1, y: 8 },
                Self { x: 1, y: 9 },
            ]),
            // three way touch:
            Self { x: 9, y: 0 } => Neighbors::new(vec![
                Self { x: 8, y: 0 },
                Self { x: 8, y: 1 },
                Self { x: 9, y: 1 },
            ]),
            // three way touch:
            Self { x: 9, y: 9 } => Neighbors::new(vec![
                Self { x: 8, y: 8 },
                Self { x: 8, y: 9 },
                Self { x: 9, y: 8 },
            ]),
            // five way touch:
            Self { x: 0, y } => Neighbors::new(vec![
                Self { x: 0, y: *y - 1 },
                Self { x: 0, y: *y + 1 },
                Self { x: 1, y: *y - 1 },
                Self { x: 1, y: *y },
                Self { x: 1, y: *y + 1 },
            ]),
            Self { x: 9, y } => Neighbors::new(vec![
                Self { x: 8, y: *y - 1 },
                Self { x: 8, y: *y },
                Self { x: 8, y: *y + 1 },
                Self { x: 9, y: *y - 1 },
                Self { x: 9, y: *y + 1 },
            ]),
            // five way touch:
            Self { x, y: 0 } => Neighbors::new(vec![
                Self { x: *x - 1, y: 0 },
                Self { x: *x - 1, y: 1 },
                Self { x: *x, y: 1 },
                Self { x: *x + 1, y: 1 },
                Self { x: *x + 1, y: 0 },
            ]),
            // five way touch:
            Self { x, y: 9 } => Neighbors::new(vec![
                Self { x: *x - 1, y: 8 },
                Self { x: *x - 1, y: 9 },
                Self { x: *x, y: 8 },
                Self { x: *x + 1, y: 8 },
                Self { x: *x + 1, y: 9 },
            ]),

            Self { x, y } => Neighbors::new(vec![
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
            ]),
        }
    }
}
