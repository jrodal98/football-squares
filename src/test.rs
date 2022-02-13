use crate::entities::{Board, Coordinate, GameBlock, PayoutType, Score, WinningCoordinate};

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

#[test]
fn test_complex() {
    let board_str = include_str!("../examples/complex.yml");
    serde_yaml::from_str::<GameBlock>(board_str).unwrap();
}

#[test]
fn test_coordinate_maximums() {
    assert!(std::panic::catch_unwind(|| Coordinate::new(10, 4)).is_err());
    assert!(std::panic::catch_unwind(|| Coordinate::new(3, 10)).is_err());
    assert!(std::panic::catch_unwind(|| Coordinate::new(10, 10)).is_err());
    assert!(std::panic::catch_unwind(|| Coordinate::new(9, 9)).is_ok());
}

#[test]
fn test_get_neighbors_three_touch() {
    let c1 = Coordinate::new(0, 0);
    let c1_neighbors = vec![
        Coordinate::new(0, 1),
        Coordinate::new(1, 0),
        Coordinate::new(1, 1),
    ];
    assert_eq!(c1.get_neighbors(), c1_neighbors);

    let c2 = Coordinate::new(0, 9);
    let c2_neighbors = vec![
        Coordinate::new(0, 8),
        Coordinate::new(1, 8),
        Coordinate::new(1, 9),
    ];
    assert_eq!(c2.get_neighbors(), c2_neighbors);

    let c3 = Coordinate::new(9, 0);
    let c3_neighbors = vec![
        Coordinate::new(8, 0),
        Coordinate::new(8, 1),
        Coordinate::new(9, 1),
    ];
    assert_eq!(c3.get_neighbors(), c3_neighbors);

    let c4 = Coordinate::new(9, 9);
    let c4_neighbors = vec![
        Coordinate::new(8, 8),
        Coordinate::new(8, 9),
        Coordinate::new(9, 8),
    ];
    assert_eq!(c4.get_neighbors(), c4_neighbors);
}

#[test]
fn test_get_neighbors_five_touch() {
    let c1 = Coordinate::new(5, 0);
    let c1_neighbors = vec![
        Coordinate::new(4, 0),
        Coordinate::new(4, 1),
        Coordinate::new(5, 1),
        Coordinate::new(6, 1),
        Coordinate::new(6, 0),
    ];
    assert_eq!(c1.get_neighbors(), c1_neighbors);

    let c2 = Coordinate::new(0, 7);
    let c2_neighbors = vec![
        Coordinate::new(0, 6),
        Coordinate::new(0, 8),
        Coordinate::new(1, 6),
        Coordinate::new(1, 7),
        Coordinate::new(1, 8),
    ];
    assert_eq!(c2.get_neighbors(), c2_neighbors);

    let c3 = Coordinate::new(9, 3);
    let c3_neighbors = vec![
        Coordinate::new(8, 2),
        Coordinate::new(8, 3),
        Coordinate::new(8, 4),
        Coordinate::new(9, 2),
        Coordinate::new(9, 4),
    ];
    assert_eq!(c3.get_neighbors(), c3_neighbors);

    let c4 = Coordinate::new(4, 9);
    let c4_neighbors = vec![
        Coordinate::new(3, 8),
        Coordinate::new(3, 9),
        Coordinate::new(4, 8),
        Coordinate::new(5, 8),
        Coordinate::new(5, 9),
    ];
    assert_eq!(c4.get_neighbors(), c4_neighbors);
}

#[test]
fn test_get_neighbors_eight_touch() {
    let c = Coordinate::new(4, 7);
    let c_neighbors = vec![
        Coordinate::new(3, 6),
        Coordinate::new(3, 7),
        Coordinate::new(3, 8),
        Coordinate::new(4, 6),
        Coordinate::new(4, 8),
        Coordinate::new(5, 6),
        Coordinate::new(5, 7),
        Coordinate::new(5, 8),
    ];
    assert_eq!(c.get_neighbors(), c_neighbors);
}

#[test]
fn test_get_winning_coordinates() {
    let afc = [0, 8, 9, 6, 3, 2, 5, 1, 4, 7];
    let nfc = [4, 8, 5, 0, 3, 1, 9, 6, 2, 7];
    let board = Board { afc, nfc };
    let score = Score::new(32, 48);
    let winning_coordinates = vec![
        WinningCoordinate {
            coordinate: Coordinate::new(5, 1),
            payout_type: PayoutType::DirectHit,
        },
        WinningCoordinate {
            coordinate: Coordinate::new(4, 0),
            payout_type: PayoutType::EightWayTouch,
        },
        WinningCoordinate {
            coordinate: Coordinate::new(4, 1),
            payout_type: PayoutType::EightWayTouch,
        },
        WinningCoordinate {
            coordinate: Coordinate::new(4, 2),
            payout_type: PayoutType::EightWayTouch,
        },
        WinningCoordinate {
            coordinate: Coordinate::new(5, 0),
            payout_type: PayoutType::EightWayTouch,
        },
        WinningCoordinate {
            coordinate: Coordinate::new(5, 2),
            payout_type: PayoutType::EightWayTouch,
        },
        WinningCoordinate {
            coordinate: Coordinate::new(6, 0),
            payout_type: PayoutType::EightWayTouch,
        },
        WinningCoordinate {
            coordinate: Coordinate::new(6, 1),
            payout_type: PayoutType::EightWayTouch,
        },
        WinningCoordinate {
            coordinate: Coordinate::new(6, 2),
            payout_type: PayoutType::EightWayTouch,
        },
    ];
    assert_eq!(board.get_winning_coordinates(score), winning_coordinates);
}
