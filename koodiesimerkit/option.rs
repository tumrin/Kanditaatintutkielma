fn return_option() -> Option<i32> {
    // Random number generator from rand crate
    let mut rng = rand::thread_rng(); 
    // Generate random number between 1 and 20 (inclusive)
    let random_num: i32 = rng.gen_range(1..=20); 
    if random_num > 10 {
        Some(random_num)
    } else {
        None
    }
}
fn main() {
    // Match return value. Match must be exhaustive i.e all possible cases must be handled
    let number = match return_option() {
        Some(number) => number,
        None => 0, // Handle missing value
    };
    println!("{}", number);
}