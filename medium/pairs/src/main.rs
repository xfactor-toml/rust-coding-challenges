use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'pairs' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER k
 *  2. INTEGER_ARRAY arr
 */

fn pairs(k: i32, arr: &[i32]) -> i32 {
    let mut values = arr.to_vec();
    
    values.sort();
    
    let mut count = 0;
    let (mut left, mut right) = (0, 1);

    while right < values.len() {
        if values[right] - values[left] < k {
            right += 1;

            continue;
        }

        if values[right] - values[left] == k {
            count += 1;
        }

        left += 1;
    }

    count
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

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = pairs(k, &arr);

    // writeln!(&mut fptr, "{}", result).ok();
    println!("{}", result);
}
