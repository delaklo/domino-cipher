
fn default_alphabet() -> Vec<char> {
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.,!?-_():*#&".chars().collect()
}
pub fn create_polybius_grid(key: Option<&str>) -> [[char; 7]; 7] {
    let mut seen = std::collections::HashSet::new();
    let mut ordered = Vec::new();

    if let Some(k) = key {
        for ch in k.to_uppercase().chars() {
            if default_alphabet().contains(&ch) && seen.insert(ch) {
                ordered.push(ch);
            }
        }
    }

    for ch in default_alphabet() {
        if seen.insert(ch) {
            ordered.push(ch);
        }
    }
    let mut grid = [[' '; 7]; 7];
    for i in 0..7 {
        for j in 0..7 {
            grid[i][j] = *ordered.get(i * 7 + j).unwrap_or(&' ');
        }
    }

    grid
}

pub fn encrypt(text: &str, key: Option<&str>) -> Vec<(u8, u8)> {
    let grid = create_polybius_grid(key);
    let mut tiles = Vec::new();

    for ch in text.to_uppercase().chars() {
        for (i, row) in grid.iter().enumerate() {
            if let Some(j) = row.iter().position(|&c| c == ch) {
                tiles.push((i as u8, j as u8));
            break;
            }
        }
    }
    tiles
}

pub fn decrypt(tiles: &[(u8, u8)], key: Option<&str>) -> String {
   
    let grid = create_polybius_grid(key);
    let mut result = String::new();

    for &(i, j) in tiles {
        if i < 7 && j < 7 {
            result.push(grid[i as usize][j as usize]);
        } else {
            result.push('?');
        }
    }
    result
}

pub fn print_grid(key: Option<&str>) { 
    let grid = create_polybius_grid(key);
    println!("grid:"); 
    for row in grid.iter() {
        for &ch in row.iter() {
            print!("{:>3}", ch);
        }
        println!();
    }
}
