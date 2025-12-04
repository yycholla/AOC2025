use std::fs;

fn main() {
    let mut sum = 0;
    for line in read_file().lines() {
        sum += max_power(line.trim());
    }
    println!("{}", sum);
}

fn read_file() -> String {
    fs::read_to_string("/home/chanway/Documents/AOC2025/day3/src/input")
        .expect("Unable to read file")
}

fn max_power(s: &str) -> u64 {
    // turn string into list of chars
    let chars: Vec<char> = s.chars().collect();
    // Get length of list
    let n = chars.len();
    // Allowed result len
    let k = 12;

    let mut result = String::new();
    let mut start = 0;

    // Set area available for digit prev_selection <- x -> digits remaining
    for remaining in (1..=k).rev() {
        let end = n - remaining;

        let mut best_idx = start;
        let mut best_char = chars[start];
        for i in start..=end {
            if chars[i] > best_char {
                best_char = chars[i];
                best_idx = i;
            }
        }

        result.push(best_char);
        start = best_idx + 1;
    }
    chars.iter().for_each(|x| print!("{}", x));
    print!("= {}\n\n", result);
    result.parse().unwrap()
}
