fn clone_example() {
    let s1 = String::from("Rust");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn main() {
    clone_example();
}