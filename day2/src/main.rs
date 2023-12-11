use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename: &str = args.get(1).map(String::as_ref).unwrap_or("input/sample");

    let contents = fs::read_to_string(filename).unwrap();
    let sum = sum_of_possible_game_ids(&contents);
    println!("The sum of the possible game IDs is {}", sum);

    let power = power_of_fewest_cubes_per_color(&contents);
    println!(
        "The power of the fewest number of cubes of each color is {}",
        power
    );
}

fn power_of_fewest_cubes_per_color(contents: &str) -> u32 {
    let games: Vec<Game> = parse_games(contents);

    let min_cubes = games.iter().map(minimum_set_of_cubes);
    let powers = min_cubes.map(|cubes| cubes.power());
    powers.sum()
}

fn minimum_set_of_cubes(game: &Game) -> CubeCounts {
    game.cube_draws
        .iter()
        .fold(CubeCounts::empty(), |current_minimum, new_cube_draw| {
            current_minimum.max(new_cube_draw)
        })
}

fn sum_of_possible_game_ids(input: &str) -> u32 {
    const TOTAL_CUBES: CubeCounts = CubeCounts {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games: Vec<Game> = parse_games(input);
    let game_ids = games.iter().filter_map(|game| {
        if game
            .cube_draws
            .iter()
            .all(|draw| draw.is_subset_of(&TOTAL_CUBES))
        {
            Some(game.id)
        } else {
            None
        }
    });

    game_ids.sum::<u32>()
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    cube_draws: Vec<CubeCounts>,
}

#[derive(Debug, PartialEq)]
struct CubeCounts {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeCounts {
    fn is_subset_of(&self, other: &CubeCounts) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    fn empty() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }

    fn max(&self, other: &CubeCounts) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
}

fn parse_games(input: &str) -> Vec<Game> {
    input.lines().map(parse_game).collect()
}

fn parse_game(line: &str) -> Game {
    let tokens: Vec<&str> = line.split(": ").collect();
    let id = tokens[0].split(' ').collect::<Vec<&str>>()[1]
        .parse::<u32>()
        .unwrap();
    let draws = tokens[1].split("; ");
    let cube_draws = draws.map(parse_draw).collect();

    Game { id, cube_draws }
}

fn parse_draw(input: &str) -> CubeCounts {
    let mut counts = CubeCounts {
        red: 0,
        green: 0,
        blue: 0,
    };

    for color in input.split(", ") {
        let tokens: Vec<_> = color.split(' ').collect();
        let count = tokens[0].parse::<u32>().unwrap();
        let color = tokens[1];

        match color {
            "red" => {
                counts.red = count;
            }
            "green" => {
                counts.green = count;
            }
            "blue" => {
                counts.blue = count;
            }
            _ => {
                panic!("unknown color");
            }
        }
    }

    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_sample_for_sum_of_possible_game_ids() {
        let contents = fs::read_to_string("input/sample").unwrap();

        let sum = sum_of_possible_game_ids(&contents);
        assert_eq!(sum, 8);
    }

    #[test]
    fn parsing_games_from_line() {
        let examples = [(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            Game {
                id: 1,
                cube_draws: vec![
                    CubeCounts {
                        blue: 3,
                        red: 4,
                        green: 0,
                    },
                    CubeCounts {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    CubeCounts {
                        red: 0,
                        green: 2,
                        blue: 0,
                    },
                ],
            },
        )];

        for (input, expected) in examples.into_iter() {
            let actual = parse_game(input);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn validate_sample_for_power_set_of_fewest_cubes() {
        let contents = fs::read_to_string("input/sample").unwrap();

        let power = power_of_fewest_cubes_per_color(&contents);
        assert_eq!(power, 2286);
    }
}
