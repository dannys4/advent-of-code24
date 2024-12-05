fn create_rows(array: &String) -> (usize,Vec<Vec<char>>) {
    let rows: Vec<Vec<char>> =  array.lines().map(|a| a.chars().collect()).collect();
    return (rows[0].len(), rows);
}

fn create_columns(row_len: usize, rows: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut columns: Vec<Vec<char>> = Vec::with_capacity(row_len);
    for i in 0..row_len {
        columns.push(Vec::with_capacity(rows.len()));
        for j in 0..rows.len() {
            columns[i].push(rows[j][i]);
        }
    }
    return columns;
}

// A right diagonal is, given an array A[i,j], the diagonal is A[i+1,j+1].
// The rows are all of same length, but A may not be "square" (i.e. rows.len() != row_len)
fn create_right_diagonals(row_len: usize, rows: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut diagonals: Vec<Vec<char>> = Vec::new();
    // First construct the diagonals that start at the top row
    for i in 0..row_len {
        let mut diagonal: Vec<char> = Vec::new();
        let mut x = 0;
        let mut y = i;
        while x < rows.len() && y < row_len {
            diagonal.push(rows[x][y]);
            x += 1;
            y += 1;
        }
        diagonals.push(diagonal);
    }
    // Now construct the diagonals that start at the left column
    for i in 1..rows.len() {
        let mut diagonal: Vec<char> = Vec::new();
        let mut x = i;
        let mut y = 0;
        while x < rows.len() && y < row_len {
            diagonal.push(rows[x][y]);
            x += 1;
            y += 1;
        }
        diagonals.push(diagonal);
    }
    // println!("{:?}", diagonals);
    return diagonals;
}

// A left diagonal is, given an array A[i,j], the diagonal is A[i+1,j-1].
// The rows are all of same length, but A may not be "square" (i.e. rows.len() != row_len)
fn create_left_diagonals(row_len: usize, rows: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    // Invert all the rows (iterate bottom to top and right to left)
    let mut inverted_rows: Vec<Vec<char>> = Vec::with_capacity(rows.len());
    for i in 0..rows.len() {
        let mut row = Vec::with_capacity(row_len);
        for j in (0..row_len).rev() {
            row.push(rows[i][j]);
        }
        inverted_rows.push(row);
    }
    return create_right_diagonals(row_len, &inverted_rows);
}

// Given a "rectangular" string of letters,
// Create list of all the rows, all the columns, and all the diagonals
fn create_lists(array: &String) -> Vec<String> {
    let (row_len, rows) = create_rows(array);
    let columns = create_columns(row_len, &rows);
    let right_diagonals = create_right_diagonals(row_len, &rows);
    let left_diagonals = create_left_diagonals(row_len, &rows);
    let v2s = |v: Vec<char>| -> String { v.into_iter().collect() };
    let row_strs = rows.into_iter().map(v2s).collect::<Vec<String>>();
    let col_strs = columns.into_iter().map(v2s).collect::<Vec<String>>();
    let right_diag_strs = right_diagonals.into_iter().map(v2s).collect::<Vec<String>>();
    let left_diag_strs = left_diagonals.into_iter().map(v2s).collect::<Vec<String>>();
    return vec![row_strs, col_strs, right_diag_strs, left_diag_strs].concat();
}

fn count_word(list: &String, word: &String) -> i32 {
    let mut count = 0;
    if list.len() < word.len() {
        return count;
    }
    for i in 0..list.len()-word.len()+1 {
        if list[i..i+word.len()] == *word {
            count += 1;
        }
    }
    return count;
}

fn part1(word_search: &String, word: &String) -> i32 {
    let all_lists = create_lists(word_search);
    let mut count = 0;
    let reverse_word: String = word.chars().rev().collect();
    for list in all_lists {
        let mut curr_count = count_word(&list, word);
        curr_count += count_word(&list, &reverse_word);
        // println!("{}: {}", list, curr_count);
        count += curr_count
    }
    return count;
}

// Class that represents the X in the 3x3 tile
struct XTile {
    x: (char, char),
    y: (char, char),
    z: char
}

fn print_rows(row0: &[char], row1: &[char], row2: &[char]) {
    println!("{:?}", row0);
    println!("{:?}", row1);
    println!("{:?}", row2);
    println!();
}

fn print_xtile(tile: &XTile) {
    println!("{}.{}", tile.x.0, tile.y.0);
    println!(".{}.", tile.z);
    println!("{}.{}", tile.y.1, tile.x.1);
    println!();
}

impl XTile {
    fn new(row0: &[char], row1: &[char], row2: &[char]) -> XTile {
        let x = (row0[0], row2[2]);
        let y = (row0[2], row2[0]);
        let z = row1[1];
        XTile { x, y, z }
    }
}

fn is_xmas(tile: XTile) -> bool {
    let XTile { x, y, z } = tile;
    if z != 'A' {
        // println!("Invalid z: {}", z);
        return false;
    }
    if (x.0 != 'M' || x.1 != 'S') && (x.0 != 'S' || x.1 != 'M') {
        // println!("Invalid x diag: {}{}{}", x.0, z, x.1);
        return false;
    }
    if (y.0 != 'M' || y.1 != 'S') && (y.0 != 'S' || y.1 != 'M') {
        // println!("Invalid y diag: {}{}{}", y.0, z, y.1);
        return false;
    }
    return true;
}

fn create_3x3(array: &String) -> Vec<XTile> {
    let (row_len, rows) = create_rows(array);
    let mut three_by_three: Vec<XTile> = Vec::new();
    for i in 0..rows.len()-2 {
        for j in 0..row_len-2 {
            let row0 = &rows[i][j..j+3];
            let row1 = &rows[i+1][j..j+3];
            let row2 = &rows[i+2][j..j+3];
            // print_rows(row0, row1, row2);
            let tile = XTile::new(row0, row1, row2);
            // print_xtile(&tile);
            three_by_three.push(tile);
        }
    }
    return three_by_three;
}

fn part2(word_search: &String) -> i32 {
    let three_by_three = create_3x3(word_search);
    let mut count = 0;
    for tile in three_by_three {
        if is_xmas(tile) {
            count += 1;
        }
    }
    return count;
}

pub fn fcn(word_search: &String) {
    let word = "XMAS".to_string();
    let p1 = part1(word_search, &word);
    println!("Part 1: {}", p1);
    let p2 = part2(word_search);
    println!("Part 2: {}", p2);
}