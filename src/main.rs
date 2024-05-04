use std::{path::PathBuf, process::Command};
use clap::Parser;
use cli::GiboCommand::{Dump, CurrentList, List, Root, Search, Update, Version};

mod cli;
mod list;
mod terminal;

fn call_gibo_command(command: String, args: Vec<String>) {
    let _ = Command::new("gibo")
        .arg(command)
        .args(args)
        .spawn();
}

fn print_prologue(dir: &PathBuf) {
    for item in list::find_prologue(dir) {
        println!("{}", item);
    }
}

pub fn dedup_ignorecase(args: &mut Vec<String>) {
    let mut seen = std::collections::HashSet::new();
    args.retain(|e| seen.insert(e.to_lowercase()));
}

fn remove_items_from_list_ignorecase(list: &mut Vec<String>, removal: &Vec<String>) {
    let r = removal.clone().iter().map(|e| e.to_lowercase()).collect::<Vec<String>>();
    list.retain(|e| !r.contains(&e.to_lowercase()));
}

struct DumpArgs {
    ordinal: Vec<String>,
    appendage: Vec<String>,
    removal: Vec<String>,
    dir: PathBuf,
}

impl DumpArgs {
    fn new_with_dir(args: Vec<String>, dir: PathBuf) -> Self {
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
        Self { ordinal, appendage, removal, dir }
    }
    fn new(args: Vec<String>) -> Self {
        Self::new_with_dir(args, PathBuf::from("."))
    }

    fn resultant_args(&self, remove_duplication: bool) -> Vec<String> {
        let mut new_args = vec![];
        if !self.appendage.is_empty() { // append mode
            let current = list::current_list(&self.dir);
            new_args.extend(current);
            new_args.extend(self.ordinal.clone());
            new_args.extend(self.appendage.clone());
        }
        if !self.removal.is_empty() { // remove mode
            remove_items_from_list_ignorecase(&mut new_args, &self.removal);
        }
        if remove_duplication {
            dedup_ignorecase(&mut new_args);
        }
        new_args
    }
}

fn main() {
    let app = cli::CliOpts::parse();
    let dir = PathBuf::from(".");
    match app.command {
        Dump { keep_prologue, remove_duplication, args } => {
            let dump_args = DumpArgs::new(args);
            if keep_prologue {
                print_prologue(&dir);
            }
            let new_args = dump_args.resultant_args(remove_duplication);
            call_gibo_command("dump".to_string(), new_args);
        }
        CurrentList => {
            terminal::print_in_column(list::current_list(&dir));
        }
        List | Root | Search | Update | Version => {
            call_gibo_command(format!("{}", app.command), vec![]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dump_args() {
        let args1 = DumpArgs::new_with_dir(vec!["+emacs".to_string()], PathBuf::from("testdata"));
        assert_eq!(args1.resultant_args(false), vec!["macOS", "Linux", "Windows", "emacs"]);

        let args2 = DumpArgs::new_with_dir(vec!["+emacs".to_string(), "macos".to_string(), "_windows".to_string()], PathBuf::from("testdata"));
        assert_eq!(args2.resultant_args(true), vec!["macOS", "Linux", "emacs"]);
    }
}