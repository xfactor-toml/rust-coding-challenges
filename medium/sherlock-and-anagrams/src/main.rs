use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;

/*
 * Complete the 'sherlockAndAnagrams' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING s as parameter.
 */

fn sherlockAndAnagrams(s: &str) -> i32 {
    let origin: Vec<u8> = s.bytes().collect();

    let mut substrings: HashMap<u128, u8> = HashMap::new();

    for i in 0..s.len() {
        for j in i+1..=s.len() {
            let mut substring: u128 = 0;

            for k in i..j {
                substring += 26_u128.pow((origin[k] - 97) as u32);
            }

            if substring == 0 {
                continue;;
            }

            if let Some(count) = substrings.get_mut(&substring) {
                *count += 1;
            } else {
                substrings.insert(substring, 1);
            }
        }
    }

    let mut result = 0;

    for count in substrings.into_values() {
        result += count as i32 * (count as i32 - 1) / 2;
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let q = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..q {
        let s = stdin_iterator.next().unwrap().unwrap();

        let result = sherlockAndAnagrams(&s);

        println!("{}", result);

        // writeln!(&mut fptr, "{}", result).ok();
    }
}
