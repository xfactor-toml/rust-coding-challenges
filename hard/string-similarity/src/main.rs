use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'stringSimilarity' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING s as parameter.
 */

fn stringSimilarity(s: &str) -> i64 {
    let mut similarity = 0;
    let mut i = 0;

    let mut repeat = 1;

    let s_vec: Vec<char> = s.chars().collect();

    while repeat < s.len() && s_vec[repeat] == s_vec[0] {
        repeat += 1;
    }

    while i < s.len() {
        let suffix: &str = &s[i..];

        let (mut start, mut end) = (1, suffix.len());

        if &s[0..start] != &suffix[0..start] {
            i += 1;
            continue;
        }

        if &s[0..end] == &suffix[0..end] {
            start = end;
        } else {
            loop {
                if start >= end {
                    break;
                }
    
                let mid = (start + end) / 2 + 1;
    
                if &s[0..mid] != &suffix[0..mid] {
                    end = mid - 1;
                } else {
                    start = mid;
                }
            }
        }

        if repeat > 0 && suffix.len() == s.len() {
            similarity += (repeat + 1) * repeat / 2;

            similarity += start - repeat;

            i += repeat;
        } else {
            similarity += start;
            i += 1;
        }
    }

    similarity as i64
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let s = stdin_iterator.next().unwrap().unwrap();

        let result = stringSimilarity(&s);

        println!("{}", result);

        // writeln!(&mut fptr, "{}", result).ok();
    }
}
