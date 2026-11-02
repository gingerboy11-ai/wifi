use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use std::io::{self, Write};

const DEMO_PASSWORD: &str = "university-demo-2026";
const SALT: &[u8] = b"BrutiFi-DemoSalt";
const ITERATIONS: u32 = 100_000;

fn derive_key(input: &str) -> String {
    let mut output = [0u8; 32];
    pbkdf2_hmac::<Sha256>(input.as_bytes(), SALT, ITERATIONS, &mut output);
    hex::encode(output)
}

fn main() {
    println!("BrutiFi - local cryptography learning demo");
    println!("This program checks one phrase against a built-in demo phrase.");
    println!("It does not scan Wi-Fi networks or process captures.\n");

    print!("Enter the demo phrase to check: ");
    let _ = io::stdout().flush();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("Could not read input.");
        return;
    }

    let entered = input.trim_end_matches(['\r', '\n']);
    let entered_hash = derive_key(entered);
    let expected_hash = derive_key(DEMO_PASSWORD);

    println!("\nPBKDF2-HMAC-SHA256 result (hex):");
    println!("{entered_hash}");

    if entered_hash == expected_hash {
        println!("\nMatch: the entered phrase is the built-in demo phrase.");
    } else {
        println!("\nNo match: the phrase is different from the built-in demo phrase.");
    }

    println!("\nPress Enter to close...");
    let mut pause = String::new();
    let _ = io::stdin().read_line(&mut pause);
}
