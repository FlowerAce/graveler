use rand::Rng;
use rayon::prelude::*;

fn main() {
    let rolls_limit = 1_000_000;

    let max_ones = (0..rolls_limit)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            let mut count_ones = 0;

            for _ in 0..231 {
                if rng.gen_range(1..=4) == 1 {
                    count_ones += 1;
                }
            }
            count_ones
        })
        .filter(|&ones| ones < 177)
        .max()
        .unwrap_or(0);

    println!("Highest Ones Roll: {}", max_ones);
    println!("Number of Roll Sessions: {}", rolls_limit);
}
