use std::fs;


fn count_increases(depths: Vec::<u32>) -> u32 {
    let mut increases: u32 = 0;

    for i in 0..depths.len()-1 {
        if depths[i+1] > depths[i] {
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
    print!("{:?}\n", count_increases(depths));
}
