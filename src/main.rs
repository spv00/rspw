use std::env;

mod lib;
fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>().to_owned();
    let config = lib::Config::parse(args.clone()).unwrap();
    println!("{}", args.join(" "));
    println!("{}", lib::generate_final(config));
}