mod entities;
mod test;

use std::collections::HashMap;

use entities::{Coordinate, Event, EventSummary, GameBlock, Score};

fn main() {
    let input = include_str!("../examples/complex.yml");
    let game = serde_yaml::from_str::<GameBlock>(input).unwrap();
    let mut coordinates = HashMap::new();
    coordinates.insert("game1".to_string(), vec![Coordinate::new(3, 8)]);
    coordinates.insert("game2".to_string(), vec![Coordinate::new(4, 3)]);
    coordinates.insert("game3".to_string(), vec![Coordinate::new(8, 9)]);
    coordinates.insert("game4".to_string(), vec![Coordinate::new(2, 3)]);

    let mut summaries: Vec<EventSummary> = vec![];

    // // quarter 1
    // let score = Score::new(8, 9);
    // let event = Event::Quarter1;
    //
    // let event_summary = game.summarize_event(score, event, &coordinates);
    // summaries.push(event_summary);
    //
    // // warning 1
    // let score = Score::new(8, 9);
    // let event = Event::Warning1;
    //
    // let event_summary = game.summarize_event(score, event, &coordinates);
    // summaries.push(event_summary);
    //
    // // quarter 2
    // let score = Score::new(8, 9);
    // let event = Event::Quarter2;
    //
    // let event_summary = game.summarize_event(score, event, &coordinates);
    // summaries.push(event_summary);
    //
    // // quarter 3
    // let score = Score::new(8, 9);
    // let event = Event::Quarter3;
    //
    // let event_summary = game.summarize_event(score, event, &coordinates);
    // summaries.push(event_summary);
    //
    // // warning 2
    // let score = Score::new(8, 9);
    // let event = Event::Warning2;
    //
    // let event_summary = game.summarize_event(score, event, &coordinates);
    // summaries.push(event_summary);
    //
    // // quarter 4
    // let score = Score::new(8, 9);
    // let event = Event::Quarter4;
    //
    // let event_summary = game.summarize_event(score, event, &coordinates);
    // summaries.push(event_summary);
    //
    // // final
    // let score = Score::new(8, 9);
    // let event = Event::Final;
    //
    // let event_summary = game.summarize_event(score, event, &coordinates);
    // summaries.push(event_summary);

    let total_won = summaries.iter().map(|e| e.amount_won).sum::<u64>();
    dbg!(summaries);
    dbg!(total_won);
}
