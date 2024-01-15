use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'nonDivisibleSubset' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER k
 *  2. INTEGER_ARRAY s
 */

fn nonDivisibleSubset(k: i32, s: &[i32]) -> i32 {
    let mut length ;
    let mut remains = vec![0; k as usize];

    for &value in s.iter() {
        remains[(value % k) as usize] += 1;
    }
    
    length = remains[0].min(1);
    
    if k % 2 == 0 {
        length +=  remains[(k / 2) as usize].min(1);
    }

    for index in 1..=k / 2 {
        if index != k - index {
            length += remains[index as usize].max(remains[(k - index) as usize]);
        }
    }
    
    length
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

    let s: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = nonDivisibleSubset(k, &s);

    println!("{}", result);

    // writeln!(&mut fptr, "{}", result).ok();
}