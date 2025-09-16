use crate::{cli::GenerateArgs, tex::Config};
use bat::PrettyPrinter;
use colored::Colorize;
use std::{
    fs::OpenOptions,
    io::Write,
    process::{Command, exit},
};

pub fn genrate_template(cfg: Config, args: GenerateArgs) {
    // if it fails, let the error bubble up
    let main_file_content: String = output_template(&cfg, &args.template).unwrap();
    let mut main_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(cfg.general_options.main_file_name)
        .expect("Error createing your main file");

    match main_file.write_all(main_file_content.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{e}");
            exit(1)
        }
    }

    // write a gitignore and initialize a git repo
    if args.git {
        let _ = Command::new("git")
            .arg("init")
            .output()
            .expect("Unexpected problems occurred while initializing a git repository");

        let mut gitignore = OpenOptions::new()
            .create(true)
            .write(true)
            .open(".gitignore")
            .expect("Error creating the `.gitignore");

        let _ = gitignore.write_all(GITIGNORE.as_bytes());
    }
}

pub fn list_templates(cfg: Config) {
    println!("{}", "All the templates defined in your config:".green());
    for i in cfg.templates.keys() {
        println!("\t- {i}")
    }
}

pub fn preview_template(cfg: Config, template: String) {
    // if it fails, let the error bubble up
    let out: String = output_template(&cfg, &template).unwrap();
    println!("Showing template `{}`:\n", &template.green());
    let _ = PrettyPrinter::new()
        .input_from_bytes(out.as_bytes())
        .language("tex")
        .print();
}

fn output_template(cfg: &Config, template_name: &String) -> std::io::Result<String> {
    if let Some(template) = cfg.templates.get(template_name) {
        let complete_class_opts =
            if let Some(template_class_opts) = &template.documentclass.class_opts {
                vec![
                    vec![
                        cfg.general_options.papersize.to_owned(),
                        cfg.general_options.textsize.to_owned(),
                    ],
                    template_class_opts.to_vec(),
                ]
                .concat()
                .join(", ")
            } else {
                vec![
                    cfg.general_options.papersize.to_owned(),
                    cfg.general_options.textsize.to_owned(),
                ]
                .join(", ")
            };
        let documentclass_slot = format!(
            "\\documentclass[{}]{{{}}}",
            complete_class_opts, template.documentclass.class
        );
        let packages_slot = if let Some(pkgs) = &template.packages {
            let mut tmp = Vec::new();
            for i in pkgs {
                tmp.push(i.to_string());
            }
            tmp.join("\n")
        } else {
            String::new()
        };
        let bibfile_slot = if let Some(bibfile) = &template.bib_file {
            format!("\\addbibresource{{{}}}", bibfile)
        } else {
            String::new()
        };

        Ok(format!(
            "{documentclass_slot}\n\n{packages_slot}\n\n{bibfile_slot}\n\n\\begin{{document}}\nHello, \\LaTeX\n\\end{{document}}"
        ))
    } else {
        eprintln!(
            "{}: No template named {} was found!",
            "[Error]".red(),
            template_name.green()
        );
        eprintln!(
            "{}: Try to run `{}` to see a list of the templates in your configuration",
            "[Tip]".blue(),
            "boilertex list".green()
        );
        exit(1)
    }
}

const GITIGNORE: &'static str = r#"## Core latex/pdflatex auxiliary files:
*.aux
*.lof
*.log
*.lot
*.fls
*.out
*.toc
*.fmt
*.fot
*.cb
*.cb2
.*.lb

## Intermediate documents:
*.dvi
*.xdv
*-converted-to.*
# these rules might exclude image files for figures etc.
# *.ps
# *.eps
*.pdf

## Generated if empty string is given at
.pdf

## Bibliography auxiliary files (bibtex/biblatex/biber):
*.bbl
*.bbl-SAVE-ERROR
*.bcf
*.bcf-SAVE-ERROR
*.blg
*-blx.aux
*-blx.bib
*.run.xml

