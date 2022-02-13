use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn main() {
    // let input = include_str!("../example.yml");
    let input = include_str!("../examples/different_payouts.yml");
    solution(input);
}

fn solution(input: &str) {
    let game = serde_yaml::from_str::<GameBlock>(input).unwrap();
    dbg!(game);
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum GameBlock {
    Board(Game),
    Map(HashMap<String, Game>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Game {
    cost_per_square: u64,
    payout: HashMap<PayoutType, Payout>,
    board: BoardBlock,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum Payout {
    Integer(u32),
    Map(HashMap<Event, u32>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum BoardBlock {
    Board(Board),
    Map(HashMap<Event, Board>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[serde(rename_all = "snake_case")]
enum Event {
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
enum PayoutType {
    DirectHit,
    ThreeWayTouch,
    FiveWayTouch,
    EightWayTouch,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Board {
    afc: [u32; 10],
    nfc: [u32; 10],
}

#[test]
fn test_simple_board() {
    let board_str = include_str!("../examples/simple_board.yml");
    serde_yaml::from_str::<GameBlock>(board_str).unwrap();
}

#[test]
fn test_different_payouts() {
    let board_str = include_str!("../examples/different_payouts.yml");
    serde_yaml::from_str::<GameBlock>(board_str).unwrap();
}

#[test]
fn test_different_board() {
    let board_str = include_str!("../examples/different_boards.yml");
    serde_yaml::from_str::<GameBlock>(board_str).unwrap();
}

#[test]
fn test_multiple_games() {
    let board_str = include_str!("../examples/different_boards.yml");
    serde_yaml::from_str::<GameBlock>(board_str).unwrap();
}
