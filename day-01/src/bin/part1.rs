use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("file path {}", file_path);
    let contents = fs::read_to_string(file_path).unwrap();

    let output = contents
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|char| {
                char.to_digit(10)
                });
            let first = 
                it.next().expect("should be a number");
            let last = it.last();
            match last {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .expect("Should be a number")
        })
        .sum::<u32>();

    println!("{}", output)
}
