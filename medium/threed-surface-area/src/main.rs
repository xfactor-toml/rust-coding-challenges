use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'surfaceArea' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY A as parameter.
 */

fn surfaceArea(A: &[Vec<i32>]) -> i32 {
    let row = A.len();
    let col = A[0].len();

    let mut surface = 0;

    for i in 0..row {
        for j in 0..col {
            surface += 2;

            if i == 0 {
                surface += A[i][j];
            } else {
                surface += (A[i][j] - A[i-1][j]).max(0);
            }

            if j == 0 {
                surface += A[i][j];
            } else {
                surface += (A[i][j] - A[i][j-1]).max(0);
            }

            if i == row - 1 {
                surface += A[i][j];
            } else {
                surface += (A[i][j] - A[i+1][j]).max(0);
            }

            if j == col - 1 {
                surface += A[i][j];
            } else {
                surface += (A[i][j] - A[i][j+1]).max(0);
            }
        }
    }

    surface
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let H = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let W = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let mut A: Vec<Vec<i32>> = Vec::with_capacity(H as usize);

    for i in 0..H as usize {
        A.push(Vec::with_capacity(W as usize));

        A[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = surfaceArea(&A);

    println!("{}", result);

    // writeln!(&mut fptr, "{}", result).ok();
}
