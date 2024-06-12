use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

use std::collections::VecDeque;

/*
 * Complete the 'quickestWayUp' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. 2D_INTEGER_ARRAY ladders
 *  2. 2D_INTEGER_ARRAY snakes
 */

fn quickestWayUp(ladders: &[Vec<i32>], snakes: &[Vec<i32>]) -> i32 {
    let mut connect: Vec<usize> = vec![0; 101];
    let mut visited: Vec<bool> = vec![false; 101];

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    let mut result = 0;

    for ladder in ladders.iter() {
        let from = ladder[0] as usize;
        let to = ladder[1] as usize;

        connect[from] = to;
    }

    for snake in snakes.iter() {
        let from = snake[0] as usize;
        let to = snake[1] as usize;

        connect[from] = to;
    }

    if connect[1] > 0 {
        queue.push_back((connect[1], 0));
    } else {
        queue.push_back((1, 0));
    }

    loop {
        if queue.is_empty() {
            result = -1;
            
            break;
        }

        let (current, step) = queue.pop_front().unwrap();

        visited[current] = true;

        if current == 100 {
            result = step as i32;

            break;
        }

        for i in 1..=6 {
            if current + i > 100 {
                continue;
            }

            let next = if connect[current + i] > 0 {
                connect[current + i]
            } else {
                current + i
            };

            if !visited[next] {
                queue.push_back((next, step + 1));
            }
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
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let mut ladders: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

        for i in 0..n as usize {
            ladders.push(Vec::with_capacity(2_usize));

            ladders[i] = stdin_iterator.next().unwrap().unwrap()
                .trim_end()
                .split(' ')
                .map(|s| s.to_string().parse::<i32>().unwrap())
                .collect();
        }

        let m = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let mut snakes: Vec<Vec<i32>> = Vec::with_capacity(m as usize);

        for i in 0..m as usize {
            snakes.push(Vec::with_capacity(2_usize));

            snakes[i] = stdin_iterator.next().unwrap().unwrap()
                .trim_end()
                .split(' ')
                .map(|s| s.to_string().parse::<i32>().unwrap())
                .collect();
        }

        let result = quickestWayUp(&ladders, &snakes);

        // writeln!(&mut fptr, "{}", result).ok();
        println!("{}", result);
    }
}
