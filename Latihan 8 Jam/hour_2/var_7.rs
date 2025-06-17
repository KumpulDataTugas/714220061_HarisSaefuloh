fn main(){
    let num = funt(100);
    println!("The value of num is: {}", num);
}

fn funt(num: i32) -> i32 {
    num + 200
}