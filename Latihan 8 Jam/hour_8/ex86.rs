fn main() {
    let mut capacity = "Hard disk capacity: 5000".to_string();
    {
        let mut my_closure = | c: char |{capacity.push(c)};
        my_closure('G');
    }
    println!("{:?}", capacity);
}