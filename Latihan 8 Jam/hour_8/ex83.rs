pub trait Show {
    fn show(&self);
}

impl<T> Show for T

where T: ToString {
    fn show (&self){
        print!("{}",self.to_string());
    }
}

fn main(){
    String::from("C# in 8 Hours").show();
}