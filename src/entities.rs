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
