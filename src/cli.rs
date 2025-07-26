use clap::{Args, Parser, Subcommand, arg, builder::styling};

const STYLES: styling::Styles = styling::Styles::styled()
    .header(styling::AnsiColor::Green.on_default().bold())
    .usage(styling::AnsiColor::Green.on_default().bold())
    .literal(styling::AnsiColor::Blue.on_default().bold())
    .placeholder(styling::AnsiColor::Cyan.on_default());

#[derive(Parser)]
#[command(name = "boilertex")]
#[command(
    about = "Generate TeX project boilerplate",
    long_about = "A utility to declaratively create boilerplate from templates",
    styles = STYLES,
    version,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Generate boilerplate from a template
    #[clap(aliases = &["g", "gen"])]
    Generate(GenerateArgs),
    /// List the templates in your config
    #[clap(alias = "l")]
    List, // No arguments here are needed
    /// Preview the output of some template
    #[clap(alias = "p")]
    Preview(PreviewArgs),
}

#[derive(Args, Debug)]
pub struct GenerateArgs {
    /// The name of the template to be produced
    #[arg(value_name = "TEMPLATE")]
    pub template: String,

    /// Also initialize a Git repository with a default gitignore
    #[arg(long, short, value_name = "GIT")]
    pub git: bool,
}

#[derive(Args, Debug)]
pub struct PreviewArgs {
    /// The name of the template to be previewed
    #[arg(value_name = "TEMPLATE")]
    pub template: String,
}
