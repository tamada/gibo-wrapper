use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use crate::list;

pub struct DumpArgs {
    ordinal: Vec<String>,
    appendage: Vec<String>,
    removal: Vec<String>,
    dir: PathBuf,
}

impl DumpArgs {
    /// new with current working directory.
    /// <code>Self::new(args, PathBuf::from("."))</code>.
    pub fn new_with_cwd(args: Vec<String>) -> Self {
        Self::new(args, PathBuf::from("."))
    }

    pub fn new(args: Vec<String>, dir: PathBuf) -> Self {
        let mut ordinal = vec![];
        let mut appendage = vec![];
        let mut removal = vec![];
        for arg in args {
            if arg.starts_with("+") {
                appendage.push(arg.trim_start_matches("+").to_string());
            } else if arg.starts_with("_") {
                removal.push(arg.trim_start_matches("_").to_string());
            } else {
                ordinal.push(arg);
            }
        }
        Self {
            ordinal,
            appendage,
            removal,
            dir,
        }
    }

    pub fn dest(&self, inplace: bool) -> Result<Box<dyn Write>, std::io::Error> {
        if !inplace {
            Ok(Box::new(BufWriter::new(std::io::stdout())))
        } else {
            let path = self.dir.join(".gitignore");
            match File::create(path) {
                Ok(file) => Ok(Box::new(BufWriter::new(file))),
                Err(e) => return Err(e),
            }
        }
    }

    pub fn prologue(&self) -> Vec<String> {
        list::find_prologue(&self.dir)
    }

    pub fn resultant_args(&self, remove_duplication: bool) -> Vec<String> {
        let mut new_args = vec![];
        new_args.extend(self.ordinal.clone());
        if !self.appendage.is_empty() {
            // append mode
            let current = list::current_list(&self.dir);
            new_args.extend(current);
            new_args.extend(self.appendage.clone());
        }
        if !self.removal.is_empty() {
            // remove mode
            remove_items_from_list_ignorecase(&mut new_args, &self.removal);
        }
        if remove_duplication {
            dedup_ignorecase(&mut new_args);
        }
        new_args
    }
}

fn dedup_ignorecase(args: &mut Vec<String>) {
    let mut seen = std::collections::HashSet::new();
    args.retain(|e| seen.insert(e.to_lowercase()));
}

fn remove_items_from_list_ignorecase(list: &mut Vec<String>, removal: &Vec<String>) {
    let r = removal
        .clone()
        .iter()
        .map(|e| e.to_lowercase())
        .collect::<Vec<String>>();
    list.retain(|e| !r.contains(&e.to_lowercase()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dump_args() {
        let args1 = DumpArgs::new(vec!["+emacs".to_string()], PathBuf::from("testdata"));
        assert_eq!(
            args1.resultant_args(false),
            vec!["macOS", "Linux", "Windows", "emacs"]
        );

        let args2 = DumpArgs::new(
            vec![
                "+emacs".to_string(),
                "macos".to_string(),
                "_windows".to_string(),
            ],
            PathBuf::from("testdata"),
        );
        assert_eq!(args2.resultant_args(true), vec!["macOS", "Linux", "emacs"]);
    }
}
