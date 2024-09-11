use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Output};
use clap::{Parser, CommandFactory};
use cli::GiboCommand::{CurrentList, Dump, List, Root, Search, Update, Version};

mod cli;
mod dump;
mod list;
mod terminal;
mod verboser;

fn call_gibo_command(command: String, args: Vec<String>, v: &Box<dyn verboser::Verboser>) -> Result<Output, std::io::Error>  {
    v.eprint(format!("gibo {} {:?}", command, args));
    Command::new("gibo")
        .arg(command)
        .args(args)
        .output()
}

fn write_to_stdout(output: Output) -> Result<(), std::io::Error> {
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    Ok(())
}

fn print_prologue(dest: &mut Box<dyn Write>, prologue: Vec<String>) {
    for line in prologue {
        writeln!(dest, "{}", line).unwrap();
    }
}

fn write_output(dest: &mut Box<dyn Write>, output: Output) -> Result<(), io::Error>{
    dest.write_all(&output.stdout)
}

fn perform_dump(keep_prologue: bool, remove_duplication: bool, in_place: bool, verbose: bool, args: Vec<String>) -> Result<(), io::Error> {
    let verboser = verboser::create(verbose);
    let dump_args = dump::DumpArgs::new_with_cwd(args);
    let new_args = dump_args.resultant_args(remove_duplication);
    let dest = dump_args.dest(in_place);
    let mut dest = match dest {
        Ok(d) => d,
        Err(e) => return Err(e)
    };
    let o = call_gibo_command("dump".to_string(), new_args, &verboser);
    let o = match o {
        Err(e) => return Err(e),
        Ok(output) => output,
    };
    if keep_prologue {
        print_prologue(&mut dest, dump_args.prologue());
    }
    write_output(&mut dest, o)
}

fn wrapped_version(_app: cli::CliOpts, v: &Box<dyn verboser::Verboser>) -> Result<Output, io::Error> {
    let binding = cli::CliOpts::command();
    let version = binding.get_version();
    println!("gibo-wrapper version {}", version.unwrap());
    print!("gibo         version ");
    call_gibo_command("version".into(), vec![], v)
}

fn open_impl(open_flag: bool, v: &Box<dyn verboser::Verboser>) -> Result<(), io::Error> {
    let o = call_gibo_command("root".into(), vec![], v);
    if open_flag {
        let output = o.unwrap();
        let path = String::from_utf8(output.stdout).unwrap();
        match opener::open(path.trim()) {
            Ok(_) => Ok(()),
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
        }
    } else {
        match o {
            Ok(output) => write_to_stdout(output),
            Err(e) => Err(e),
        }
    }
}

fn main() {
    let app = cli::CliOpts::parse();
    let dir = PathBuf::from(".");
    let verboser = verboser::create(app.verbose);
    let result = match app.command {
        Dump {
            keep_prologue,
            remove_duplication,
            in_place,
            verbose,
            args,
        } => perform_dump(keep_prologue, remove_duplication, in_place, verbose, args),
        CurrentList => {
            terminal::print_in_column(list::current_list(&dir));
            Ok(())
        },
        Version => match wrapped_version(app, &verboser) {
            Ok(output) => write_to_stdout(output),
            Err(e) => Err(e),
        },
        Root{ open } => open_impl(open, &verboser),
        List | Search | Update => {
            match call_gibo_command(format!("{}", app.command), vec![], &verboser) {
                Ok(output) => write_to_stdout(output),
                Err(e) => Err(e),
            }
        }
    };
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

