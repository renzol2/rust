use rand::Rng;
use std::io;

enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Loss,
    Tie,
}

fn play_rps(player_choice: &Choice, opponent_choice: &Choice) -> Outcome {
    let outcome = match (player_choice, opponent_choice) {
        // Wins
        (Choice::Rock, Choice::Scissors) => Outcome::Win,
        (Choice::Scissors, Choice::Paper) => Outcome::Win,
        (Choice::Paper, Choice::Rock) => Outcome::Win,

        // Losses
        (Choice::Rock, Choice::Paper) => Outcome::Loss,
        (Choice::Paper, Choice::Scissors) => Outcome::Loss,
        (Choice::Scissors, Choice::Rock) => Outcome::Loss,

        _ => Outcome::Tie,
    };

    outcome
}

fn print_results(games: u32, wins: u32) {
    println!(
        "Total games: {}, total wins: {}, win ratio: {}",
        games,
        wins,
        (wins as f32) / (games as f32)
    )
}

fn main() {
    println!("Play Rock Paper Scissors!");

    let mut games: u32 = 0;
    let mut wins: u32 = 0;

    let choices = [Choice::Rock, Choice::Paper, Choice::Scissors];
    loop {
        println!("Enter 'r' to play rock.");
        println!("Enter 'p' to play paper.");
        println!("Enter 's' to play scissors.");
        println!("Enter 'w' to show results so far.");
        println!("Enter 'q' to quit the game.");
        let opponent_choice = &choices[rand::thread_rng().gen_range(0..3)];
        let mut player_choice = String::new();

        io::stdin()
            .read_line(&mut player_choice)
            .expect("Failed to read line");

        let player_choice: Choice = match player_choice.trim() {
            "r" => Choice::Rock,
            "p" => Choice::Paper,
            "s" => Choice::Scissors,
            "q" => break,
            "w" => {
                print_results(games, wins);
                continue;
            }
            _ => continue,
        };

        let outcome = play_rps(&player_choice, opponent_choice);

        games += 1;

        match outcome {
            Outcome::Win => {
                wins += 1;
                print_results(games, wins);
            }
            _ => print_results(games, wins),
        }
    }
}
