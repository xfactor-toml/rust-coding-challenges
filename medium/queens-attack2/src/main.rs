use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'queensAttack' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER k
 *  3. INTEGER r_q
 *  4. INTEGER c_q
 *  5. 2D_INTEGER_ARRAY obstacles
 */

fn queensAttack(n: i32, k: i32, r_q: i32, c_q: i32, obstacles: &[Vec<i32>]) -> i32 {
    let mut attacks = vec![
        n - c_q, // right
        (n - c_q).min(n - r_q), // right bottom
        n - r_q, // bottom
        (c_q - 1).min(n - r_q), // left bottom
        c_q - 1, // left
        (c_q - 1).min(r_q - 1), // left top
        r_q - 1, // top
        (n - c_q).min(r_q - 1), // right top
    ];

    for obstacle in obstacles.iter() {
        if obstacle[0] == r_q {
            if obstacle[1] > c_q {
                attacks[0] = attacks[0].min(obstacle[1] - c_q - 1);
            } else {
                attacks[4] = attacks[4].min(c_q - obstacle[1] - 1);
            }
        } else if obstacle[1] == c_q {
            if obstacle[0] > r_q {
                attacks[2] = attacks[2].min(obstacle[0] - r_q - 1);
            } else {
                attacks[6] = attacks[6].min(r_q - obstacle[0] - 1);
            }
        } else if (obstacle[0] - r_q).abs() == (obstacle[1] - c_q).abs() {
            if obstacle[0] > r_q && obstacle[1] > c_q {
                attacks[1] = std::cmp::min(
                    attacks[1],
                    (obstacle[0] - r_q).min(obstacle[1] - c_q - 1)
                );
            } else if obstacle[0] > r_q && obstacle[1] < c_q {
                attacks[3] = std::cmp::min(
                    attacks[3],
                    (obstacle[0] - r_q).min(c_q - obstacle[1] - 1)
                );
            } else if obstacle[0] < r_q && obstacle[1] < c_q {
                attacks[5] = std::cmp::min(
                    attacks[5],
                    (r_q - obstacle[0]).min(c_q - obstacle[1] - 1)
                );
            } else {
                attacks[7] = std::cmp::min(
                    attacks[7],
                    (r_q - obstacle[0]).min(obstacle[1] - c_q - 1)
                );
            }
        }
    }

    attacks.iter().sum()
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

    let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let second_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let r_q = second_multiple_input[0].trim().parse::<i32>().unwrap();

    let c_q = second_multiple_input[1].trim().parse::<i32>().unwrap();

    let mut obstacles: Vec<Vec<i32>> = Vec::with_capacity(k as usize);

    for i in 0..k as usize {
        obstacles.push(Vec::with_capacity(2_usize));

        obstacles[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = queensAttack(n, k, r_q, c_q, &obstacles);

    println!("{}", result);
    // writeln!(&mut fptr, "{}", result).ok();
}
