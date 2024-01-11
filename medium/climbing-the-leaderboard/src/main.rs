use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'climbingLeaderboard' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY ranked
 *  2. INTEGER_ARRAY player
 */

fn climbingLeaderboard(ranked: &[i32], player: &[i32]) -> Vec<i32> {
    // Timeout Failed Solution
    let mut ranks = vec![];

    let mut rank = 1;
    let mut last_ranked_score = 0;
    let mut last_rank = 0;

    for &score in player.iter().rev() {
        rank -= 1;
        
        for index in last_rank..ranked.len() {
            if score < ranked[index] && last_ranked_score != ranked[index] {
                last_ranked_score = ranked[index];
                rank += 1;
            }

            if score >= ranked[index] || index == ranked.len() - 1 {
                last_rank = index;
                break;
            }
        }

        rank += 1;

        ranks.push(rank);
    }

    ranks = ranks.into_iter().rev().collect();

    ranks
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _ranked_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let ranked: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let _player_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let player: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = climbingLeaderboard(&ranked, &player);

    for i in 0..result.len() {
        // write!(&mut fptr, "{}", result[i]).ok();

        // if i != result.len() - 1 {
        //     writeln!(&mut fptr).ok();
        // }

        println!("{}", result[i]);
    }


    // writeln!(&mut fptr).ok();
}
