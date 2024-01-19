use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeInWords' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. INTEGER h
 *  2. INTEGER m
 */

fn timeInWords(h: i32, m: i32) -> String {
    let numbers: Vec<_> = vec![
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "quarter",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
        "twenty",
        "twenty one",
        "twenty two",
        "twenty three",
        "twenty four",
        "twenty five",
        "twenty six",
        "twenty seven",
        "twenty egith",
        "twenty nine",
        "half"
    ];

    if m > 30 {
        let mut minute = numbers[(60 - m - 1) as usize].to_string();
        let hour = numbers[(h % 12) as usize].to_string();
        
        if m % 15 > 0 {
            minute.push_str(" minute");

            if m != 59 {
                minute.push_str("s");
            }
        }

        return format!("{} to {}", minute, hour);
    } else if m <= 30 && m > 0 {
        let mut minute = numbers[(m - 1) as usize].to_string();
        let hour = numbers[((h + 11) % 12) as usize].to_string();

        if m % 15 > 0 {
            minute.push_str(" minute");

            if m != 1 {
                minute.push_str("s");
            }
        }

        return format!("{} past {}", minute, hour);
    } else {
        return format!("{} o' clock", numbers[((h + 11) % 12) as usize].to_string());
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let h = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let m = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = timeInWords(h, m);

    // writeln!(&mut fptr, "{}", result).ok();
    println!("{}", result);
}
