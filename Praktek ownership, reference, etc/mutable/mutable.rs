fn mutable_example() {
    let mut s = String::from("Hello");
    s.push_str(" Rust!");
    println!("{}", s);
}

fn main() {
    mutable_example();
}