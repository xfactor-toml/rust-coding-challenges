use std::io::{self, BufRead};

/*
 * Complete the 'matrixRotation' function below.
 *
 * The function accepts following parameters:
 *  1. 2D_INTEGER_ARRAY matrix
 *  2. INTEGER r
 */
fn get_position(m: usize, n: usize, layer: usize, step: usize) -> (usize, usize) {
    let col = n - 2 * layer;
    let row = m - 2 * layer;

    if step < col {
        (layer, layer + step)
    } else if step >= col && step < col + row - 1 {
        (layer + step - col + 1, col + layer - 1)
    } else if step >= col + row - 1 && step < col * 2 + row - 2 {
        (layer + row - 1, n - 1 - layer + (col + row - 2) - step)
    } else {
        (m - 1 - layer + (col * 2 + row - 3) - step, layer)
    }
}

fn matrixRotation(matrix: &[Vec<i32>], r: i32) {
    let m = matrix.len();
    let n = matrix[0].len();
    let layer = m.min(n) / 2;
    let max_border_length = m * 2 + n * 2 - 4;
    let mut rotated_matrix =  vec![vec![0; n]; m];

    for i in 0..layer {
        let border_length = max_border_length - i * 8;

        for j in 0 .. border_length {
            let (x, y) = get_position(m, n, i, j);
            let (x_origin, y_origin) = get_position(m, n, i, (j + r as usize) % border_length);

            rotated_matrix[x][y] = matrix[x_origin][y_origin];
        }
    }

    for row in rotated_matrix.iter() {
        println!("{}", row.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let m = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let n = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let r = first_multiple_input[2].trim().parse::<i32>().unwrap();

    let mut matrix: Vec<Vec<i32>> = Vec::with_capacity(m as usize);

    for i in 0..m as usize {
        matrix.push(Vec::with_capacity(n as usize));

        matrix[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    matrixRotation(&matrix, r);
}
