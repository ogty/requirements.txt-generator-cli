use std::env;

mod modules;
use crate::modules::generator::{RequirementstxtGenerator, RequirementstxtGeneratorComponents};

fn print_usage() {
    eprintln!(
        "
        
        reqgene - requirements.txt Generator

    Description:
        reqgene is a tool to generate requirements.txt from a Python project.
        It is a simple command line tool that uses regular expressions to
        extract the modules used in a Python project.

    Usage:
        reqgene [options] <language> <directory path>

    Options:
        -h, --help:
            Show this help message and exit.
        -v, --version:
            Include the version number in the output.
        -j, --jupyter:
            Generate requirements.txt for Jupyter.
        -s, --show:
            Show the generated requirements.txt.

    "
    )
}

fn main() {
    let mut generator: RequirementstxtGenerator = RequirementstxtGenerator::default();
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        panic!("No arguments provided");
    }

    // display usage
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        print_usage();
        return;
    }

    // version
    if args.contains(&"-v".to_string()) || args.contains(&"--version".to_string()) {
        generator.is_version = true;
        args.retain(|x: &String| x != "-v" && x != "--version");
    }

    // jupyter
    if args.contains(&"-j".to_string()) || args.contains(&"--jupyter".to_string()) {
        generator.is_jupyter = true;
        args.retain(|x: &String| x != "-j" && x != "--jupyter");
    }

    // show
    if args.contains(&"-s".to_string()) || args.contains(&"--show".to_string()) {
        generator.is_show = true;
        args.retain(|x: &String| x != "-s" && x != "--show");
    }

    generator.language = args[0].to_string();
    generator.directory_path = args[1].to_string();

    generator.generate();
}
