fn main(){
    let my_closure = | num: i32 | { num + 200 };
    let num = 100;
    println!("{}", my_closure(num));
}
