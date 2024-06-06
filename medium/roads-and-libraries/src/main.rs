use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashMap;

/*
 * Complete the 'roadsAndLibraries' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER c_lib
 *  3. INTEGER c_road
 *  4. 2D_INTEGER_ARRAY cities
 */

fn roadsAndLibraries(n: i32, c_lib: i32, c_road: i32, cities: &[Vec<i32>]) -> i64 {
    if c_lib <= c_road {
        n as i64 * c_lib as i64 
    } else {
        let mut connection: Vec<Vec<usize>> = vec![vec![]; n as usize];
        let mut visited: Vec<bool> = vec![false; n as usize];
    
        let mut cost = 0i64;
    
        for possible_connection in cities.iter() {
            let &start = possible_connection.get(0).unwrap();
            let &end = possible_connection.get(1).unwrap();
    
            connection[start as usize - 1].push(end as usize - 1);
            connection[end as usize - 1].push(start as usize - 1);
        }
    
        for city in 0..n as usize {
            if visited[city] {
                continue;
            } else {
                visited[city] = true;
                cost += c_lib as i64;
    
                let mut no_visit:Vec<usize> = connection[city].clone();
    
                loop {
                    if no_visit.is_empty() {
                        break;
                    }
    
                    let next_city = no_visit.pop().unwrap();
    
                    if !visited[next_city] {
                        no_visit.extend(connection[next_city].clone());
                        visited[next_city] = true;
                        cost += c_road as i64;
                    }
                }
            }
        }
    
        cost
    }
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

        let c_lib = first_multiple_input[2].trim().parse::<i32>().unwrap();

        let c_road = first_multiple_input[3].trim().parse::<i32>().unwrap();

        let mut cities: Vec<Vec<i32>> = Vec::with_capacity(m as usize);

        for i in 0..m as usize {
            cities.push(Vec::with_capacity(2_usize));

            cities[i] = stdin_iterator.next().unwrap().unwrap()
                .trim_end()
                .split(' ')
                .map(|s| s.to_string().parse::<i32>().unwrap())
                .collect();
        }

        let result = roadsAndLibraries(n, c_lib, c_road, &cities);

        println!("{}", result);

        // writeln!(&mut fptr, "{}", result).ok();
    }
}
