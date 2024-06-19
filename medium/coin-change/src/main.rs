use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'getWays' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. LONG_INTEGER_ARRAY c
 */

fn getWays(n: i32, c: &[i64]) -> i64 {
    let mut ways: Vec<i64> = vec![0; n as usize + 1];

    ways[0] = 1;

    for i in 0..c.len() {
        for j in (c[i] as usize)..(n as usize + 1) {
            ways[j] += ways[j - c[i] as usize];
        }
    }

    ways[n as usize]
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

    let c: Vec<i64> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i64>().unwrap())
        .collect();

    // Print the number of ways of making change for 'n' units using coins having the values given by 'c'

    let ways = getWays(n, &c);

    println!("{}", ways);
    // writeln!(&mut fptr, "{}", ways).ok();
}
