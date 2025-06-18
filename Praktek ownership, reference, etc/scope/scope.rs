fn scope_example() {
    {
        let s = String::from("inside scope");
        println!("{}", s);
    }
}

fn main() {
    scope_example();
}