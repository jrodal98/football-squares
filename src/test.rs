use crate::entities::GameBlock;

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
