use clap::{Command, CommandFactory};
use clap_complete::Shell;
use std::fs::File;
use std::path::Path;

include!("src/cli.rs");

fn generate(s: Shell, app: &mut Command, appname: &str, outdir: &Path, file: String) {
    let destfile = outdir.join(file);
    std::fs::create_dir_all(destfile.parent().unwrap()).unwrap();
    let mut dest = File::create(destfile).unwrap();
    
    clap_complete::generate(s, app, appname, &mut dest);
}

fn main() {
    let appname = "gibo";

    let mut app = CliOpts::command();
    app.set_bin_name(appname);

    let outdir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("target/completions/");

    generate(Shell::Bash, &mut app, appname, &outdir, format!("bash/{}", appname));
    generate(Shell::Elvish, &mut app, appname, &outdir, format!("elvish/{}", appname));
    generate(Shell::Fish, &mut app, appname, &outdir, format!("fish/{}", appname));
    generate(Shell::PowerShell, &mut app, appname, &outdir, format!("powershell/{}", appname));
    generate(Shell::Zsh, &mut app, appname, &outdir, format!("zsh/_{}", appname));
}
