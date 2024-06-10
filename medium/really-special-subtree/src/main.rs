use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'kruskals' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts WEIGHTED_INTEGER_GRAPH g as parameter.
 */

/*
 * For the weighted graph, <name>:
 *
 * 1. The number of nodes is <name>_nodes.
 * 2. The number of edges is <name>_edges.
 * 3. An edge exists between <name>_from[i] and <name>_to[i]. The weight of the edge is <name>_weight[i].
 *
 */

fn kruskals(g_nodes: i32, g_from: &[i32], g_to: &[i32], g_weight: &[i32]) -> i32 {
    let nodes = g_nodes as usize;
    let mut edges: Vec<Vec<i32>> = vec![vec![i32::MAX; nodes + 1]; nodes + 1];
    let mut connections: Vec<Vec<usize>> = vec![vec![]; nodes + 1];
    
    let mut visited: Vec<usize> = vec![];
    let mut distance = 0;

    for i in 0..g_from.len() {
        let from = g_from[i] as usize;
        let to = g_to[i] as usize;
        let weight = g_weight[i];

        if edges[from][to] == i32::MAX {
            connections[from].push(to);
            connections[to].push(from);
        }

        if weight < edges[from][to] {
            edges[from][to] = weight;
            edges[to][from] = weight;
        }
    }

    visited.push(1);

    for _ in 1..nodes {
        let mut min_weight = i32::MAX;
        let mut min_node_sum = usize::MAX;
        let mut min_node = 0;

        for &node in visited.iter() {
            for &next in connections[node].iter() {
                if visited.contains(&next) {
                    continue;
                }

                if min_weight > edges[node][next] || (min_weight == edges[node][next] && min_node_sum > node + next) {
                    min_weight = edges[node][next];
                    min_node_sum = node + next;
                    min_node = next;
                }
            }
        }

        visited.push(min_node);
        distance += min_weight;
    }

    distance
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let g_nodes_edges: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let g_nodes = g_nodes_edges[0].trim().parse::<i32>().unwrap();
    let g_edges = g_nodes_edges[1].trim().parse::<i32>().unwrap();

    let mut g_from: Vec<i32> = Vec::with_capacity(g_edges as usize);
    let mut g_to: Vec<i32> = Vec::with_capacity(g_edges as usize);
    let mut g_weight: Vec<i32> = Vec::with_capacity(g_edges as usize);

    for _ in 0..g_edges {
        let g_from_to: Vec<String> = stdin_iterator.next().unwrap().unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let g_from_temp = g_from_to[0].trim().parse::<i32>().unwrap();
        let g_to_temp = g_from_to[1].trim().parse::<i32>().unwrap();
        let g_weight_temp = g_from_to[2].trim().parse::<i32>().unwrap();

        g_from.push(g_from_temp);
        g_to.push(g_to_temp);
        g_weight.push(g_weight_temp);
    }

    let res = kruskals(g_nodes, &g_from, &g_to, &g_weight);

    // Write your code here.

    println!("{}", res);
}
