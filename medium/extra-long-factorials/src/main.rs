use std::{io::{self, BufRead}, borrow::Borrow};

/*
 * Complete the 'extraLongFactorials' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn extraLongFactorials(n: i32) {
    let mut result = vec![1];

    for value in 1..=n {
        let mut temp = 0;
        
        for index in 0..result.len() {
            let sum = result[index] * value + temp;
            result[index] = sum % 100;
            temp = sum / 100;
        }

        if temp > 0 {
            result.insert(result.len(), temp);
        }
    }

    let left_most_index = result.len() - 1;
    let bignumber = result
        .into_iter()
        .enumerate()
        .map(|(index, value)| {
            if index == left_most_index {
                format!("{}", value)
            } else {
                format!("{:0>2}", value)
            }
        })
        .collect::<Vec<String>>()
        .into_iter()
        .rev()
        .map(|value| value.to_string())
        .collect::<Vec<String>>()
        .join("");

    println!("{}", bignumber);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    extraLongFactorials(n);
}
