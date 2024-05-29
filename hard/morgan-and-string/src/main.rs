use std::borrow::BorrowMut;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::cmp::Ord;

/*
 * Complete the 'morganAndString' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. STRING a
 *  2. STRING b
 */

fn morganAndString(a: &str, b: &str) -> String {
    let (mut az, mut bz) = (a.to_string(), b.to_string());

    let mut result = String::new();
    
    az.push('z');
    bz.push('z');

    for _ in 0..a.len()+b.len() {
        if az < bz {
            result.push(az.remove(0));
        } else {
            result.push(bz.remove(0));
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
        let a = stdin_iterator.next().unwrap().unwrap();

        let b = stdin_iterator.next().unwrap().unwrap();

        let result = morganAndString(&a, &b);

        println!("{}", result);

        // writeln!(&mut fptr, "{}", result).ok();
    }
}
