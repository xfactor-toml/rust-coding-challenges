use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'legoBlocks' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER m
 */

fn legoBlocks(n: i32, m: i32) -> i32 {
    let h = n as usize;
    let w = m as usize;

    let modulo: u64 = 1000000007;

    // if width is 0, combination is 1
    // if width is 1, combination is 1
    // if width is 2, combination is 2
    // if width is 3, combination is 4
    let mut row_combinations: Vec<u64> = vec![1, 1, 2, 4];

    // Build row combinations up to current wall's width
    while row_combinations.len() <= w {
        row_combinations.push(row_combinations.iter().rev().take(4).sum::<u64>() % modulo);
    }

    // Calculate total combination containing violations
    let mut total_combination: Vec<u64> = vec![];

    for (i, combination) in row_combinations.iter().enumerate() {
        total_combination.push(1);

        for _ in 0..h {
            total_combination[i] = total_combination[i] * combination % modulo;
        }
    }


    // Calculate unstable
    let mut unstable: Vec<u64> = vec![0, 0];

    for i in 2..=w {
        let mut unstable_i = 0;

        for j in 1..i {
            unstable_i = (unstable_i + (total_combination[j] + modulo - unstable[j]) * total_combination[i - j]) % modulo;
        }

        unstable.push(unstable_i);
    }

    ((total_combination[w] + modulo - unstable[w]) % modulo) as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let result = legoBlocks(n, m);

        // writeln!(&mut fptr, "{}", result).ok();
        println!("{}", result);
    }
}
