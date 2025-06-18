use rand::Rng;

fn main() {
    let number = rand::thread_rng().gen_range(1..=10);
    println!("Random number: {}", number);
}