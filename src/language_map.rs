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

    pub fn char_replace(&mut self, orig_char: char) -> char{
        match self.map.get(&orig_char) {
            Some(rep_chars) => rep_chars[0],
            None => orig_char,
        }
    }

    pub fn replace(&mut self, s: &str) -> String {
        s.chars().map(|c| self.char_replace(c)).collect()
    }
}
