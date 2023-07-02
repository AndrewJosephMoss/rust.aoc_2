use std::str::FromStr;

#[derive(Debug)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Move, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Not a move".to_string())
        }
    }
}

trait ToScore {
    fn to_base_points(&self) -> u32;

    fn to_game_points(&self, other: &Move) -> u32;
}

impl ToScore for Move {
    fn to_base_points(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn to_game_points(&self, other: &Move) -> u32 {
        match self {
            Move::Rock => {
                match other {
                    Move::Rock => 3,
                    Move::Paper => 0,
                    Move::Scissors => 6,
                }
            },
            Move::Paper => {
                match other {
                    Move::Rock => 6,
                    Move::Paper => 3,
                    Move::Scissors => 0,
                }
            },
            Move::Scissors => {
                match other {
                    Move::Rock => 0,
                    Move::Paper => 6,
                    Move::Scissors => 3,
                }
            }
        }
    }
}

#[derive(Debug)]
enum GameEnd {
    Win = 6,
    Loss = 0,
    Draw = 3,
}

impl FromStr for GameEnd {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameEnd::Loss),
            "Y" => Ok(GameEnd::Draw),
            "Z" => Ok(GameEnd::Win),
            _ => Err(String::from("Not able to parse"))
        }
    }
}

trait ForGameEnd {
    fn get_move(&self, end_type: &GameEnd) -> Move;

    fn get_game_end(&self) -> GameEnd;
}

impl ForGameEnd for Move {
    fn get_move(&self, end_type: &GameEnd) -> Move {
        match self {
            Move::Rock => {
                match end_type {
                    GameEnd::Win => Move::Paper,
                    GameEnd::Draw => Move::Rock,
                    GameEnd::Loss => Move::Scissors,
                }
            },
            Move::Paper => {
                match end_type {
                    GameEnd::Win => Move::Scissors,
                    GameEnd::Draw => Move::Paper,
                    GameEnd::Loss => Move::Rock,
                }
            },
            Move::Scissors => {
                match end_type {
                    GameEnd::Win => Move::Rock,
                    GameEnd::Draw => Move::Scissors,
                    GameEnd::Loss => Move::Paper,
                }
            }
        }
    }

    fn get_game_end(&self) -> GameEnd {
        match self {
            Move::Rock => GameEnd::Loss,
            Move::Paper => GameEnd::Draw,
            Move::Scissors => GameEnd::Win,
        }
    }
}

pub fn part_1(input: &str) -> u32 {
    let score = input.lines().map(|moves| {
        let moves = moves.split(" ").map(|s| Move::from_str(s).unwrap()).collect::<Vec<Move>>();
        Move::to_base_points(&moves[1]) + Move::to_game_points(&moves[1], &moves[0])
    }).sum();
    score
}

pub fn part_2(input: &str) -> u32 {
    let score = input.lines().map(|moves| {
        let moves = moves.split(" ").collect::<Vec<&str>>();
        let game_end = GameEnd::from_str(&moves[1]).unwrap();
        let other_move = Move::from_str(&moves[0]).unwrap();
        let my_move = other_move.get_move(&game_end);
        return game_end as u32 + my_move as u32
    }).sum::<u32>();
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("input_test.txt").unwrap();
        let output = part_1(&input);
        assert_eq!(output, 15)
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("input_test.txt").unwrap();
        let output = part_2(&input);
        assert_eq!(output, 12)
    }
}