mod cipher;
use cipher::{encrypt, decrypt, print_grid};
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
    let mut encrypt_flag = false;
    let mut decrypt_flag = false;
    let mut input = None;
    let mut key = None;
    let mut print_grid_flag = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--encrypt" => {
                encrypt_flag = true;
                input = args.get(i + 1).cloned();
                i += 1;
            }
            "--decrypt" => {
                decrypt_flag = true;
                input = args.get(i + 1).cloned();
                i += 1;
            }
            "--key" => {
                key = args.get(i + 1).map(|s| s.as_str());
                i += 1;
            }
            "--print-grid" => {
                print_grid_flag = true;
            }
            _ => {}
        }
        i += 1;
    }

    if print_grid_flag {
        print_grid(key);
    }
    if let Some(data) = input {
        if encrypt_flag {
            let encoded = encrypt(&data, key);
            let output: Vec<String> = encoded.iter().map(|(x, y)| format!("[{}|{}]", x, y)).collect();
            println!("{}", output.join(","));
        } else if decrypt_flag {
            let tiles = parse_tiles(&data);
            let decoded = decrypt(&tiles, key);
            println!("{}", decoded);
        } else {
            println!("Usage:\n  --encrypt TEXT\n  --decrypt \"[x|y],[a|b]\" [--key SECRET] [--print-grid]");
        }
    } else if !print_grid_flag {
        println!("Usage:\n  --encrypt TEXT\n  --decrypt \"[x|y],[a|b]\" [--key SECRET] [--print-grid]");
    }
}
