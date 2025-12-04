use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;
use std::path::Path;

fn main() {
    let mut tally = 0;
    let mut current = 50;
    if let Ok(lines) = read_lines("/home/yycholla/Documents/AOC2025/Day1/input") {
        for line in lines.map_while(Result::ok) {
            let ch = &line[..1];
            let numstr = &line[1..];
            let num = numstr
                .parse::<i32>()
                .expect("Unable to parse string to number");

            let prev = current;
            if ch == "L" {
                current -= num;
            } else {
                current += num;
            }
            current = clamp_round(current, prev, &mut tally);
            println!(
                "LOOP: prev = {}, current = {}, num = {}, Tally = {}",
                prev, current, num, tally
            )
        }
    }
    println!("Tally = {}", tally);
}

fn clamp_round(n: i32, p: i32, t: &mut i32) -> i32 {
    let r: Range<i32> = 0..100;
    if r.contains(&n) {
        if n == 0 {
            increment_tally(t, 1);
        }
        return n;
    }
    if n < 0 && p != 0 {
        increment_tally(t, 1);
    }
    let product = (n / 100).abs();
    increment_tally(t, product);
    (n + r.start).rem_euclid(r.end - r.start) + r.start
}

fn increment_tally(t: &mut i32, n: i32) {
    *t += n;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
