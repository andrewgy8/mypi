use clap::{App, Arg, SubCommand};


pub fn build_cli() -> App<'static, 'static> {
    App::new("mypi")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommands(vec![])
}
