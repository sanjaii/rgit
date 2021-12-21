use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        init()
    }
}
fn init(){
    println!("Hello,World");
}
