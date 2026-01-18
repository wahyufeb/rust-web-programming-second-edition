// Interacting with Cargo

use std::env;
fn main() {
    let args: Vec<String> =  env::args().collect();
    let path: &str = &args[0];
    if path.contains("/debug/") {
        println!("Debug is running");
    }
    else if path.contains("/release/") {
        println!("Release is running");
    }
    else {
        panic!("The setting is neither debug or release");
    }
}