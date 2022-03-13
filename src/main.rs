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
        print_usage();
        return;
    }

    // display usage
    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        print_usage();
        return;
    }

    // version
    if args.contains(&String::from("-v")) || args.contains(&String::from("--version")) {
        generator.is_version = true;
        args.retain(|x: &String| x != "-v" && x != "--version");
    }

    // jupyter
    if args.contains(&String::from("-j")) || args.contains(&String::from("--jupyter")) {
        generator.is_jupyter = true;
        args.retain(|x: &String| x != "-j" && x != "--jupyter");
    }

    // show
    if args.contains(&String::from("-s")) || args.contains(&String::from("--show")) {
        generator.is_show = true;
        args.retain(|x: &String| x != "-s" && x != "--show");
    }

    generator.language = args[0].to_string();
    generator.directory_path = args[1].to_string();

    generator.generate();
}
