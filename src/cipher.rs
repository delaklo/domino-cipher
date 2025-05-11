
pub fn create_polybius_grid() -> [[char; 7]; 7] {
    let symbols: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.,!?-_()".chars().collect();
    let mut grid = [[' '; 7]; 7];
    for i in 0..7 {
        for j in 0..7 {
            grid[i][j] = *symbols.get(i * 7 + j).unwrap_or(&' ');
        }
    }
    grid
}
pub fn encrypt(text: &str) -> Vec<(u8, u8)> {
    let grid = create_polybius_grid();
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

pub fn decrypt(tiles: &[(u8, u8)]) -> String {
    let grid = create_polybius_grid();
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
