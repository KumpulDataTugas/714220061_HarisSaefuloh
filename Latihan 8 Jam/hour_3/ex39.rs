fn main(){
    let a = [0, 10, 20, 30, 40, 50, 60];
    let slice = &a[2..5];
    println!("{}",slice[0] );
    println!("{}",slice[1] );
    println!("{}",slice[2] );
}