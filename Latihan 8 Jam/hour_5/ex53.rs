enum Language{
    JS,
    GO,
    VB,
}

fn program(var:Language){
    match var{
        Language::JS=> println!("JS in 8 Hours"),
        Language::GO=> println!("GO in 8 Hours"),
        Language::VB=> println!("VB in 8 Hours"),
    }
}

fn main(){
    program(Language::JS);
    program(Language::GO);
    program(Language::VB);
}
