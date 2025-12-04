use std::{collections::BTreeMap, fs, str::Chars};

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

fn max_power(s: &str) -> u32 {
    let joltage_array: [(char, usize); 12];
    let mut i = 0;
    for (val, index) in note_index_sort(s.chars()).iter().rev() {
        let lowest_index = index.iter().min().expect("");
        joltage_array[i] = (*val, *lowest_index);
        for (val2, index2) in note_index_sort(s.chars()).iter().rev() {
            let second = *val2;
            if index2.iter().max() > index.iter().min() {
                let mut output = String::new();
                output.push(first);
                output.push(second);
                return output.parse::<u32>().expect("unable to parse final int");
            }
        }
    }
    0
}

fn note_index_sort(b: Chars) -> BTreeMap<char, Vec<usize>> {
    let mut byte_map: BTreeMap<char, Vec<usize>> = BTreeMap::new();
    for (i, char) in b.enumerate() {
        byte_map
            .entry(char)
            .and_modify(|vec_value| vec_value.push(i))
            .or_insert(vec![i]);
    }
    byte_map
}
