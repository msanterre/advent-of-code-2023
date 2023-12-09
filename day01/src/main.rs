use std::{fs};
use anyhow::Result;


// Take all lines
// Remove the characters that are not numbers
// Take the first and last number of each line
// Sum all the numbers

fn main() -> Result<()>{
  let inputs = fs::read_to_string("./input.txt")?;
  let lines = inputs.lines();
  let result = lines.map(|line| {
    let numbers: Vec<i32> = line
      .chars()
      .filter(|c| c.is_digit(10))
      .map(|c| c.to_digit(10).unwrap() as i32)
      .collect();
    let first = numbers.first().unwrap();
    let last = numbers.last().unwrap();
    let combined = format!("{}{}", first, last);
    combined.parse::<i32>().unwrap()
  }).sum::<i32>();

  println!("Result: {}", result);

  Ok(())
}
