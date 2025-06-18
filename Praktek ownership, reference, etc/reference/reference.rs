fn reference_example() {
    let s1 = String::from("Rust");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main() {
    reference_example();
}