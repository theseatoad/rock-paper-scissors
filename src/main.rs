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
    Scissors
}

impl Distribution<Move> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Move {
        match rng.gen_range(0..=2) {
            0 => Move::Rock,
            1 => Move::Paper,
            _ => Move::Scissors,
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Move::Rock, Move::Rock) => Some(Ordering::Equal),
            (Move::Rock, Move::Paper) => Some(Ordering::Less),
            (Move::Rock, Move::Scissors) => Some(Ordering::Greater),
            (Move::Paper, Move::Rock) => Some(Ordering::Greater),
            (Move::Paper, Move::Paper) => Some(Ordering::Equal),
            (Move::Paper, Move::Scissors) => Some(Ordering::Less),
            (Move::Scissors, Move::Rock) => Some(Ordering::Less),
            (Move::Scissors, Move::Paper) => Some(Ordering::Greater),
            (Move::Scissors, Move::Scissors) => Some(Ordering::Equal),
        }
    }
}

impl FromStr for Move {
    type Err = clap::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "rock" => Ok(Move::Rock),
            "paper" => Ok(Move::Paper),
            "scissors" => Ok(Move::Scissors),
            _ => Err(clap::ErrorKind::InvalidValue),
        }
    }
}
#[derive(Debug, PartialEq)]
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
#[cfg(test)]
mod tests {
    use super::*;

        /** ==== Partial ordering ==== **/
        /* ==== Rocks ==== */

    #[test]
    fn test_partial_ordering_rock_rock(){
        assert_eq!(Move::Rock, Move::Rock)
    }

    #[test]
    fn test_partial_ordering_rock_paper(){
        assert_eq!((Move::Rock < Move::Paper), true)
    }

    #[test]
    fn test_partial_ordering_rock_scissors(){
        assert_eq!((Move::Rock > Move::Scissors), true)
    }

        /* ==== Paper ==== */

    #[test]
    fn test_partial_ordering_paper_rock(){
        assert_eq!((Move::Paper > Move::Rock), true)
    }

    #[test]
    fn test_partial_ordering_paper_paper(){
        assert_eq!(Move::Paper, Move::Paper)
    }

    #[test]
    fn test_partial_ordering_paper_scissors(){
        assert_eq!((Move::Paper < Move::Scissors), true)
    }

        /* ==== Scissors ==== */

    #[test]
    fn test_partial_ordering_scissors_rock(){
        assert_eq!((Move::Scissors < Move::Rock), true)
    }

    #[test]
    fn test_partial_ordering_scissors_paper(){
        assert_eq!((Move::Scissors > Move::Paper), true)
    }

    #[test]
    fn test_partial_ordering_scissors_scissors(){
        assert_eq!(Move::Scissors, Move::Scissors)
    }

        /** ==== Calculate winner==== **/

    #[test]
    fn test_calculate_winner_userwin(){
        assert_eq!(calculate_winner(Move::Rock, Move::Scissors), GameResult::UserWin)
    }

    #[test]
    fn test_calculate_winner_tie(){
        assert_eq!(calculate_winner(Move::Rock, Move::Rock), GameResult::Tie)
    }

    #[test]
    fn test_calculate_winner_opponentwin(){
        assert_eq!(calculate_winner(Move::Rock, Move::Paper), GameResult::OpponentWin)
    }

        /** ==== FromStr ==== **/

    #[test]
    fn test_fromstr_rock(){
        assert_eq!(Move::from_str("rock"), Ok(Move::Rock))
    }

    #[test]
    fn test_fromstr_paper(){
        assert_eq!(Move::from_str("paper"), Ok(Move::Paper))
    }

    #[test]
    fn test_fromstr_scissors(){
        assert_eq!(Move::from_str("scissors"), Ok(Move::Scissors))
    }

    #[test]
    fn test_fromstr_error(){
        assert_eq!(Move::from_str("notreal"), Err(clap::ErrorKind::InvalidValue))
    }

    /** ==== Seeded Random Completeness ==== **/
    
    #[test]
    fn test_distribution_rock(){
        let mut seed = StdRng::seed_from_u64(2);
        let seeded_move : Move = seed.gen();
        assert_eq!(seeded_move, Move::Rock)
    }
    
    #[test]
    fn test_distribution_paper(){
        let mut seed = StdRng::seed_from_u64(7);
        let seeded_move : Move = seed.gen();
        assert_eq!(seeded_move, Move::Paper)
    }

    #[test]
    fn test_distribution_scissors(){
        let mut seed = StdRng::seed_from_u64(1);
        let seeded_move : Move = seed.gen();
        assert_eq!(seeded_move, Move::Scissors)
    }

}