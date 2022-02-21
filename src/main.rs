use std::env;

mod modules;
use crate::modules::generator::Generator;
use crate::modules::generator::Operator;


fn main() {
    let args: Vec<String> = env::args().collect();

    let language: String = args[2].to_string();

    let extension: String = 
        if language == String::from("python") { 
            String::from(".py") 
        } else if language == String::from("julia") {
            String::from(".jl")
        } else if language == String::from("ipynb") {
            String::from(".ipynb")
        } else {
            panic!("The requirements.txt Generator CLI does not support \"{}\"", language)
        };
    
    let mut generator = modules::generator::RequirementstxtGenerator{ ..Default::default() };

    generator.language = language;
    generator.extension = extension;
    generator.path = args[1].to_string();

    let _ = generator.get_files(args[1].to_string());

    for arg in args {
        if arg == String::from("-v") {
            generator.version = true;
        } else if arg == String::from("-d") {
            generator.extension = String::from("");
            generator.detail();
        }
    }
}
