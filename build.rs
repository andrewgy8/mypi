#[macro_use]
extern crate clap;

use clap::Shell;

include!("src/cli.rs");

fn main() {
    // this might fail in CI
    let mut app = build_cli();
    app.gen_completions("mypi", Shell::Bash, "completions/");
    app.gen_completions("mypi", Shell::Fish, "completions/");
    app.gen_completions("mypi", Shell::Zsh, "completions/");
    app.gen_completions("mypi", Shell::PowerShell, "completions/");
}
