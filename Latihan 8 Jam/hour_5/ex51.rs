struct Member {
    id: i32,
    name: String,
    working: bool,
}

fn main() {
    let clerk = Member {
        id: 016320,
        name: "Smith".to_string(),
        working: true,
    };
    println!("ID is {}", clerk.id);
    println!("Name is {}", clerk.name);
    println!("Working is {}", clerk.working);
}