use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        init()
    }
}
fn init(){
    fs::create_dir(".rgit").expect("rgit repository already exist!");
    println!("Initialized rgit repository");
    
}
