use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'biggerIsGreater' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING w as parameter.
 */

fn biggerIsGreater(w: &str) -> String {
    let mut rev_w: Vec<char> = w.chars().rev().collect();

    let mut i_pos: usize = 0;

    for i in 0..rev_w.len() {
        if i < rev_w.len() - 1 && rev_w[i] > rev_w[i + 1] {
            i_pos = i + 1;

            break;
        }
    }

    if i_pos == 0 {
        return "no answer".to_string();
    } else {
        let mut j_pos = i_pos - 1 ;

        for j in 0..i_pos {
            if rev_w[j] > rev_w[i_pos] {
                j_pos = j;

                break;
            }
        }

        let mut new_w: Vec<char> = rev_w.clone();

        new_w[i_pos] = rev_w[j_pos];
        rev_w[j_pos] = rev_w[i_pos];


        for i in 0..i_pos {
            new_w[i] = rev_w[i_pos - 1 - i];
        }

        new_w.into_iter().rev().collect()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let T = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..T {
        let w = stdin_iterator.next().unwrap().unwrap();

        let result = biggerIsGreater(&w);

        println!("{}", result);

        // writeln!(&mut fptr, "{}", result).ok();
    }
}
