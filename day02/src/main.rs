// Plan:
// - Make a constant map of colors to ints that are possible
// - Make a game class with a vector of pulls
// - Make a pull class with a map of colors to ints
// - Make a function to parse the input into a game class
// - Make a function to check if a game is possible given the pulls and the possible colors compared to the maximum
// - Map over the games and sum the ids of the possible games

use std::{collections::HashMap, fs};
use anyhow::Result;

#[derive(Debug)]
struct Pull {
    colors: HashMap<String, i32>,
}
struct Game {
    id: i32,
    pulls: Vec<Pull>,
}

impl Game {
    fn is_valid(&self) -> bool {
        let mut possible = true;
        self.pulls.iter().for_each(|pull| {
            pull.colors.iter().for_each(|(color, value)| {
                if is_over_max(color.to_string(), *value) {
                    possible = false;
                }
            });
        });
        possible
    }

    fn max_color(&self, color: String) -> i32 {
        let mut max = 0;
        self.pulls.iter().for_each(|pull| {
            pull.colors.iter().for_each(|(pull_color, value)| {
                if pull_color == &color && value > &max {
                    max = *value;
                }
            });
        });
        max
    }
    fn mult_colors(&self) -> i32 {
        self.max_color("red".to_string()) * self.max_color("green".to_string()) * self.max_color("blue".to_string())
    }
}
fn main() -> Result<()> {
    let games = fs::read_to_string("input.txt")?
        .lines()
        .map(|line| line_to_game(line)).collect::<Vec<Game>>();


    // Part 1
    let part1_count = games.iter().filter(|game| game.is_valid())
        .map(|game| game.id)
        .sum::<i32>();

    println!("Count: {}", part1_count);

    // Part 2
    let part2_color_sum = games.iter().map(|game| game.mult_colors()).sum::<i32>();
    println!("Color sum: {}", part2_color_sum);

    Ok(())
}

fn line_to_game(line: &str) -> Game {
    let mut spl = line.split(":");
    let game_id = digits_from_str(spl.next().unwrap());
    let pulls_str = spl.next().unwrap().split(";");
    let pulls = pulls_str.map(|x| {
        let mut pull = Pull {
            colors: HashMap::new(),
        };
        let colors = x.split(",");
        colors.for_each(|y| {
            let mut color = y.trim().split(" ");
            let color_number = color.next().unwrap().trim().parse::<i32>().unwrap();
            let color_name = color.next().unwrap().trim().to_string();
            pull.colors.insert(color_name.to_string(), color_number);
        });
        pull
    }).collect();

    Game {
        id: game_id,
        pulls,
    }
}

fn digits_from_str(s: &str) -> i32 {
    let digits: String = s.chars().filter(|c| c.is_digit(10)).collect();
    digits.parse::<i32>().unwrap()
}

fn is_over_max(color: String, value: i32) -> bool {
    let max = match color.as_str() {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
        _ => panic!("Invalid color"),
    };
    value > max
}
