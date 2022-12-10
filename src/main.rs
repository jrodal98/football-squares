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

    let score_events = vec![
        ScoreEvent::new(Score::new(3, 7), Event::Quarter1),
        ScoreEvent::new(Score::new(10, 13), Event::Warning1),
        ScoreEvent::new(Score::new(10, 13), Event::Quarter2),
        ScoreEvent::new(Score::new(20, 16), Event::Quarter3),
        ScoreEvent::new(Score::new(20, 16), Event::Warning2),
        ScoreEvent::new(Score::new(20, 23), Event::Quarter4),
        ScoreEvent::new(Score::new(20, 23), Event::Final),
    ];

    let player_event_summaries = score_events
        .into_iter()
        .flat_map(|score_event| {
            PlayerEventSummary::winners_only(players_block.summarize_event(
                &game_block,
                score_event.score,
                score_event.event,
            ))
        })
        .collect();

    let game_summary = PlayerGameSummary::summarize_player_events(player_event_summaries);
    dbg!(game_summary);
}
