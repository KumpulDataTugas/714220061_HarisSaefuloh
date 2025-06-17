mod my_module {
    pub fn a() {
        println!("function a");
    }

    fn b() {
        println!("function b");
    }
}

fn main() {
    my_module::a();
    my_module::b();
}