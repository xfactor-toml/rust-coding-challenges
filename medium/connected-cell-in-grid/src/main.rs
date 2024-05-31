use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'connectedCell' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY matrix as parameter.
 */

fn connectedCell(matrix: &[Vec<i32>]) -> i32 {
    let row = matrix.len();
    let col = matrix[0].len();
    let mut regions = vec![vec![false;col]; row];
    let mut max_depth = 0;

    for i in 0..row {
        for j in 0..col {
            if matrix[i][j] > 0 && regions[i][j] == false {
                let mut depth = 0;
                check_regions(&mut regions, matrix, i as i32, j as i32, &mut depth);

                if max_depth < depth {
                    max_depth = depth;
                }
            }
        }
    }

    max_depth
}

fn check_regions(regions: &mut Vec<Vec<bool>>, matrix: &[Vec<i32>], x : i32, y: i32, depth: &mut i32) {
    regions[x as usize][y  as usize] = true;
    *depth += 1;

    let steps = vec![(-1,0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];

    for (dx, dy) in &steps {
        let (new_x, new_y) = (x + dx, y + dy);

        if new_x >= 0 && new_y >= 0 && 
           new_x < matrix.len() as i32 && new_y < matrix[0].len() as i32 && 
           regions[new_x as usize][new_y as usize] == false && matrix[new_x as usize][new_y as usize] > 0 {
            check_regions(regions, matrix, new_x, new_y, depth);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let m = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut matrix: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        matrix.push(Vec::with_capacity(m as usize));

        matrix[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = connectedCell(&matrix);

    println!("{}", result);

    // writeln!(&mut fptr, "{}", result).ok();
}
