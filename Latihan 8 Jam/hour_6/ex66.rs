mod my_module {
    pub fn a() {
        println!("function a");
        b();
    }

    fn b() {
        println! ("function b");
    }
}

fn main() {
    my_module::a();
}