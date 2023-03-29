pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for (row_index, row) in minefield.iter().enumerate() {
        let mut new_row = String::new();
        for (col_index, col) in row.as_bytes().iter().enumerate() {
            if *col as char == '*' {
                new_row.push('*');
            } else {
                let mut count = 0;
                for i in row_index.saturating_sub(1)..=row_index + 1 {
                    for j in col_index.saturating_sub(1)..=col_index + 1 {
                        if i < minefield.len() && j < row.len() {
                            if minefield[i].as_bytes()[j] as char == '*' {
                                count += 1;
                            }
                        }
                    }
                }
                if count > 0 {
                    new_row.push_str(&count.to_string());
                } else {
                    new_row.push(' ');
                }
            }
        }
        result.push(new_row);
    }
    result
    // unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n");
}
