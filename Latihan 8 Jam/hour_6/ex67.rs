mod sup_module{
    fn a() -> i32 {
        100
    }
    pub mod sub_module {
        use super::a;
        pub fn b() {
            println!("{}",a());
        }
    }
}

fn main() {
    sup_module::sub_module::b();
}
