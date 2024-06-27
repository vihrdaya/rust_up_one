use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(1..10);
    println!("Starting with random number: {}", random_number);
    for current_number in 0..random_number {
        if current_number % 2 == 0 {
            println!("Hello, world!, number {}", current_number);
        }
    }
}
