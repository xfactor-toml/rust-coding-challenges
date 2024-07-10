use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'abbreviation' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. STRING a
 *  2. STRING b
 */

fn abbreviation(a: &str, b: &str) -> String {
    let x: Vec<char> = a.chars().collect();
    let y: Vec<char> = b.chars().collect();
    let z: Vec<char> = a.to_uppercase().chars().collect();

    let mut dp: Vec<Vec<bool>> = vec![vec![false; b.len() + 1]; a.len() + 1];

    dp[0][0] = true;

    for i in 1..=a.len() {
        if x[i - 1].is_lowercase() {
            dp[i][0] = true;
        }
    }

    for i in 1..=a.len() {
        for j in 1..=b.len() {
            if x[i - 1] == y[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else if z[i - 1] == y[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] || dp[i - 1][j];
            } else if x[i - 1] == z[i - 1] {
                dp[i][j] = false;
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }

    match dp[a.len()][b.len()] {
        true => "YES".to_string(),
        false => "NO".to_string(),
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let q = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..q {
        let a = stdin_iterator.next().unwrap().unwrap();

        let b = stdin_iterator.next().unwrap().unwrap();

        let result = abbreviation(&a, &b);

        // writeln!(&mut fptr, "{}", result).ok();
        println!("{}", result);
    }
}
