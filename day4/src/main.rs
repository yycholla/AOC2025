use std::fs;

fn main() {
    let input = read_file();
    let input_grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let output = pickable(input_grid);
    let mut count = 0;
    for i in output {
        for j in i {
            if j {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

fn pickable(input: Vec<Vec<char>>) -> Vec<Vec<bool>> {
    let height: usize = input.len();
    let width: usize = input[0].len();
    let directions: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut output = vec![vec![false; width]; height];
    for i in 0..height {
        for j in 0..width {
            if input[i][j] == '@' {
                let mut count = 0;
                for (di, dj) in &directions {
                    if let (Some(ni), Some(nj)) =
                        (i.checked_add_signed(*di), j.checked_add_signed(*dj))
                    {
                        if ni < height && nj < width {
                            count += 1;
                        }
                    }
                }
                println!("current count: {}", count);
                if count < 4 {
                    output[i][j] = true;
                }
            }
        }
    }
    output
}

fn read_file() -> String {
    fs::read_to_string("/home/chanway/Documents/AOC2025/day4/src/input2")
        .expect("Unable to read file")
}
