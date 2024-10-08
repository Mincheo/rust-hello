use std::env;
fn main() {
    let args:Vec<_> = env::args().collect();
    if args.len() > 1{
        println!("Hello, {}", args[1])
    }
}
