use std::env;

mod modules;
use crate::modules::extractor::{ModuleExtractor, Components, EachLanguageRegex};

fn main() {
    let _args: Vec<String> = env::args().skip(1).collect();


    let mut extractor: ModuleExtractor = ModuleExtractor {
        language_regex: EachLanguageRegex::Python,
    };

    let splited_source_code: Vec<String> = modules::file::read_lines("sample.py");
    let result = extractor.extract(splited_source_code);
    println!("{:?}", result);
}

/*

reqgene - requirements.txt Generator

Description:
    reqgene is a tool to generate requirements.txt from a Python project.
    It is a simple command line tool that uses regular expressions to
    extract the modules used in a Python project.

Usage:
    reqgene [options] <language> <directory path ...>

Options:
    -h, --help
        Show this help message and exit.
    -v, --version
        Include the version number in the output.
    -j, --jupyter
        Generate requirements.txt for Jupyter.
    -s, --show
        Show the generated requirements.txt.

*/