use std::{collections::HashMap, io::Error};

use aoc_22::read_input;

fn main() -> Result<(), Error> {
    let input = read_input("input/02")?;
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));

    Ok(())
}

fn part1(input: &str) -> Score {
    let strategy = HashMap::from([("X", Move::Rock), ("Y", Move::Paper), ("Z", Move::Scissors)]);

    let game = Game::from_moves(input, strategy);
    game.play()
}

fn part2(input: &str) -> Score {
    let strategy = HashMap::from([
        ("X", Outcome::Lost),
        ("Y", Outcome::Draw),
        ("Z", Outcome::Won),
    ]);

    let game = Game::from_outcome(input, strategy);
    game.play()
}

type Score = usize;

struct Game {
    rounds: Vec<Round>,
}

impl Game {
    fn from_moves(input: &str, strategy: HashMap<&str, Move>) -> Game {
        let mut rounds = Vec::new();
        for line in input.lines() {
            let mut moves = line.split_ascii_whitespace();
            let opponent_move = Move::parse(
                moves
                    .next()
                    .expect("Expected opponent move, found nothing."),
            );
            let my_move = *strategy
                .get(moves.next().expect("Expected my move, found nothing."))
                .unwrap();

            let round = Round {
                my_move,
                opponent_move,
            };

            rounds.push(round);
        }
        Game { rounds }
    }

    fn from_outcome(input: &str, strategy: HashMap<&str, Outcome>) -> Game {
        let mut rounds = Vec::new();
        for line in input.lines() {
            let mut moves = line.split_ascii_whitespace();
            let opponent_move = Move::parse(
                moves
                    .next()
                    .expect("Expected opponent move, found nothing."),
            );
            let game_outcome = strategy
                .get(moves.next().expect("Expected my move, found nothing."))
                .unwrap();
            let my_move = opponent_move.results(game_outcome);

            let round = Round {
                my_move,
                opponent_move,
            };

            rounds.push(round);
        }
        Game { rounds }
    }

    fn play(&self) -> Score {
        let mut score = Score::default();

        for round in self.rounds.iter() {
            score += round.play();
        }

        score
    }
}

struct Round {
    my_move: Move,
    opponent_move: Move,
}

impl Round {
    fn play(&self) -> Score {
        self.my_move as Score + self.my_move.play(self.opponent_move) as Score
    }
}

#[derive(Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    fn parse(input: &str) -> Move {
        match input {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => panic!("Unexpected move value, brrrrr!"),
        }
    }

    fn results(&self, result: &Outcome) -> Move {
        match self {
            Move::Rock => match result {
                Outcome::Lost => Move::Scissors,
                Outcome::Draw => Move::Rock,
                Outcome::Won => Move::Paper,
            },
            Move::Paper => match result {
                Outcome::Lost => Move::Rock,
                Outcome::Draw => Move::Paper,
                Outcome::Won => Move::Scissors,
            },
            Move::Scissors => match result {
                Outcome::Lost => Move::Paper,
                Outcome::Draw => Move::Scissors,
                Outcome::Won => Move::Rock,
            },
        }
    }
}

enum Outcome {
    Lost = 0,
    Draw = 3,
    Won = 6,
}

impl Move {
    fn play(&self, opponent: Self) -> Outcome {
        match self {
            Move::Rock => match opponent {
                Move::Rock => Outcome::Draw,
                Move::Paper => Outcome::Lost,
                Move::Scissors => Outcome::Won,
            },
            Move::Paper => match opponent {
                Move::Rock => Outcome::Won,
                Move::Paper => Outcome::Draw,
                Move::Scissors => Outcome::Lost,
            },
            Move::Scissors => match opponent {
                Move::Rock => Outcome::Lost,
                Move::Paper => Outcome::Won,
                Move::Scissors => Outcome::Draw,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::*;

    #[test]
    fn test() {
        let input = indoc! {"
            A Y
            B X
            C Z
        "};

        assert_eq!(part1(input), 15);
        assert_eq!(part2(input), 12);
    }
}
