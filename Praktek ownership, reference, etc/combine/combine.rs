fn combine_all_examples() {
    let mut s = String::from("Hello");
    let len = calculate_length(&s);
    change(&mut s);
    let s2 = s.clone();
    let part = &s2[0..5];
    let x = 5;
    let y = x;
    println!("len: {}, s: {}, part: {}, x: {}, y: {}", len, s, part, x, y);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", Rustaceans!");
}

fn main() {
    combine_all_examples();
}