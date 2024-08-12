use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

fn main() {
    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".into());
    let release_dir = format!("{}/release", target_dir);
    let binary_path = PathBuf::from(&release_dir).join("leli");

    println!("The binary is located at: {}", binary_path.display());

    print!("Do you want to make 'leli' available system-wide? (y/N): ");
    io::stdout().flush().unwrap();

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();
    let answer = answer.trim().to_lowercase();

    if answer == "y" || answer == "yes" {
        let dest = "/usr/local/bin/leli";
        match fs::copy(&binary_path, dest) {
            Ok(_) => println!("'leli' is now available system-wide at {}", dest),
            Err(e) => eprintln!("Failed to copy 'leli' to {}: {}", dest, e),
        }
    } else {
        println!("'leli' was not installed system-wide.");
    }
}
