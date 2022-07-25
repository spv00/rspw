mod lib;
fn main() {
    let config: lib::Config = lib::Config::new(5, true, true, true, false);
    println!("{}", lib::generate_pass(5, &config));
}