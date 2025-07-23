use crate::config::read_config_to_string;
use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Config {
    #[serde(rename = "general")]
    pub general_options: General,

    #[serde(rename = "templates")]
    pub templates: HashMap<String, TeXTemplate>,
}

impl Config {
    pub fn new() -> Config {
        if let Ok(document) = toml::from_str(read_config_to_string().as_str()) {
            document
        } else {
            Self::default()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            general_options: General {
                draft: true,
                main_file_name: "main.tex".to_string(),
                papersize: "letterpaper".to_string(),
                textsize: "12pt".to_string(),
            },
            templates: HashMap::from([(
                "example".to_string(),
                TeXTemplate {
                    bib_file: Some("bibliography.bib".to_string()),
                    documentclass: Documentclass {
                        class: "article".to_string(),
                        class_opts: None,
                    },
                    packages: Some(vec![
                        TeXPackage {
                            pkg_name: "fontenc".to_string(),
                            pkg_opts: Some(vec!["T1".to_string()]),
                        },
                        TeXPackage {
                            pkg_name: "hyperref".to_string(),
                            pkg_opts: Some(vec![
                                "hidelinks".to_string(),
                                "hypertexnames=false".to_string(),
                            ]),
                        },
                        TeXPackage {
                            pkg_name: "graphicx".to_string(),
                            pkg_opts: None,
                        },
                    ]),
                },
            )]),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TeXTemplate {
    pub bib_file: Option<String>,
    pub documentclass: Documentclass,
    pub packages: Option<Vec<TeXPackage>>,
}

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TeXPackage {
    pub pkg_name: String,
    pub pkg_opts: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Documentclass {
    pub class: String,
    pub class_opts: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct General {
    pub draft: bool,
    pub main_file_name: String,
    pub papersize: String,
    pub textsize: String,
}
