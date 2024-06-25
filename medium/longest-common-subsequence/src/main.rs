use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'longestCommonSubsequence' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

fn longestCommonSubsequence(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut lcs: Vec<i32> = Vec::new();
    let mut lcs_length: Vec<Vec<i32>> = vec![vec![0; a.len() + 1]; b.len() + 1];

    for i in 1..=b.len() {
        for j in 1..=a.len() {
            if a[j - 1] == b[i - 1] {
                lcs_length[i][j] = lcs_length[i - 1][j - 1] + 1;
            } else {
                lcs_length[i][j] = lcs_length[i - 1][j].max(lcs_length[i][j - 1]);
            }
        }
    }

    let mut x = b.len();
    let mut y = a.len();
    let mut max_length = lcs_length[x][y];

    while max_length > 0 {
        if lcs_length[x][y] == lcs_length[x][y - 1] {
            y -= 1;
        } else if lcs_length[x][y] == lcs_length[x - 1][y] {
            x -= 1;
        } else {
            x -= 1;
            y -= 1;
            
            lcs.push(b[x]);
            max_length = lcs_length[x][y];
        }
    }

    lcs.reverse();

    lcs
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let b: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = longestCommonSubsequence(&a, &b);

    for i in 0..result.len() {
        // write!(&mut fptr, "{}", result[i]).ok();
        print!("{}", result[i]);

        if i != result.len() - 1 {
            // write!(&mut fptr, " ").ok();
            print!(" ");
        }
    }

    // writeln!(&mut fptr).ok();
}
