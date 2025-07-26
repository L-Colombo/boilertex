use boilertex::tex::{Config, Documentclass, General, TeXPackage, TeXTemplate};
use pretty_assertions::assert_eq;
use std::collections::HashMap;

const _EXAMPLE_TOML_DOCUMENT: &'static str = r#"
[general]
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

#[test]
fn generate_template() {
    let serialized_document: Config = Config::new();
    let default_document: Config = Config::default();

    let test_document: Config = Config {
        general_options: General {
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
    };

    assert_eq!(test_document, default_document);
    assert_eq!(test_document, serialized_document)
}
