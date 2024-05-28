use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'steadyGene' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING gene as parameter.
 */

fn steadyGene(gene: &str) -> i32 {
    let mut excess: Vec<i32> = vec![0;4];

    let gene_vec: Vec::<char> = gene.chars().collect();
    let gen_len = gene.len();

    for &letter in gene_vec.iter() {
        excess[get_letter_index(letter)] += 1;
    }

    excess = excess.iter().map(|&value| (value - gen_len as i32 / 4).max(0)).collect();

    let mut min_length = usize::MAX;
    let excess_origin = excess.clone();

    if excess.iter().all(|&value| value == 0) {
        0i32
    } else {
        let mut left: usize = 0;

        for right in 0..gen_len {
            let rightmost = get_letter_index(gene_vec[right]);
    
    
            if excess_origin[rightmost] > 0 {
                excess[rightmost] -= 1;
            }
    
            while excess.iter().all(|&value| value <= 0) {
                min_length = min_length.min(right + 1 - left);
    
                let leftmost = get_letter_index(gene_vec[left]);
    
                if excess_origin[leftmost] > 0 {
                    excess[leftmost] += 1;
                }
    
                left += 1;
            }
        }
    
        min_length as i32
    }
}

fn get_letter_index(letter: char) -> usize {
    match letter {
        'A' => 0,
        'C' => 1,
        'T' => 2,
        _ => 3
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let gene = stdin_iterator.next().unwrap().unwrap();

    let result = steadyGene(&gene);

    println!("{}", result);

    // writeln!(&mut fptr, "{}", result).ok();
}
