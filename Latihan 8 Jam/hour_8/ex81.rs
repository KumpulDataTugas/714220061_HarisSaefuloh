struct Circle{
    radius: f32,
}

impl Circle{
    fn area(&self) -> f32 {
    std::f32::consts::PI * self.radius * self.radius
    }
}
fn main(){
    let obj = Circle { radius : 2000.00};
    println!("The Circle area is: {}", obj.area());
}