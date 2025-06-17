struct Square {
    len: i32,
    wid: i32,
}

fn main() {
    let table = Square { len: 10, wid: 8 };
    println!("The area is {}", table.len * table.wid);
}