mod entities;
mod test;

use entities::GameBlock;

fn main() {
    // let input = include_str!("../example.yml");
    let input = include_str!("../examples/different_payouts.yml");
    solution(input);
}

fn solution(input: &str) {
    let game = serde_yaml::from_str::<GameBlock>(input).unwrap();
    dbg!(game);
}

