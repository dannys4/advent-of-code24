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

fn create_3x3(array: &String) -> Vec<String> {
    let (row_len, rows) = create_rows(array);
    let mut three_by_three: Vec<String> = Vec::new();
    for i in 0..rows.len()-2 {
        for j in 0..row_len-2 {
            let mut s = String::new();
            for k in 0..3 {
                for l in 0..3 {
                    s.push(rows[i+k][j+l]);
                }
            }
            three_by_three.push(s);
        }
    }
    return three_by_three;
}

pub fn fcn(word_search: &String) {
    let word = "XMAS".to_string();
    let result = part1(word_search, &word);
    println!("Part 1: {}", result);
}