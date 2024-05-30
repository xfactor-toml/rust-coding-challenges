use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'minimumLoss' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts LONG_INTEGER_ARRAY price as parameter.
 */

fn minimumLoss(price: &[i64]) -> i32 {
    let mut prices: Vec<(usize, &i64)> = price.iter().enumerate().into_iter().collect();

    prices.sort_by(|(_, a), (_, b)| a.cmp(b));

    prices
        .iter()
        .zip(prices.iter().skip(1))
        .filter(|((sell_time, _), (buy_time, _))| buy_time < sell_time)
        .map(|(&(_, sell_price), &(_, buy_price))| buy_price - sell_price)
        .min()
        .unwrap() as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let price: Vec<i64> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i64>().unwrap())
        .collect();

    let result = minimumLoss(&price);

    println!("{}", result);

    // writeln!(&mut fptr, "{}", result).ok();
}
