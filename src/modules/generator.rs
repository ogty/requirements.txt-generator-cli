use core::panic;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, ReadDir};
use std::process::{Command, Output};

use crate::modules::extractor::ModuleExtractor;
use crate::modules::extractor::Extractor;
use crate::modules::file::read;
use crate::modules::file::write;


pub struct RequirementstxtGenerator {
    pub path: String,
    pub language: String,
    pub extension: String,
    pub version: bool,
    pub file_paths: Vec<String>,
    pub modules: Vec<String>,
    pub installed_modules: Vec<String>,
}


pub trait Generator {
    fn get_modules_version(&mut self) {}
    fn command_runner(&mut self) -> String;
    fn detail(&mut self) {}
    fn generate(&mut self) {}
}


pub trait Operator {
    fn get_files(&mut self, path: String) -> Result<(), Box<dyn Error>>;
}


impl Default for RequirementstxtGenerator {
    fn default() -> RequirementstxtGenerator {
        RequirementstxtGenerator {
            path: String::new(),
            language: String::new(),
            extension: String::new(),
            version: false,
            file_paths: Vec::new(),
            modules: Vec::new(),
            installed_modules: Vec::new(),
        }
    }
}


impl Operator for RequirementstxtGenerator {
    fn get_files(&mut self, path: String) -> Result<(), Box<dyn Error>> {
        let dir: ReadDir = fs::read_dir(path)?;
        for item in dir.into_iter() {
            let tmp: String = format!("{}", item?.path().to_string_lossy());
            let tmp2: String = tmp.clone();
            if tmp.ends_with(&self.extension) { self.file_paths.push(tmp) }
            let _ = self.get_files(tmp2);
        }

        Ok(())
    }
}


impl Generator for RequirementstxtGenerator {
    fn get_modules_version(&mut self) {
        let command_output: String = self.command_runner();
        let splited_command_output: Vec<&str> = command_output.split("\r\n").collect();
        
        let mut installed_modules: Vec<String> = Vec::new();

        if self.language == String::from("python") {
            for command_output_line in splited_command_output {
                let module_name_and_version: Vec<&str> = command_output_line.split("==").collect();
                if module_name_and_version.len() == 2 {
                    installed_modules.push(command_output_line.to_string());
                }
            }
        } else if self.language == String::from("julia") {
            for command_output_line in splited_command_output {
                let module_name_and_version: Vec<&str> = command_output_line.split_whitespace().collect();
                
                if module_name_and_version.len() == 3 {
                    installed_modules.push(module_name_and_version[1..].join("@"));
                }
            }
        } else {
            panic!("Error")
        };

        self.installed_modules = installed_modules;
    }

    
    fn command_runner(&mut self) -> String {
        if self.language == String::from("python") {
            let output: Output = Command::new("pip3")
                .args(["freeze"])
                .output()
                .expect("failed to execute process");

            return String::from_utf8(output.stdout).unwrap();
            
        } else if self.language == String::from("julia") {
            let output: Output = Command::new("julia")
                .args(["-e", "using Pkg; Pkg.status();"])
                .output()
                .expect("failed to execute process");

            return String::from_utf8(output.stdout).unwrap();
        } else {
            panic!("Error")
        };
    }


    fn detail(&mut self) {
        println!("{}\n", self.path);

        let supported_languages: Vec<String> = vec![
            String::from("python"), 
            String::from("julia"), 
            String::from("go"),
            String::from("other"),
        ];

        let supported_extensions: Vec<String> = vec![
            String::from("py"), 
            String::from("jl"), 
            String::from("go"),
        ];

        let mut py = 0;
        let mut jl = 0;
        let mut ipynb = 0;
        let mut other = 0;

        println!("{}", self.extension);
        for file_path in &self.file_paths {
            println!("{}", file_path);
            for extension in &supported_extensions {
                if file_path.ends_with(extension) && extension == &String::from("py") {
                    py += 1;
                } else if file_path.ends_with(extension) && extension == &String::from("jl") {
                    jl += 1;
                } else if file_path.ends_with(extension) && extension == &String::from("ipynb") {
                    ipynb += 1;
                } else {
                    other += 1;
                }
            }
        }
        let number_of_each_file: Vec<i32> = vec![py, jl, ipynb, other];

        let detail_data: HashMap<_, _> = supported_languages.iter().zip(number_of_each_file.iter()).collect();

        for (language, number_of_file) in detail_data {
            println!("{:<12}{:>3}", language, number_of_file);
        }
    }


    fn generate(&mut self) {
        if self.version {
            self.get_modules_version()
        }

        println!("{}", self.language);
        let mut extractor: ModuleExtractor = ModuleExtractor{ ipynb: true, modules: Vec::new() };

        for file_path in &self.file_paths {
            let source: String = read(file_path);

            if self.language == String::from("python") {
                extractor.python(source);
            } else if self.language == String::from("julia") {
                extractor.julia(source);
            }
        }
        let mut content = String::new();

        for module in &extractor.modules {
            for installed_module in &self.installed_modules {
                if installed_module.starts_with(module) {
                    content.push_str(&format!("{}\n", installed_module));
                }
            }
        }

        write(&self.path, content);
    }
}
