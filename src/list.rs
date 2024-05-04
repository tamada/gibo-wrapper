use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// Find the prologue of .gitignore file on the current directory
/// and return them as a list (one line is one element).
/// If the current directory does not have the .gitignore file, return an empty list.
pub fn find_prologue(dir: &PathBuf) -> Vec<String> {
    let ignore_file = dir.join(PathBuf::from(".gitignore"));
    if !ignore_file.exists() {
        return vec![];
    }
    let file = File::open(ignore_file).unwrap();
    let reader = BufReader::new(file);
    let mut items = vec![];
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.starts_with("### ") {
                break;
            } else {
                items.push(line);
            }
        }
    }
    items

}

fn find_boilerplate_name(line: String) -> Option<String> {
    if !line.starts_with("### ") || !line.ends_with(".gitignore") {
        None
    } else {
        let items = line
            .trim_start_matches("### ")
            .rsplit("/")
            .collect::<Vec<&str>>();
        if items.len() < 1 {
            None
        } else {
            Some(items[0].trim_end_matches(".gitignore").to_string())
        }
    }
}

/// Extract the boilerplates in the .gitignore file on the current directory
/// and return them as a list.
/// If the current directory does not have the .gitignore file, return an empty list.
pub fn current_list(dir: &PathBuf) -> Vec<String> {
    let ignore_file = dir.join(PathBuf::from(".gitignore"));
    if !ignore_file.exists() {
        return vec![];
    }
    let file = File::open(ignore_file).unwrap();
    let reader = BufReader::new(file);
    let mut items = vec![];
    for line in reader.lines() {
        if let Ok(line) = line {
            match find_boilerplate_name(line) {
               Some(name) => items.push(name),
                None => {},
            }
        }
    }
    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_boilerplate_name() {
        assert_eq!(
            find_boilerplate_name("### Python.gitignore".to_string()),
            Some("Python".to_string())
        );
        assert_eq!(
            find_boilerplate_name("### Generated by gibo (https://github.com/simonwhitaker/gibo)".to_string()),
            None
        );
        assert_eq!(
            find_boilerplate_name("### https://raw.github.com/github/gitignore/4488915eec0b3a45b5c63ead28f286819c0917de/Global/Linux.gitignore".to_string()),
            Some("Linux".to_string())
        );
    }
}