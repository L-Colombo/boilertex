#![allow(dead_code)]
#![allow(unused_variables)]

use boilertex::{
    cli::{Cli, Commands},
    cmds::{genrate_template, list_templates, preview_template},
    tex::Config,
};
use clap::Parser;

fn main() {
    let cfg = Config::new();
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate(args) => genrate_template(cfg, args),
        Commands::List => list_templates(cfg),
        Commands::Preview(args) => preview_template(cfg, args.template),
    }
}
