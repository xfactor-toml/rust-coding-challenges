use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::VecDeque;
use std::collections::HashMap;

/*
 * Complete the 'hanoi' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY posts as parameter.
 */

fn hanoi(posts: &[i32]) -> i32 {
    let n = posts.len();
    let target = vec![0; n];
    
    let initial_state: Vec<i32> = posts.to_vec().iter().map(|v| v - 1).collect();
    
    let mut queue: VecDeque<(Vec<i32>, i32)> = VecDeque::new();
    let mut visited: HashMap<Vec<i32>, bool> = HashMap::new();
    
    queue.push_back((initial_state.clone(), 0));
    visited.insert(initial_state, true);

    loop {
        let (current_state, steps) = queue.pop_front().unwrap();

        if current_state == target {
            return steps;
        }

        // find valid next moves
        let mut smallest_discs = vec![usize::MAX; 4];

        for (disc, &pos) in current_state.iter().enumerate() {
            if smallest_discs[pos as usize] > disc {
                smallest_discs[pos as usize] = disc
            }
        }

        for i in 0..4 {
            for j in 0..4 {
                if i != j && smallest_discs[i] < smallest_discs[j] {
                    let mut next_state = current_state.clone();
                    next_state[smallest_discs[i]] = j as i32;

                    if !visited.contains_key(&next_state) {
                        queue.push_back((next_state.clone(), steps + 1));
                        visited.insert(next_state, true);
                    }
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let loc: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let res = hanoi(&loc);

    println!("{}", res);

    // writeln!(&mut fptr, "{}", res).ok();
}
