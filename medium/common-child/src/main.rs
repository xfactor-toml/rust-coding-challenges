use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'commonChild' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. STRING s1
 *  2. STRING s2
 */

fn commonChild(s1: &str, s2: &str) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![0; s1.len() + 1]; s2.len() + 1];

    for (i, x) in s2.chars().enumerate() {
        for (j, y) in s1.chars().enumerate() {
            if x == y {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }

    dp[s2.len()][s1.len()]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s1 = stdin_iterator.next().unwrap().unwrap();

    let s2 = stdin_iterator.next().unwrap().unwrap();

    let result = commonChild(&s1, &s2);

    // writeln!(&mut fptr, "{}", result).ok();

    println!("{}", result);
}
