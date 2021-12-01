mod main_test;

use std::fs;


fn count_increases_for_window(depths: &Vec::<u32>) -> u32 {
    let mut increases: u32 = 0;

    for i in 2..depths.len()-1 {
        let first_window = depths[i-2] + depths[i-1] + depths[i];
        let second_window = depths[i-1] + depths[i] + depths[i+1];
        if second_window > first_window {
            increases += 1;
        }
    }

    return increases
}

fn count_increases(depths: &Vec::<u32>) -> u32 {
    let mut increases: u32 = 0;

    for i in 1..depths.len() {
        if depths[i] > depths[i-1] {
            increases += 1;
        }
    }

    return increases
}


fn main() {
    let input_file = fs::read_to_string("input.txt").unwrap();

    let number_strings: Vec<&str> = input_file
        .split("\r\n")
        .collect();

    let mut depths: Vec<u32> = vec![];
    for number_string in number_strings {
        match number_string.parse::<u32>() {
            Ok(n) => depths.push(n),
            _ => {}
        }
    }
    print!("{:?}\n", count_increases(&depths));
    print!("{:?}\n", count_increases_for_window(&depths));
}
