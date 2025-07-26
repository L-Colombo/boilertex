use boilertex::{
    cli::{Cli, Commands},
    cmds::{genrate_template, list_templates, preview_template},
    tex::Config,
};
use clap::Parser;
use clap_allgen::*;
use std::process::exit;

fn main() {
    // uncomment and run once at every version update
    // _generate_manpages_and_completions();

    let cfg = Config::new();
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate(args) => genrate_template(cfg, args),
        Commands::List => list_templates(cfg),
        Commands::Preview(args) => preview_template(cfg, args.template),
    }
}

fn _generate_manpages_and_completions() {
    let current_dir = match std::env::current_dir() {
        Ok(path) => path,
        Err(_) => {
            println!("Could not get current working directory");
            exit(1)
        }
    };

    render_manpages::<Cli>(current_dir.join("etc/man/")).unwrap();
    render_shell_completions::<Cli>(current_dir.join("etc/shell_comp")).unwrap();
}
