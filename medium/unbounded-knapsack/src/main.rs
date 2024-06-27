use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'unboundedKnapsack' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER k
 *  2. INTEGER_ARRAY arr
 */

fn unboundedKnapsack(k: i32, arr: &[i32]) -> i32 {
    let bound = k as usize;
    let mut knapsack: Vec<bool> = vec![false; bound + 1];

    knapsack[0] = true;

    for x in 0..=bound {
        if knapsack[x] == false {
            continue;
        }

        for &pack in arr {
            if x + pack as usize <= bound {
                knapsack[x + pack as usize] = true;
            }
        }
    }

    let mut result = 0;

    for i in (0..=bound).rev() {
        if knapsack[i] == true {
            result = i as i32;

            break;
        }
    }

    result
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

        let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        let result = unboundedKnapsack(k, &arr);

        // writeln!(&mut fptr, "{}", result).ok();

        println!("{}", result);
    }
}
