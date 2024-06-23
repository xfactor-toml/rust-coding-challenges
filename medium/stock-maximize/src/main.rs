use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'stockmax' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts INTEGER_ARRAY prices as parameter.
 */

fn stockmax(prices: &[i32]) -> i64 {
    let mut profit = 0;

    let mut local_max = 0;

    for &price in prices.iter().rev() {
        if price > local_max {
            local_max = price;
        }

        profit += (local_max - price) as i64;
    }

    profit
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();


    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let prices: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        let result = stockmax(&prices);

        // writeln!(&mut fptr, "{}", result).ok();
        println!("{}", result);
    }
}
