fn get_max_length_and_number_of_columns(list: &Vec<String>, cols: usize) -> (usize, usize) {
    let max_length = list.iter().map(|item| item.len()).max().unwrap_or(0);
    let number_of_columns: usize = (cols + 1) / (max_length + 1);
    if number_of_columns == 0 {
        (max_length, 1)
    } else {
        (max_length, number_of_columns)
    }
}

fn padding_list(list: Vec<String>, max_length: usize) -> Vec<String> {
    let mut result = vec![];
    for item in list {
        let mut padded_item = item.clone();
        for _ in 0..max_length - item.len() {
            padded_item.push(' ');
        }
        result.push(padded_item);
    }
    result
}

fn print_in_column_string(list: Vec<String>, cols: usize) -> Vec<String> {
    let (max_length, number_of_columns) = get_max_length_and_number_of_columns(&list, cols);
    let targets = padding_list(list, max_length);
    let mut result = vec![];
    let mut line = Vec::<u8>::new();
    for (i, item) in targets.iter().enumerate() {
        line.extend(item.as_bytes());
        if i % number_of_columns == (number_of_columns - 1) || i == targets.len() - 1 {
            let r = String::from_utf8(line.clone()).unwrap();
            result.push(r.trim().to_string());
            line.clear();
        } else {
            line.push(b' ');
        }
    }
    result
}

/// Print a list of strings in columns
/// The routine of this function is ported from https://github.com/simonwhitaker/gibo/blob/main/utils/fmt.go
pub fn print_in_column(list: Vec<String>) {
    let (cols, _rows) = termion::terminal_size().unwrap();
    for item in print_in_column_string(list, cols as usize) {
        println!("{}", item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_print_in_column_string() {
        let v1 = vec!["macOS", "Linux", "Windows", "Go", "VisualStudioCode", "JetBrains"];
        let r1 = print_in_column_string(v1.iter().map(|s| s.to_string()).collect(), 125);
        assert_eq!(r1.len(), 1);
        assert_eq!(r1[0], "macOS            Linux            Windows          Go               VisualStudioCode JetBrains");

        let v2 = vec!["macOS", "Linux", "Windows", "Go", "VisualStudioCode", "JetBrains", "Rust", "NetBeans"];
        let r2 = print_in_column_string(v2.iter().map(|s| s.to_string()).collect(), 125);
        assert_eq!(r2.len(), 2);
        assert_eq!(r2[0], "macOS            Linux            Windows          Go               VisualStudioCode JetBrains        Rust");
        assert_eq!(r2[1], "NetBeans");
    }
}