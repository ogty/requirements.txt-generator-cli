extern crate regex;
use regex::Regex;

pub use crate::modules::file::read_lines;


pub enum EachLanguageRegex {
    Python,
    Julia,
}

// I don't like something about the name of this implementation.
impl EachLanguageRegex {
    fn get_regex(&self) -> Regex {
        // TODO: Implement regex for other languages
        // TODO: There may be more than one pattern of regular expressions...
        //       So we need an array to manage regular expressions?
        match self {
            EachLanguageRegex::Python => Regex::new(r"^import (?P<module>.+)").unwrap(),
            EachLanguageRegex::Julia => Regex::new(r"^using (?P<module>.+)").unwrap(),
        }
    }
}

pub trait Components {
    fn extract(&mut self, source_code: Vec<String>) -> Vec<String>;
}

// TOOD: Receives directory path and language regex
pub struct ModuleExtractor {
    pub language_regex: EachLanguageRegex,
}


impl Components for ModuleExtractor {
    fn extract(&mut self, splited_source_code: Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for line in splited_source_code {
            if let Some(captures) = self.language_regex.get_regex().captures(&line) {
                let module: String = captures.name("module").unwrap().as_str().to_string();
                result.push(module);
            }
        }
        result
    }
}