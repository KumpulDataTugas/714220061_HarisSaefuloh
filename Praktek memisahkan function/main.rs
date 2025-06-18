mod utils;

fn main() {
    utils::say_hello("Rustacean");

    let result = utils::add(5, 7);
    println!("5 + 7 = {}", result);
}