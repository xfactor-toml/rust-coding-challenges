use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'formingMagicSquare' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY s as parameter.
 */

fn formingMagicSquare(s: &[Vec<i32>]) -> i32 {
    let mut sums = vec![0; 8];
    let sample = &[vec![8, 3, 4], vec![1, 5, 9], vec![6, 7, 2]];

    for i in 0..3 {
        for j in 0..3 {
            sums[0] += (sample[i][j] - s[i][j]).abs();
            sums[1] += (sample[i][2 - j] - s[i][j]).abs();
            sums[2] += (sample[2 - i][j] - s[i][j]).abs();
            sums[3] += (sample[2 - i][2 - j] - s[i][j]).abs();
            sums[4] += (sample[j][i] - s[i][j]).abs();
            sums[5] += (sample[j][2 - i] - s[i][j]).abs();
            sums[6] += (sample[2 - j][i] - s[i][j]).abs();
            sums[7] += (sample[2 - j][2 - i] - s[i][j]).abs();
        }
    }

    *sums.iter().min().unwrap()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let mut s: Vec<Vec<i32>> = Vec::with_capacity(3_usize);

    for i in 0..3_usize {
        s.push(Vec::with_capacity(3_usize));

        s[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = formingMagicSquare(&s);

    // writeln!(&mut fptr, "{}", result).ok();
    println!("{}", result);
}
