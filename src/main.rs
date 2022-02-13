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
    payout: PayoutBlock,
    grid: GridBlock,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum PayoutBlock {
    Map(HashMap<String, Payout>),
    // MapMap(HashMap<String, PayoutBlock>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum Payout {
    Integer(u32),
    Map(HashMap<String, u32>),
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum GridBlock {
    Grid(Grid),
    Map(HashMap<String, Grid>),
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
