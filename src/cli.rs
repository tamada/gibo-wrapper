use clap::{Parser, Subcommand};
use std::fmt::{Display, Formatter, Result};

#[derive(Parser, Debug, PartialEq)]
#[command(
    version,
    author,
    about,
    arg_required_else_help = true,
    disable_version_flag = true,
    long_about = "gibo-wrapper acts like gibo and improves gibo by adding the following features.
    1. introduce current-list subcommand for dumping the boilerplates while keeping the prologue of .gitignore file.
    2. improve dump subcommand
       * append mode: appending the boilerplates into the .gitignore file.
       * remove mode: removing the boilerplates from the .gitignore file.
       * remove-duplication option removes the duplicated boilerplates names by dumping (-r option).
       * keep-prologue option keeps the prologue in the .gitignore (-k option).
    3. introduce the option for root subcommand
       * --open option of root subcommand opens the folder in the GUI file manager."
)]
pub struct CliOpts {
    #[clap(subcommand)]
    pub(crate) command: GiboCommand,

    #[clap(short, long, help = "Show verbose output")]
    pub(crate) verbose: bool,
}

#[derive(Debug, Subcommand, PartialEq)]
pub(crate) enum GiboCommand {
    #[command(about = "Dump a boilerplate")]
    Dump {
        #[clap(
            short,
            long,
            default_value_t = false,
            help = "Keep the prologue of the .gitignore"
        )]
        keep_prologue: bool,

        #[clap(
            short,
            long = "remove-duplication",
            default_value_t = false,
            help = "Remove the duplicated boilerplate names"
        )]
        remove_duplication: bool,

        #[clap(short, long = "in-place", help = "Update .gitignore files in-place")]
        in_place: bool,

        #[clap(short, long, help = "Show verbose output")]
        verbose: bool,

        #[clap(help = "the boilerplate names to dump.
Append boilerplates into the current .gitignore file if the name starts with `+`.
Remove boilerplates from the current .gitignore file if the name starts with `_`.")]
        args: Vec<String>,
    },
    #[command(about = "List available boilerplates")]
    List,
    #[command(about = "List the current boilerplates in the .gitignore file")]
    CurrentList,
    #[command(about = "Show the directory where gibo stores its boilerplates")]
    Root {
        #[clap(short = 'o', long = "open", help = "Open the folder in the GUI file manager")]
        open: bool,
    },
    #[command(about = "Search for boilerplates")]
    Search,
    #[command(about = "Update the gitignore boilerplate repository")]
    Update,
    #[command(about = "Show the current version number of gibo")]
    Version,
}

impl Display for GiboCommand {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            GiboCommand::Dump { .. } => write!(f, "dump"),
            GiboCommand::List => write!(f, "list"),
            GiboCommand::CurrentList => write!(f, "current-list"),
            GiboCommand::Root { .. } => write!(f, "root"),
            GiboCommand::Search => write!(f, "search"),
            GiboCommand::Update => write!(f, "update"),
            GiboCommand::Version => write!(f, "version"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gibo_command() {
        let cli1 = CliOpts::parse_from(&["gibo-wrapper", "dump", "+macos", "linux", "-k", "-r"]);
        assert_eq!(
            cli1.command,
            GiboCommand::Dump {
                keep_prologue: true,
                remove_duplication: true,
                in_place: false,
                verbose: false,
                args: vec!["+macos".to_string(), "linux".to_string()],
            }
        );
    }
}
