use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'substrings' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING n as parameter.
 */

fn substrings(n: &str) -> i32 {
    let numbers: Vec<u64> = n.bytes().map(|v| (v - 48) as u64).collect();
    let modulo: u64 = 1000000007;
    let mut result: Vec<u64> = vec![0; n.len()];

    for (i, &n) in numbers.iter().enumerate() {
        if i == 0 {
            result[0] = numbers[0];
        } else {
            result[i] = ((i as u64 + 1) * n + 10 * result[i - 1]) % modulo;
        }
    }

    result.into_iter().reduce(|a, b| (a + b) % modulo).unwrap() as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap();

    let result = substrings(&n);

    // writeln!(&mut fptr, "{}", result).ok();
    println!("{}", result);
}
