fn borrowing_example() {
    let mut s = String::from("Hello");
    change(&mut s);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn main() {
    borrowing_example();
}