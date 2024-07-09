use std::io::{self, BufRead};
use std::collections::VecDeque;
use std::collections::HashMap;

/*
 * Complete the 'getCost' function below.
 *
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

fn getCost(g_nodes: i32, g_from: &[i32], g_to: &[i32], g_weight: &[i32]) {
    let n = g_nodes as usize;

    let mut costs: Vec<Vec<i32>> = vec![vec![0; n + 1]; n + 1];
    let mut connections: Vec<Vec<usize>> = vec![vec![]; n + 1];

    for i in 0..g_from.len() {
        let from = g_from[i] as usize;
        let to = g_to[i] as usize;
        let weight = g_weight[i];

        costs[from][to] = weight;
        costs[to][from] = weight;
        
        connections[from].push(to);
        connections[to].push(from);
    }

    let mut visited: Vec<usize> = vec![1];
    let mut min_costs: Vec<i32> = vec![0; n + 1];

    min_costs[n] = i32::MAX;

    loop {
        let (mut cost, mut from, mut to) = (i32::MAX, 0, 0);

        for &current in visited.iter() {
            if current == n {
                continue;
            }

            for &next in connections[current].iter() {
                if (!visited.contains(&next) || next == n)
                    && min_costs[current].max(costs[current][next]) < cost
                    && min_costs[current].max(costs[current][next]) < min_costs[n]
                {
                    to = next;
                    from = current;
                    cost = min_costs[from].max(costs[from][to]);
                }
            }
        }

        if cost == i32::MAX {
            break;
        }

        visited.push(to);
        min_costs[to] = cost;
    }


    if min_costs[n] == i32::MAX {
        println!("NO PATH EXISTS");
    } else {
        println!("{}", min_costs[n]);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

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

    getCost(g_nodes, &g_from, &g_to, &g_weight);
}
