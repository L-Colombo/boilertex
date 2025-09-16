use colored::Colorize;
use std::{
    env,
    fs::{OpenOptions, create_dir_all, exists, read_to_string},
    io::{Result, Write},
    path::PathBuf,
    process::exit,
};

const DEFAULT_CONFIG_FILE: &str = r#"[general]
main_file_name = "main.tex"
papersize = "letterpaper"
textsize = "12pt"

[templates]

[templates.example]
bib_file = "bibliography.bib"
documentclass = { class = "article" }
packages = [
    { pkg_name = "fontenc", pkg_opts = ["T1"] },
    { pkg_name = "hyperref", pkg_opts = ["hidelinks", "hypertexnames=false"] },
    { pkg_name = "graphicx" }
]
"#;

fn get_config_directory_path() -> PathBuf {
    let home_directory = match env::home_dir() {
        Some(home_dir) => home_dir,
        None => {
            eprintln!("Error: could not get your $HOME directory");
            exit(1)
        }
    };

    home_directory.join(".config/boilertex")
}

fn config_file_path() -> PathBuf {
    get_config_directory_path().join("boilertex.toml")
}

pub fn create_default_config_file() -> Result<()> {
    let config_path = get_config_directory_path();
    let config_file_path = config_file_path();

    let _ = create_dir_all(&config_path);

    let mut default_config_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(config_file_path)
        .expect("Something went wrong while creating a default config file...");

    default_config_file.write_all(DEFAULT_CONFIG_FILE.as_bytes())
}

pub fn read_config_to_string() -> String {
    let config_file_path = config_file_path();

    if !exists(&config_file_path).unwrap() {
        println!("{}: Configuration file not found", "[Error]".red());
        println!(
            "{}: Attempting to create a default config file",
            "[Info]".blue()
        );

        if let Ok(_) = create_default_config_file() {
            println!(
                "{}: Default config file successfully created at\n\t{}",
                "[Success]".green(),
                &config_file_path.display()
            );
        }
    }

    // It is safe to unwrap beacuse the situation has already been handled
    read_to_string(config_file_path).unwrap()
}
