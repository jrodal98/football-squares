mod entities;
mod test;

use std::collections::HashMap;

use entities::{Coordinate, Event, EventSummary, GameBlock, PlayerEventSummary, PlayersBlock, Score};

fn main() {
    let board_str = include_str!("../examples/complex.yml");
    let game_block = serde_yaml::from_str::<GameBlock>(board_str).unwrap();
    let players_str = include_str!("../examples/multiple_players.yml");
    let players_block = serde_yaml::from_str::<PlayersBlock>(players_str).unwrap();

    // quarter 1
    let score = Score::new(8, 9);
    let event = Event::Quarter1;

    let summaries =
        PlayerEventSummary::winners_only(players_block.summarize_event(&game_block, score, event));
    dbg!(summaries);

    // let total_won = summaries.iter().map(|e| e.amount_won).sum::<u64>();
    // dbg!(summaries);
    // dbg!(total_won);
}
