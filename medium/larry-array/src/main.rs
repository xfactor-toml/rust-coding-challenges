use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'larrysArray' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts INTEGER_ARRAY A as parameter.
 */

fn larrysArray(A: &[i32]) -> String {
    let mut count = 0;
    
    for i in 0..A.len() {
        for j in (i+1)..A.len() {
            if A[i] > A[j] {
                count += 1;
            }
        }
    }

    if count % 2 == 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let A: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        let result = larrysArray(&A);

        println!("{}", result);
        // writeln!(&mut fptr, "{}", result).ok();
    }
}
