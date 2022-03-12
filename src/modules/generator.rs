use std::error::Error;
use std::fs::{self, ReadDir};
use std::process::{Command, Output};

use crate::modules::extractor::{EachLanguageRegex, ModuleExtractor, ModuleExtractorComponents};
use crate::modules::file::{read_lines, write};

pub trait Operator {
    fn get_directories(&mut self, path: String) -> Result<(), Box<dyn Error>>;
    fn get_files(&mut self, path: String) -> Result<(), Box<dyn Error>>;
}

// TODO: get directory path from command line arguments
impl Operator for RequirementstxtGenerator {
    fn get_directories(&mut self, path: String) -> Result<(), Box<dyn Error>> {
        let mut directories: Vec<String> = Vec::new();
        let mut directories_iterator: ReadDir = fs::read_dir(path)?;
        while let Some(entry) = directories_iterator.next() {
            let entry: fs::DirEntry = entry?;
            let entry_path: String = entry.path().display().to_string();
            directories.push(entry_path);
        }
        self.directory_paths.append(&mut directories);
        Ok(())
    }

    fn get_files(&mut self, extension: String) -> Result<(), Box<dyn Error>> {
        println!("{}", extension);
        let mut files: Vec<String> = Vec::new();
        self.file_paths.append(&mut files);
        Ok(())
    }
}

pub trait RequirementstxtGeneratorComponents {
    fn generate(&mut self);
    fn get_installed_modules(&mut self);
}

pub struct RequirementstxtGenerator {
    pub language: String,
    pub directory_path: String,
    pub is_jupyter: bool,
    pub is_version: bool,
    pub is_show: bool,
    extension: String,
    file_paths: Vec<String>,
    directory_paths: Vec<String>,
    installed_modules: Vec<String>,
}

impl Default for RequirementstxtGenerator {
    fn default() -> RequirementstxtGenerator {
        RequirementstxtGenerator {
            language: String::from(""),
            directory_path: String::from(""),
            is_jupyter: false,
            is_version: false,
            is_show: false,
            extension: String::from(""),
            file_paths: Vec::new(),
            directory_paths: Vec::new(),
            installed_modules: Vec::new(),
        }
    }
}

impl RequirementstxtGeneratorComponents for RequirementstxtGenerator {
    fn generate(&mut self) {
        if self.is_version {
            self.get_installed_modules()
        }

        let language_regex: EachLanguageRegex = match self.language.as_str() {
            "python" => EachLanguageRegex::Python,
            "julia" => EachLanguageRegex::Julia,
            _ => panic!("Language not supported"),
        };

        let mut extractor: ModuleExtractor = ModuleExtractor {
            language: language_regex,
        };

        self.get_directories(self.directory_path.clone()).unwrap();
        self.get_files(self.extension.clone()).unwrap();

        let test_file_paths: Vec<&str> = vec!["test/test_sample.py", "test/test_sample2.py"];

        // test_file_paths -> self.file_paths
        let mut modules: Vec<String> = Vec::new();
        for file_path in test_file_paths {
            let splited_source_code: Vec<String> = read_lines(file_path);
            let mut extracted_modules: Vec<String> = extractor.extract(splited_source_code);
            modules.append(&mut extracted_modules);
        }

        modules.sort();
        modules.dedup();

        let result_path: &String = &format!("{}/requirements.txt", self.directory_path);
        if !self.installed_modules.is_empty() {
            let mut result: Vec<String> = Vec::new();
            for installed_module in &self.installed_modules {
                for module in &modules {
                    if installed_module.starts_with(module.as_str()) {
                        result.push(String::from(installed_module));
                    }
                }
                write(result_path, result.join("\n"));
            }
        } else {
            let result: String = modules.join("\n");
            write(result_path, result);
        }
    }

    fn get_installed_modules(&mut self) {
        if self.language == String::from("python") {
            let output: Output = Command::new("pip3")
                .args(["freeze"])
                .output()
                .expect("failed to execute process");
            let output_string: String = String::from_utf8(output.stdout).unwrap();
            let splited_output: Vec<&str> = output_string.split("\n").collect();
            for module_with_version in splited_output {
                self.installed_modules
                    .push(String::from(module_with_version));
            }
        } else if self.language == String::from("julia") {
            let output: Output = Command::new("julia")
                .args(["-e", "using Pkg; Pkg.status();"])
                .output()
                .expect("failed to execute process");
            let output_string: String = String::from_utf8(output.stdout).unwrap();
            let splited_output: Vec<&str> = output_string.split("\n").collect();

            for module_with_version in splited_output {
                let splited_module_with_version: Vec<&str> =
                    module_with_version.split(" ").collect();

                if splited_module_with_version.len() == 3 {
                    let module_name: &str = splited_module_with_version[1];
                    let module_version: &str = splited_module_with_version[2];
                    self.installed_modules
                        .push(format!("{}@{}", module_name, module_version));
                }
            }
        } else {
            panic!("Error")
        };
    }
}
