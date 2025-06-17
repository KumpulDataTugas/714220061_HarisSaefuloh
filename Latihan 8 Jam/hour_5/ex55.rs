fn main() {
    let s=String::from("R in 8 Hours");
    let n=cal(s);
    println!("Value of the string is: {}",s);
    println!("Length of the string is: {}",n);
}

fn cal(s:String) -> usize {
    s.len()
}