use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

use std::collections::VecDeque;

/*
 * Complete the 'bfs' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER m
 *  3. 2D_INTEGER_ARRAY edges
 *  4. INTEGER s
 */

fn bfs(n: i32, m: i32, edges: &[Vec<i32>], s: i32) -> Vec<i32> {
    let node_count = n as usize;
    let start = s as usize - 1;

    let mut connects: Vec<Vec<usize>> = vec![vec![]; node_count];
    let mut distances: Vec<i32> = vec![-1; node_count];
    let mut visited: Vec<bool> = vec![false; node_count];

    for edge in edges.iter() {
        let from = edge[0] as usize - 1;
        let to = edge[1] as usize - 1;
        
        connects[from].push(to);
        connects[to].push(from);
    }

    let mut queue: VecDeque<(usize, i32)> = VecDeque::new();

    queue.push_back((start, 0));
    visited[start] = true;

    while !queue.is_empty() {
        let (current, distance) = queue.pop_front().unwrap();

        distances[current] = distance;

        for &next in connects[current].iter() {
            if !visited[next] {
                queue.push_back((next, distance + 6));
                visited[next] = true;
            }
        }
    }

    distances
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let q = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..q {
        let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let n = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let mut edges: Vec<Vec<i32>> = Vec::with_capacity(m as usize);

        for i in 0..m as usize {
            edges.push(Vec::with_capacity(2_usize));

            edges[i] = stdin_iterator.next().unwrap().unwrap()
                .trim_end()
                .split(' ')
                .map(|s| s.to_string().parse::<i32>().unwrap())
                .collect();
        }

        let s = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let result = bfs(n, m, &edges, s);

        for i in 0..result.len() {
            // write!(&mut fptr, "{}", result[i]).ok();
            if  result[i] != 0 {
                println!("{}", result[i]);
            }

            // if i != result.len() - 1 {
            //     write!(&mut fptr, " ").ok();
            // }
        }

        // writeln!(&mut fptr).ok();
    }
}
