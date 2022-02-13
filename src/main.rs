mod entities;
mod test;

use entities::{
    Event, GameBlock, PlayerEventSummary, PlayerGameSummary, PlayersBlock, Score, ScoreEvent,
};

fn main() {
    let board_str = include_str!("../examples/complex.yml");
    let game_block = serde_yaml::from_str::<GameBlock>(board_str).unwrap();
    let players_str = include_str!("../examples/multiple_players.yml");
    let players_block = serde_yaml::from_str::<PlayersBlock>(players_str).unwrap();

    let score_events = vec![ScoreEvent::new(Score::new(8, 9), Event::Quarter1)];

    let player_event_summaries = score_events
        .into_iter()
        .fold(vec![], |mut acc, score_event| {
            let mut event_summaries = PlayerEventSummary::winners_only(
                players_block.summarize_event(&game_block, score_event.score, score_event.event),
            );
            acc.append(&mut event_summaries);
            acc
        });

    let game_summary = PlayerGameSummary::summarize_player_events(player_event_summaries);
    dbg!(game_summary);
}