## Build tool auxiliary files:
*.fdb_latexmk
*.synctex
*.synctex(busy)
*.synctex.gz
*.synctex.gz(busy)
*.pdfsync
*.rubbercache
rubber.cache

## Build tool directories for auxiliary files
# latexrun
latex.out/

## Auxiliary and intermediate files from other packages:
# algorithms
*.alg
*.loa

# achemso
acs-*.bib

# amsthm
*.thm

# beamer
*.nav
*.pre
*.snm
*.vrb

# changes
*.soc

# comment
*.cut

# cprotect
*.cpt

# elsarticle (documentclass of Elsevier journals)
*.spl

# endnotes
*.ent

# fixme
*.lox

# feynmf/feynmp
*.mf
*.mp
*.t[1-9]
*.t[1-9][0-9]
*.tfm

#(r)(e)ledmac/(r)(e)ledpar
*.end
*.?end
*.[1-9]
*.[1-9][0-9]
*.[1-9][0-9][0-9]
*.[1-9]R
*.[1-9][0-9]R
*.[1-9][0-9][0-9]R
*.eledsec[1-9]
*.eledsec[1-9]R
*.eledsec[1-9][0-9]
*.eledsec[1-9][0-9]R
*.eledsec[1-9][0-9][0-9]
*.eledsec[1-9][0-9][0-9]R

# glossaries
*.acn
*.acr
*.glg
*.glo
*.gls
*.glsdefs
*.lzo
*.lzs
*.slg
*.slo
*.sls

# uncomment this for glossaries-extra (will ignore makeindex's style files!)
# *.ist

# gnuplot
*.gnuplot
*.table

# gnuplottex
*-gnuplottex-*

# gregoriotex
*.gaux
*.glog
*.gtex

# htlatex
*.4ct
*.4tc
*.idv
*.lg
*.trc
*.xref

# hypdoc
*.hd

# hyperref
*.brf

# knitr
*-concordance.tex
# TODO Uncomment the next line if you use knitr and want to ignore its generated tikz files
# *.tikz
*-tikzDictionary

# listings
*.lol

# luatexja-ruby
*.ltjruby

# makeidx
*.idx
*.ilg
*.ind

# minitoc
*.maf
*.mlf
*.mlt
*.mtc[0-9]*
*.slf[0-9]*
*.slt[0-9]*
*.stc[0-9]*

# minted
_minted*
*.pyg

# morewrites
*.mw

# newpax
*.newpax

# nomencl
*.nlg
*.nlo
*.nls

# pax
*.pax

# pdfpcnotes
*.pdfpc

# sagetex
*.sagetex.sage
*.sagetex.py
*.sagetex.scmd

# scrwfile
*.wrt

# svg
svg-inkscape/

# sympy
*.sout
*.sympy
sympy-plots-for-*.tex/

# pdfcomment
*.upa
*.upb

# pythontex
*.pytxcode
pythontex-files-*/

# tcolorbox
*.listing

# thmtools
*.loe

# TikZ & PGF
*.dpth
*.md5
*.auxlock

# titletoc
*.ptc

# todonotes
*.tdo

# vhistory
*.hst
*.ver

# easy-todo
*.lod

# xcolor
*.xcp

# xmpincl
*.xmpi

# xindy
*.xdy

# xypic precompiled matrices and outlines
*.xyc
*.xyd

# endfloat
*.ttt
*.fff

# Latexian
TSWLatexianTemp*

## Editors:
# WinEdt
*.bak
*.sav

# Texpad
.texpadtmp

# LyX
*.lyx~

# Kile
*.backup

# gummi
.*.swp

# KBibTeX
*~[0-9]*

# TeXnicCenter
*.tps

# auto folder when using emacs and auctex
./auto/*
*.el

# expex forward references with \gathertags
*-tags.tex

# standalone packages
*.sta

# Makeindex log files
*.lpz

# xwatermark package
*.xwm

# REVTeX puts footnotes in the bibliography by default, unless the nofootinbib
# option is specified. Footnotes are the stored in a file with suffix Notes.bib.
# Uncomment the next line to have this generated file ignored.
#*Notes.bib
"#;
