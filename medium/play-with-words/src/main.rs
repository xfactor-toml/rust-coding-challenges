use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'playWithWords' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING s as parameter.
 */

fn playWithWords(s: &str) -> i32 {
    let n = s.len();
    let seq = s.as_bytes();
    let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; n];

    for i in 0..n {
        dp[i][i] = 1;
    }

    for length in 2..=n {
        for i in 0..=(n-length) {
            let j = i + length - 1;

            if seq[i] == seq[j] {
                if length == 2 {
                    dp[i][j] = 2;
                } else {
                    dp[i][j] = dp[i+1][j-1] + 2;
                }
            } else {
                dp[i][j] = dp[i][j-1].max(dp[i+1][j]);
            }
        }
    }

    let mut result = 0;

    for i in 1..n {
        if result < dp[0][i - 1] * dp[i][n-1] {
            result = dp[0][i - 1] * dp[i][n-1];
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = playWithWords(&s);

    println!("{}", result);
    // writeln!(&mut fptr, "{}", result).ok();
}
