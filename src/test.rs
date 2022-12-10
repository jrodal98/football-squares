#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use crate::entities::{
        Board, Coordinate, Event, EventSummary, Game, GameBlock, PayoutType, PlayerEventSummary,
        PlayersBlock, Score, WinningCoordinate,
    };

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
        let board_str = include_str!("../examples/multiple_games.yml");
        serde_yaml::from_str::<GameBlock>(board_str).unwrap();
    }

    #[test]
    fn test_complex() {
        let board_str = include_str!("../examples/complex.yml");
        serde_yaml::from_str::<GameBlock>(board_str).unwrap();
    }

    #[test]
    fn test_simple_player() {
        let players_str = include_str!("../examples/simple_player.yml");
        serde_yaml::from_str::<PlayersBlock>(players_str).unwrap();
    }

    #[test]
    fn test_multiple_players() {
        let players_str = include_str!("../examples/multiple_players.yml");
        serde_yaml::from_str::<PlayersBlock>(players_str).unwrap();
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
        assert_eq!(board.get_winning_coordinates(&score), winning_coordinates);
    }

    #[test]
    fn test_simple_board_winner_values() {
        let board_str = include_str!("../examples/simple_board.yml");
        let game = serde_yaml::from_str::<Game>(board_str).unwrap();
        let score = Score::new(32, 48);
        let event = Event::Quarter1;

        let mut expected_winners = HashMap::new();

        expected_winners.insert(Coordinate::new(5, 1), 125);

        assert_eq!(
            game.calculate_winner_values(&score, &event),
            expected_winners
        );
    }

    #[test]
    fn test_different_payouts_winner_values() {
        let board_str = include_str!("../examples/different_payouts.yml");
        let game = serde_yaml::from_str::<Game>(board_str).unwrap();

        let score = Score::new(32, 48);
        let event = Event::Quarter1;
        let direct_hit = Coordinate::new(5, 1);
        let mut expected_winners =
            direct_hit
                .get_neighbors()
                .into_iter()
                .fold(HashMap::new(), |mut acc, coordinate| {
                    acc.insert(coordinate, 55);
                    acc
                });
        expected_winners.insert(direct_hit, 400);
        assert_eq!(
            game.calculate_winner_values(&score, &event),
            expected_winners
        );

        let score = Score::new(32, 48);
        let event = Event::Warning1;
        let direct_hit = Coordinate::new(5, 1);
        let mut expected_winners =
            direct_hit
                .get_neighbors()
                .into_iter()
                .fold(HashMap::new(), |mut acc, coordinate| {
                    acc.insert(coordinate, 55);
                    acc
                });
        expected_winners.insert(direct_hit, 100);
        assert_eq!(
            game.calculate_winner_values(&score, &event),
            expected_winners
        );

        let score = Score::new(32, 48);
        let event = Event::Quarter2;
        let direct_hit = Coordinate::new(5, 1);
        let mut expected_winners =
            direct_hit
                .get_neighbors()
                .into_iter()
                .fold(HashMap::new(), |mut acc, coordinate| {
                    acc.insert(coordinate, 55);
                    acc
                });
        expected_winners.insert(direct_hit, 400);
        assert_eq!(
            game.calculate_winner_values(&score, &event),
            expected_winners
        );

        let score = Score::new(32, 48);
        let event = Event::Quarter3;
        let direct_hit = Coordinate::new(5, 1);
        let mut expected_winners =
            direct_hit
                .get_neighbors()
                .into_iter()
                .fold(HashMap::new(), |mut acc, coordinate| {
                    acc.insert(coordinate, 55);
                    acc
                });
        expected_winners.insert(direct_hit, 400);
        assert_eq!(
            game.calculate_winner_values(&score, &event),
            expected_winners
        );

        let score = Score::new(32, 48);
        let event = Event::Warning2;
        let direct_hit = Coordinate::new(5, 1);
        let mut expected_winners =
            direct_hit
                .get_neighbors()
                .into_iter()
                .fold(HashMap::new(), |mut acc, coordinate| {
                    acc.insert(coordinate, 55);
                    acc
                });
        expected_winners.insert(direct_hit, 100);
        assert_eq!(
            game.calculate_winner_values(&score, &event),
            expected_winners
        );

        let score = Score::new(32, 48);
        let event = Event::Quarter4;
        let direct_hit = Coordinate::new(5, 1);
        let mut expected_winners =
            direct_hit
                .get_neighbors()
                .into_iter()
                .fold(HashMap::new(), |mut acc, coordinate| {
                    acc.insert(coordinate, 55);
                    acc
                });
        expected_winners.insert(direct_hit, 400);
        assert_eq!(
            game.calculate_winner_values(&score, &event),
            expected_winners
        );

        let score = Score::new(32, 48);
        let event = Event::Final;
        let direct_hit = Coordinate::new(5, 1);
        let mut expected_winners =
            direct_hit
                .get_neighbors()
                .into_iter()
                .fold(HashMap::new(), |mut acc, coordinate| {
                    acc.insert(coordinate, 55);
                    acc
                });
        expected_winners.insert(direct_hit, 600);
        assert_eq!(
            game.calculate_winner_values(&score, &event),
            expected_winners
        );
    }

    #[test]
    fn test_different_boards_winner_values() {
        let board_str = include_str!("../examples/different_boards.yml");
        let game = serde_yaml::from_str::<Game>(board_str).unwrap();

        let score = Score::new(32, 48);
        let event = Event::Quarter1;
        let mut expected_winners = HashMap::new();
        expected_winners.insert(Coordinate::new(7, 0), 800);
        assert_eq!(
            game.calculate_winner_values(&score, &event),
            expected_winners
        );

        let score = Score::new(32, 48);
        let event = Event::Quarter2;
        let mut expected_winners = HashMap::new();
        expected_winners.insert(Coordinate::new(9, 2), 800);
        assert_eq!(
            game.calculate_winner_values(&score, &event),
            expected_winners
        );

        let score = Score::new(32, 48);
        let event = Event::Quarter3;
        let mut expected_winners = HashMap::new();
        expected_winners.insert(Coordinate::new(4, 3), 800);
        assert_eq!(
            game.calculate_winner_values(&score, &event),
            expected_winners
        );

        let score = Score::new(32, 48);
        let event = Event::Final;
        let mut expected_winners = HashMap::new();
        expected_winners.insert(Coordinate::new(7, 8), 800);
        assert_eq!(
            game.calculate_winner_values(&score, &event),
            expected_winners
        );

        let score = Score::new(32, 48);
        let event = Event::Warning1;
        let expected_winners = HashMap::new();
        assert_eq!(
            game.calculate_winner_values(&score, &event),
            expected_winners
        );
    }

    #[test]
    fn test_get_winner_values_by_game_multiple_games() {
        let board_str = include_str!("../examples/multiple_games.yml");
        let game_block = serde_yaml::from_str::<GameBlock>(board_str).unwrap();

        let score = Score::new(32, 48);
        let event = Event::Quarter1;

        let mut expected_winners1 = HashMap::new();
        expected_winners1.insert(Coordinate::new(5, 1), 125);

        let mut expected_winners2 = HashMap::new();
        expected_winners2.insert(Coordinate::new(5, 1), 125);

        let mut expected_result = HashMap::new();
        expected_result.insert("game1".to_string(), expected_winners1);
        expected_result.insert("game2".to_string(), expected_winners2);

        assert_eq!(
            game_block.get_winner_values_per_game(&score, &event),
            expected_result
        );
    }

    #[test]
    fn test_get_winner_values_by_game_simple_board() {
        let board_str = include_str!("../examples/simple_board.yml");
        let game_block = serde_yaml::from_str::<GameBlock>(board_str).unwrap();

        let score = Score::new(32, 48);
        let event = Event::Quarter1;

        let mut expected_winners1 = HashMap::new();
        expected_winners1.insert(Coordinate::new(5, 1), 125);

        let mut expected_result = HashMap::new();
        expected_result.insert("game1".to_string(), expected_winners1);

        assert_eq!(
            game_block.get_winner_values_per_game(&score, &event),
            expected_result
        );
    }

    #[test]
    fn test_summarize_events() {
        let board_str = include_str!("../examples/multiple_games.yml");
        let game_block = serde_yaml::from_str::<GameBlock>(board_str).unwrap();

        let score = Score::new(32, 48);
        let event = Event::Quarter1;

        let mut coordinates = HashMap::new();
        // duplicates should be filtered out
        let game1_coordinates = vec![Coordinate::new(5, 1), Coordinate::new(5, 1)];
        let game2_coordinates = vec![Coordinate::new(5, 2)];
        coordinates.insert("game1".to_string(), game1_coordinates);
        coordinates.insert("game2".to_string(), game2_coordinates);
        let expected_summary = EventSummary {
            event: Event::Quarter1,
            games_won: vec!["game1".to_string()],
            amount_won: 125,
        };

        assert_eq!(
            game_block.summarize_event(&score, &event, &coordinates),
            expected_summary
        );
    }

    #[test]
    fn test_summarize_player_simple() {
        let board_str = include_str!("../examples/complex.yml");
        let game_block = serde_yaml::from_str::<GameBlock>(board_str).unwrap();
        let players_str = include_str!("../examples/simple_player.yml");
        let players_block = serde_yaml::from_str::<PlayersBlock>(players_str).unwrap();

        let score = Score::new(6, 12);
        let event = Event::Quarter1;

        let expected_event_summary = EventSummary {
            event: Event::Quarter1,
            games_won: vec!["game1".to_string(), "game3".to_string()],
            amount_won: 525,
        };
        let expected_player_summary = vec![PlayerEventSummary {
            player_name: "player1".to_string(),
            event_summary: expected_event_summary,
        }];

        assert_eq!(
            players_block.summarize_event(&game_block, score, event),
            expected_player_summary
        );
    }

    #[test]
    fn test_summarize_multiple_players() {
        let board_str = include_str!("../examples/complex.yml");
        let game_block = serde_yaml::from_str::<GameBlock>(board_str).unwrap();
        let players_str = include_str!("../examples/multiple_players.yml");
        let players_block = serde_yaml::from_str::<PlayersBlock>(players_str).unwrap();

        let score = Score::new(6, 12);
        let event = Event::Quarter1;

        let expected_player_summary = vec![
            PlayerEventSummary {
                player_name: "becca".to_string(),
                event_summary: EventSummary {
                    event: Event::Quarter1,
                    games_won: vec![],
                    amount_won: 0,
                },
            },
            PlayerEventSummary {
                player_name: "candy".to_string(),
                event_summary: EventSummary {
                    event: Event::Quarter1,
                    games_won: vec![],
                    amount_won: 0,
                },
            },
            PlayerEventSummary {
                player_name: "jake".to_string(),
                event_summary: EventSummary {
                    event: Event::Quarter1,
                    games_won: vec!["game1".to_string(), "game3".to_string()],
                    amount_won: 525,
                },
            },
            PlayerEventSummary {
                player_name: "jc".to_string(),
                event_summary: EventSummary {
                    event: Event::Quarter1,
                    games_won: vec![],
                    amount_won: 0,
                },
            },
            PlayerEventSummary {
                player_name: "john".to_string(),
                event_summary: EventSummary {
                    event: Event::Quarter1,
                    games_won: vec![],
                    amount_won: 0,
                },
            },
            PlayerEventSummary {
                player_name: "lily".to_string(),
                event_summary: EventSummary {
                    event: Event::Quarter1,
                    games_won: vec![],
                    amount_won: 0,
                },
            },
            PlayerEventSummary {
                player_name: "pebbles".to_string(),
                event_summary: EventSummary {
                    event: Event::Quarter1,
                    games_won: vec![],
                    amount_won: 0,
                },
            },
        ];

        let players_summaries = players_block.summarize_event(&game_block, score, event);
        assert_eq!(&players_summaries, &expected_player_summary);

        assert_eq!(
            PlayerEventSummary::winners_only(players_summaries),
            vec![PlayerEventSummary {
                player_name: "jake".to_string(),
                event_summary: EventSummary {
                    event: Event::Quarter1,
                    games_won: vec!["game1".to_string(), "game3".to_string()],
                    amount_won: 525,
                },
            }]
        )
    }
}
