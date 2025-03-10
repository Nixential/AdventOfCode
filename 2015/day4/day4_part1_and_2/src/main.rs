use md5::{Md5, Digest};
use std::time::Instant;

fn main() {
    let input = "iwrupvqb"; // Base input string
    let mut nonce: u64 = 0; // Start nonce
    let target_prefix = "000000"; // Five leading zeroes
    
    let start_time = Instant::now(); // Track execution time

    loop {
        let data = format!("{}{}", input, nonce); // Concatenate input + nonce
        
        // Compute MD5 hash
        let mut hasher = Md5::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();

        // Convert to hex string
        let hash_string = format!("{:x}", result);

        // Check if the hash starts with five zeroes
        if hash_string.starts_with(target_prefix) {
            println!("✅ Found nonce: {}", nonce);
            println!("MD5 Hash: {}", hash_string);
            break;
        }

        nonce += 1; // Increment nonce
    }

    let elapsed_time = start_time.elapsed();
    println!("⏳ Time taken: {:?}", elapsed_time);
}
