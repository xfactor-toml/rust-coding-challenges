use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'prims' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. 2D_INTEGER_ARRAY edges
 *  3. INTEGER start
 */

fn prims(n: i32, edges: &[Vec<i32>], start: i32) -> i32 {
    let node_count = n as usize;
    let st = start as usize - 1;

    let mut weights: Vec<Vec<i32>> = vec![vec![i32::MAX; node_count]; node_count];
    let mut connects: Vec<Vec<usize>> = vec![vec![]; node_count];

    for edge in edges.iter() {
        let from = edge[0] as usize - 1;
        let to = edge[1] as usize - 1;
        let weight = edge[2];

        if weight < weights[from][to] {
            weights[from][to] = weight;
            weights[to][from] = weight;
            
        }

        if weights[from][to] < i32::MAX {
            connects[from].push(to);
            connects[to].push(from);
        }
    }

    let mut visited_nodes: Vec<usize> = Vec::new();
    let mut visited: Vec<bool> = vec![false; node_count];
    let mut result = 0;

    visited_nodes.push(st);
    visited[st] = true;

    while visited_nodes.len() < node_count {
        let mut min_weight = i32::MAX;
        let mut min_node = 0;

        for &current in visited_nodes.iter() {
            for &next in connects[current].iter() {
                if visited[next] {
                    continue;
                }

                let weight = weights[current][next];

                if weight < min_weight {
                    min_weight = weight;
                    min_node = next;
                }
            }
        }

        result += min_weight;
        visited_nodes.push(min_node);
        visited[min_node] = true;
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

    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let mut edges: Vec<Vec<i32>> = Vec::with_capacity(m as usize);

    for i in 0..m as usize {
        edges.push(Vec::with_capacity(3_usize));

        edges[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let start = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = prims(n, &edges, start);

    // writeln!(&mut fptr, "{}", result).ok();

    println!("{}", result);
}
