// 1. Struct Dasar
struct User {
    username: String,
    age: u32,
}

fn print_user(user: &User) {
    println!("Username: {}, Age: {}", user.username, user.age);
}

// 2. Struct dengan Function Builder
struct Point {
    x: i32,
    y: i32,
}

fn new_point(x: i32, y: i32) -> Point {
    Point { x, y }
}

// 3. Struct dengan Method (impl)
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// 4. Tuple Struct
struct Color(u8, u8, u8);

fn print_color(c: &Color) {
    println!("Color RGB({}, {}, {})", c.0, c.1, c.2);
}

// 5. Unit Struct
struct AlwaysEqual;

fn check_unit(_: AlwaysEqual) {
    println!("This struct has no fields!");
}

// 6. Function mengembalikan struct
struct Book {
    title: String,
    pages: u32,
}

fn create_book(title: &str, pages: u32) -> Book {
    Book {
        title: title.to_string(),
        pages,
    }
}

// 7. Struct dalam Struct
struct Engine {
    horsepower: u32,
}

struct Car {
    brand: String,
    engine: Engine,
}

fn show_car(car: &Car) {
    println!("{} with {} HP", car.brand, car.engine.horsepower);
}

// 8. Struct Mutable
struct Counter {
    value: i32,
}

fn increment(counter: &mut Counter) {
    counter.value += 1;
}

// 9. Struct dengan Method Static dan Area
struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Circle { radius }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

// 10. Struct dengan Enum
enum Status {
    Active,
    Inactive,
}

struct Account {
    name: String,
    status: Status,
}

fn show_status(account: &Account) {
    match account.status {
        Status::Active => println!("{} is active", account.name),
        Status::Inactive => println!("{} is inactive", account.name),
    }
}

fn main() {
    // 1. Struct Dasar
    let user = User {
        username: "Alice".to_string(),
        age: 30,
    };
    print_user(&user);

    // 2. Struct Builder
    let p = new_point(5, 10);
    println!("Point({}, {})", p.x, p.y);

    // 3. Method Struct
    let rect = Rectangle {
        width: 4,
        height: 5,
    };
    println!("Area of rectangle: {}", rect.area());

    // 4. Tuple Struct
    let red = Color(255, 0, 0);
    print_color(&red);

    // 5. Unit Struct
    let unit = AlwaysEqual;
    check_unit(unit);

    // 6. Return Struct from Function
    let book = create_book("Rust Book", 300);
    println!("Book: {}, Pages: {}", book.title, book.pages);

    // 7. Nested Struct
    let car = Car {
        brand: "Toyota".to_string(),
        engine: Engine { horsepower: 150 },
    };
    show_car(&car);

    // 8. Mutable Struct
    let mut counter = Counter { value: 0 };
    increment(&mut counter);
    println!("Counter value: {}", counter.value);

    // 9. Static Method + Area
    let circle = Circle::new(3.0);
    println!("Area of circle: {:.2}", circle.area());

    // 10. Struct with Enum
    let acc = Account {
        name: "Bob".to_string(),
        status: Status::Active,
    };
    show_status(&acc);
}
