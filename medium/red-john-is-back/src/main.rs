use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'redJohn' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER n as parameter.
 */

fn redJohn(n: i32) -> i32 {
    let mut factorials: Vec<i32> = vec![];
    let (mut short, mut long) = (n, 0);
    
    let mut value = 1;
    
    factorials.push(value);
    
    for i in 1..=10 {
        value *= i;

        factorials.push(value);
    }

    let mut result = 0;
    
    while short >= 0 {

        let mut temp = 1;

        for i in (short+1)..=(short + long) {
            temp *= i;
        }

        result += temp / factorials[long as usize];

        short -= 4;
        long += 1;
    }

    let mut primes: Vec<i32> = vec![];

    for i in 2..=result {
        if primes.is_empty() {
            primes.push(i);
        } else {
            let mut is_prime = true;

            for &prime in primes.iter() {
                if i % prime == 0 {
                    is_prime = false;

                    break;
                }
            }

            if is_prime {
                primes.push(i);
            }
        }
    }

    primes.len() as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let result = redJohn(n);

        // writeln!(&mut fptr, "{}", result).ok();

        println!("{}", result);
    }
}
