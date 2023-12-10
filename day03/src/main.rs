use anyhow::Ok;
use std::{fs::read_to_string, collections::HashMap};

// Load all the characters in a [row x column] matrix
// Iterate through the array character by character
// When you hit a number, start adding to a string

// SOLUTION 1
// Check around the number for any symbol
// If there is a symbol, validate the number
// When you hit something other than a number, stop adding to the string, parse, and add the number to an array
// Iterate until it's done


fn main() -> anyhow::Result<()> {
    let input = read_to_string("input.txt")?;
    let lines = input.lines();
    let char_matrix: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let mut numbers = Vec::<i32>::new();
    let mut gear_numbers = HashMap::<String, Vec::<i32>>::new();
    let mut str_buf: String = String::new();
    let mut on_number = false;
    let mut valid = false;
    let row_size = char_matrix.len();
    let column_size = char_matrix[0].len();
    let mut gear_position = String::new();

    char_matrix.iter().enumerate().for_each(|(row_index, row)| {
        row.iter().enumerate().for_each(|(column_index, character)| {
            if character.is_numeric() {
                on_number = true;
                str_buf.push(*character);

                // Look around the number for a symbol
                // If there is a symbol, validate the number
                if !valid {
                    surrounding_coords(row_index, column_index, row_size, column_size).iter().for_each(|(x, y)| {
                        let chr = char_matrix[*x][*y];
                        if chr != '.' && !chr.is_numeric() {
                            valid = true;
                        }
                        if chr == '*' {
                            gear_position = format!("{},{}", x, y);
                        }
                    });
                }
            } else {
                if on_number {
                    on_number = false;
                    if str_buf.len() > 0 {
                        if gear_position.len() > 0 {
                            gear_numbers.entry(gear_position.clone()).or_insert(vec![]).push(str_buf.parse::<i32>().unwrap());
                        } else {
                            numbers.push(str_buf.parse::<i32>().unwrap());
                        }
                    }
                    valid = false;
                    str_buf.clear();
                    gear_position.clear();
                }
            }
        });
        if valid && str_buf.len() > 0{
            if gear_position.len() > 0 {
                gear_numbers.entry(gear_position.clone()).or_insert(vec![]).push(str_buf.parse::<i32>().unwrap());
            } else {
                numbers.push(str_buf.parse::<i32>().unwrap());
            }
        }
        valid = false;
        gear_position.clear();
        str_buf.clear();
    });
    gear_numbers.retain(|_, v| v.len() > 1);
    let sum = numbers.iter().sum::<i32>();
    let gears_sum = gear_numbers.values().map(|v| v[0] * v[1]).sum::<i32>();
    println!("\n\n\nnumbers: {:?}", numbers);
    println!("gear_numbers: {:?}", gear_numbers);

    println!("sum: {:?}", sum);
    println!("gears_sum: {:?}", gears_sum);

    Ok(())
}

fn surrounding_coords(x: usize, y: usize, x_size: usize, y_size: usize) -> Vec<(usize, usize)> {
    let mut coords = Vec::<(usize, usize)>::new();
    if x > 0 {
        coords.push((x - 1, y));
        coords.push((x - 1, y + 1));
    }
    if y > 0 {
        coords.push((x, y - 1));
        coords.push((x + 1, y - 1));
        if x > 0 {
            coords.push((x - 1, y - 1));
        }
    }

    coords.push((x, y + 1));
    coords.push((x + 1, y));
    coords.push((x + 1, y + 1));
    coords.retain(|(x, y)| *x < x_size && *y < y_size);
    coords
}
