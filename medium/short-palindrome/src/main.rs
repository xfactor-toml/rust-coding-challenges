use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'shortPalindrome' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING s as parameter.
 */

fn shortPalindrome(s: &str) -> i32 {
    let mut table_i = vec![0;26];
    let mut table_ij = vec![vec![0;26];26];
    let mut table_ijj = vec![0;26];
    let mut result = 0;
    let modulo = 1000000007;

    for ch in s.chars() {
        let i = ch as usize - 'a' as usize;

        result = (result + table_ijj[i]) % modulo;

        for j in 0..26 {
            table_ijj[j] = (table_ijj[j] + table_ij[i][j]) % modulo;
            table_ij[i][j] = (table_ij[i][j] + table_i[j]) % modulo;
        }
        
        table_i[i] += 1;
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = shortPalindrome(&s);

    println!("{}", result);

    // writeln!(&mut fptr, "{}", result).ok();
}
