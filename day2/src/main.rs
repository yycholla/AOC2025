use std::fs;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut sum = 0;
    for line in read_file().split(",") {
        let num = line.split_once("-").expect("Unable to split line");
        let low = num
            .0
            .trim()
            .parse::<i64>()
            .expect("unable to parse low value");
        let high = num
            .1
            .trim()
            .parse::<i64>()
            .expect("unable to parse high value");
        sum += sum_invalid(low, high)
    }
    let elapsed_time = now.elapsed();
    println!("Sum: {} in {} milliseconds", sum, elapsed_time.as_millis());
}

fn read_file() -> String {
    fs::read_to_string("/home/chanway/Documents/AOC2025/day2/src/input")
        .expect("Unable to read file")
}

fn sum_invalid(low: i64, high: i64) -> i64 {
    let mut sum: i64 = 0;
    for i in low..=high {
        let val = i.to_string();
        let length = val.len();
        for j in 1..length {
            if length % j == 0 {
                let subs = val
                    .as_bytes()
                    .chunks(j)
                    .map(|buf| unsafe { str::from_utf8_unchecked(buf) })
                    .collect::<Vec<&str>>();
                let first = subs[0];
                if subs.iter().all(|&item| item == first) {
                    println!("len: {}, num: {}", j, i);
                    sum += i;
                    break;
                }
            }
        }
    }
    sum
}
