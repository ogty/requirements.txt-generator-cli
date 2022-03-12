extern crate regex;
use regex::{Match, Regex};

pub use crate::modules::file::read_lines;

pub enum EachLanguageRegex {
    Python,
    Julia,
}

impl EachLanguageRegex {
    fn get_regex(&self) -> Vec<Regex> {
        match self {
            EachLanguageRegex::Python => vec![
                Regex::new(r"^import (?P<module>.+)").unwrap(),
                Regex::new(r"^from (?P<module>.+) import").unwrap(),
            ],
            EachLanguageRegex::Julia => vec![
                Regex::new(r"^using (?P<module>.+)").unwrap(),
                Regex::new(r"^import (?P<module>.+)").unwrap(),
            ],
        }
    }
}

pub trait ModuleExtractorComponents {
    fn extract(&mut self, source_code: Vec<String>) -> Vec<String>;
}

pub struct ModuleExtractor {
    pub language: EachLanguageRegex,
}

impl ModuleExtractorComponents for ModuleExtractor {
    fn extract(&mut self, splited_source_code: Vec<String>) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for line in splited_source_code {
            for regex in self.language.get_regex() {
                let captures: Vec<String> = regex
                    .captures(&line)
                    .unwrap()
                    .iter()
                    .map(|x: Option<Match>| x.unwrap().as_str().to_string())
                    .collect();
                result.push(captures[1].clone());
            }
        }
        result
    }
}
