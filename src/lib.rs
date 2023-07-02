use std::str::FromStr;

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

pub fn part_1(input: &str) -> u32 {
    let score = input.lines().map(|moves| {
        let moves = moves.split(" ").map(|s| Move::from_str(s).unwrap()).collect::<Vec<Move>>();
        Move::to_base_points(&moves[1]) + Move::to_game_points(&moves[1], &moves[0])
    }).sum();
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
}