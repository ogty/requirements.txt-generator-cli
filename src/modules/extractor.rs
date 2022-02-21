pub trait Extractor {
    fn python(&mut self, source: String) -> Vec<String>;
    fn julia(&mut self, source: String) -> Vec<String>;
    fn common(&mut self, source: String) -> Vec<String>;
}


pub struct ModuleExtractor {
    pub ipynb: bool,
    pub modules: Vec<String>,
}


impl Extractor for ModuleExtractor {
    fn python(&mut self, source: String) -> Vec<String> {
        self.common(source);

        self.modules = Vec::new();
        return Vec::new();
    }

    
    fn julia(&mut self, source: String) -> Vec<String> {
        self.common(source);
     
        return Vec::new();
    }


    fn common(&mut self, source: String) -> Vec<String> {
        if self.ipynb {
            // something
        }

        // extract modules
        let splited_source: Vec<&str> = source.split("\n").collect();
        println!("{:?}", splited_source);

        return Vec::new();
    }
}
