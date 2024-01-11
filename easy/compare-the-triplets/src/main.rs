use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'compareTriplets' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

fn compareTriplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0, 0];

    for (index, a_value) in a.iter().enumerate() {
        if a_value > b.get(index).unwrap() {
            result[0] += 1;
        } else if a_value < b.get(index).unwrap() {
            result[1] += 1;
        }
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

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

    let result = compareTriplets(&a, &b);

    println!("{} {}", result.get(0).unwrap(), result.get(1).unwrap())

    // for i in 0..result.len() {
    //     write!(&mut fptr, "{}", result[i]).ok();

    //     if i != result.len() - 1 {
    //         write!(&mut fptr, " ").ok();
    //     }
    // }

    // writeln!(&mut fptr).ok();
}
