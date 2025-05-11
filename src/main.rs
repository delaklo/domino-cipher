mod cipher;
use cipher::{encrypt, decrypt};
use std::env;

fn parse_tiles(input: &str) -> Vec<(u8, u8)> {
    let mut tiles = Vec::new();
    let parts = input.trim().split("],[");

    for part in parts {
        let clean = part.trim_matches(&['[', ']'][..]);
        let nums: Vec<&str> = clean.split('|').collect();
        if nums.len() == 2 {
            if let (Ok(a), Ok(b)) = (nums[0].parse::<u8>(), nums[1].parse::<u8>()) {
                tiles.push((a, b));
            }
        }
    }

    tiles
}



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 3 {
        let flag = &args[1];
        let input = &args[2];

        if flag == "--encrypt" {
            let encoded = encrypt(input);
            let output: Vec<String> = encoded.iter().map(|(x, y)| format!("[{}|{}]", x, y)).collect();
            println!("{}", output.join(","));
        } else if flag == "--decrypt" {
            let tiles = parse_tiles(input);
            let decoded = decrypt(&tiles);
            println!("{}", decoded);
        } else {
            println!("Usage:\n  --encrypt TEXT\n  --decrypt \"[x|y],[a|b]\"");
        }
    } else {
        println!("Usage:\n  --encrypt TEXT\n  --decrypt \"[x|y],[a|b]\"");
    }
}
