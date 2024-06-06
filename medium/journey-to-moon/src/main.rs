use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'journeyToMoon' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. 2D_INTEGER_ARRAY astronaut
 */

fn journeyToMoon(n: i32, astronaut: &[Vec<i32>]) -> i64 {
    let mut connection: Vec<Vec<usize>> = vec![vec![]; n as usize];
    let mut checked: Vec<bool> = vec![false; n as usize];

    let mut result = 0;

    for possible_connection in astronaut.iter() {
        let &start = possible_connection.get(0).unwrap();
        let &end = possible_connection.get(1).unwrap();

        connection[start as usize].push(end as usize);
        connection[end as usize].push(start as usize);
    }

    let mut member_counts: Vec<i64> = vec![];

    for member in 0..n as usize {
        if checked[member] {
            continue;
        } else {
            checked[member] = true;

            let mut member_count = 1;

            let mut no_checked:Vec<usize> = connection[member].clone();

            loop {
                if no_checked.is_empty() {
                    break;
                }

                let next_member = no_checked.pop().unwrap();

                if !checked[next_member] {
                    no_checked.extend(connection[next_member].clone());
                    checked[next_member] = true;
                    member_count += 1;
                }
            }

            member_counts.push(member_count);
        }
    }

    for i in 0..member_counts.len() {
        for j in (i+1)..member_counts.len() {
            result += member_counts[i] * member_counts[j];
        }
    }

    result
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

    let p = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let mut astronaut: Vec<Vec<i32>> = Vec::with_capacity(p as usize);

    for i in 0..p as usize {
        astronaut.push(Vec::with_capacity(2_usize));

        astronaut[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = journeyToMoon(n, &astronaut);

    println!("{}", result);

    // writeln!(&mut fptr, "{}", result).ok();
}
