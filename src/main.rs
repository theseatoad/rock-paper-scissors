use std::{cmp::Ordering, str::FromStr, process};

use clap::{Parser};
use rand::{
    distributions::{Distribution, Standard},
    Rng, SeedableRng, prelude::StdRng,
};

/**
 * A basic command line rock paper scissors game.
 */

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    pattern: String,
    seed: Option<u64>
}

#[derive(Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissor
}

impl Distribution<Move> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Move {
        match rng.gen_range(0..=2) {
            0 => Move::Rock,
            1 => Move::Paper,
            _ => Move::Scissor,
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Move::Rock, Move::Rock) => Some(Ordering::Equal),
            (Move::Rock, Move::Paper) => Some(Ordering::Less),
            (Move::Rock, Move::Scissor) => Some(Ordering::Greater),
            (Move::Paper, Move::Rock) => Some(Ordering::Greater),
            (Move::Paper, Move::Paper) => Some(Ordering::Equal),
            (Move::Paper, Move::Scissor) => Some(Ordering::Less),
            (Move::Scissor, Move::Rock) => Some(Ordering::Less),
            (Move::Scissor, Move::Paper) => Some(Ordering::Greater),
            (Move::Scissor, Move::Scissor) => Some(Ordering::Equal),
        }
    }
}

impl FromStr for Move {
    type Err = clap::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "rock" => Ok(Move::Rock),
            "paper" => Ok(Move::Paper),
            "scissors" => Ok(Move::Scissor),
            _ => Err(clap::ErrorKind::InvalidValue),
        }
    }
}

enum GameResult {
    UserWin,
    OpponentWin,
    Tie
}

fn main() {
    // Initalize cli parser.
    let args = Cli::parse();

    // Try to get our move, exits program on error.
    let our_move : Move = match Move::from_str(&args.pattern) {
        Ok(x) => x,
        Err(_) => {
            eprintln!("Invalid move");
            process::exit(1)
        }
    };

    // If optional seed provided, use it, else, random from entropy.
    let mut rng : StdRng = match args.seed {
        None => StdRng::from_entropy(),
        Some(x) => StdRng::seed_from_u64(x)
    };

    // Generate a random move.
    let opponent_move : Move = rng.gen();

    // Let the user know what the move the opponent generated.
    print!("Opponent's move: {:?}. " , opponent_move);

    // Let the user know who won.
    match calculate_winner(our_move, opponent_move){
        GameResult::UserWin => println!("You win!"),
        GameResult::Tie => println!("Tie"),
        GameResult::OpponentWin => println!("You lose!"),
    }
}

fn calculate_winner(user: Move, opponent:Move) -> GameResult {
    if user > opponent {
        return GameResult::UserWin
    } else if user == opponent {
        return GameResult::Tie
    } else {
        return GameResult::OpponentWin
    }
}