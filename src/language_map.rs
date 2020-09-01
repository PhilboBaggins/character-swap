use std::collections::HashMap;

pub struct LanguageMap
{
    pub map: HashMap<char, Vec<char>>
}

impl LanguageMap
{
    pub fn new() -> Self {
        LanguageMap { map: HashMap::new() }
    }

    pub fn add(&mut self, replacement_char: char, source_chars: &str) {
        for sc in source_chars.chars() {
            let x = self.map.entry(sc).or_insert(Vec::new());
            x.push(replacement_char);
        }
    }

    pub fn replace(&mut self, s: &str) -> String {
        let mut s = s.to_string();
        for (k, v) in &self.map {
            for c in v {
                println!("Replacing '{}' in \"{}\" with '{}'", k, s, c);
                s = s.replace(&k.to_string(), &c.to_string());
            }
        }
        s.to_string()
    }
}
