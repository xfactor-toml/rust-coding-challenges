use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'beadOrnaments' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY b as parameter.
 */

fn beadOrnaments(b: &[i32]) -> i32 {
    let mut result: i32 = 1;

    let modulo: u64 = 1000000007;

    for &k in b.iter() {
        let mut tmp = 1;

        for _ in 0..k-1 {
            tmp = (tmp * k as u64) % modulo;
        }

        result =  ((tmp * result as u64) % modulo) as i32;
    }

    if b.len() >= 2 {
        let mut tmp = 1;
        let bead_count = b.iter().sum::<i32>();

        for _ in 0..b.len() - 2 {
            tmp = (tmp * bead_count as u64) % modulo;
        }

        result =  ((tmp * result as u64) % modulo) as i32;
    } else {
        result /= b.iter().sum::<i32>().pow(2 - b.len() as u32 );
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let _b_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let b: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();

        let result = beadOrnaments(&b);

        println!("{}", result);
        // writeln!(&mut fptr, "{}", result).ok();
    }
}
