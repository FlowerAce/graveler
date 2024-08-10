use rand::Rng;

fn main() {
    let items = [1, 2, 3, 4];
    let mut numbers = [0, 0, 0, 0];
    let mut rolls = 0;
    let mut max_ones = 0;
    let mut rng = rand::thread_rng();

    while numbers[0] < 177 && rolls < 1_000_000_000 {
        numbers = [0, 0, 0, 0];
        for _ in 0..231 {
            let roll = items[rng.gen_range(0..items.len())];
            numbers[roll - 1] += 1;
        }
        rolls += 1;
        if numbers[0] > max_ones {
            max_ones = numbers[0];
        }
    }

    println!("Highest Ones Roll: {}", max_ones);
    println!("Number of Roll Sessions: {}", rolls);
}
