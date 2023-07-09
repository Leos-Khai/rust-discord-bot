use std::env;
use std::fs;

fn main() {
    let profile = env::var("PROFILE").unwrap();

    if profile == "debug" {
        // Copy the config.txt file to the target/debug folder
        fs::copy("config.txt", "target/debug/config.txt").expect("Failed to copy config.txt");
    } else if profile == "release" {
        // Copy the config.txt file to the target/release folder
        fs::copy("config.txt", "target/release/config.txt").expect("Failed to copy config.txt");
    }
}
