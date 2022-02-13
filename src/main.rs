use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn main() {
    let input = include_str!("../example.yml");
    solution(input);
}

fn solution(input: &str) {
    println!("{}", input)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Board {
    cost_per_square: u64,
    payout: HashMap<PayoutType, Payout>,
    grid: GridBlock,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum Payout {
    Integer(u32),
    Map(HashMap<Event, u32>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum GridBlock {
    Grid(Grid),
    Map(HashMap<Event, Grid>),
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
struct Grid {
    afc: [u32; 10],
    nfc: [u32; 10],
}

#[test]
fn test_read_simple_board() {
    let board_str = include_str!("../examples/simple_board.yml");
    let board: Board = serde_yaml::from_str(board_str).unwrap();
}

#[test]
fn test_read_different_payouts_board() {
    let board_str = include_str!("../examples/different_payouts.yml");
    let board: Board = serde_yaml::from_str(board_str).unwrap();
}

#[test]
fn test_read_different_grid_board() {
    let board_str = include_str!("../examples/different_grids.yml");
    let board: Board = serde_yaml::from_str(board_str).unwrap();
}
