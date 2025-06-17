struct Circle{
    radius: f32,
}

trait Calculate{
    fn area(&self) -> f32;
}

impl Calculate for Circle{
    fn area(&self) -> f32{
        std::f32::consts::PI * self.radius * self.radius
    }
}

fn main(){
    let obj = Circle { radius : 2000.00};
    println!("The Circle area is: {}", obj.area());
}
