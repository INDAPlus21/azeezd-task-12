use std::collections::{HashMap};

/// # `LSystem
/// Structure that holds rules for the LSystem and generates the next generation by taking input to be grown
pub struct LSystem {
    rules: HashMap<char, String>
}

impl LSystem {
    /// # `new`
    /// Create a new L-System with no rules
    pub fn new() -> LSystem {
        LSystem {
            rules: HashMap::new()
        }
    }

    /// # `add_rule`
    /// Add a new rule to the LSystem by taking a character `char` and what that character would become `String`
    pub fn add_rule(&mut self, character: char, becomes: String) {
        self.rules.insert(character, becomes);
    }

    /// # `generate`
    /// Takes a `String` and returns the string after one iteration of the L-System
    pub fn generate(&self, input: String) -> String {
        let mut output = String::new();
        for character in input.chars() {
            match self.rules.get(&character) {
                Some(a) => output.push_str(a),
                _ => output.push(character)
            };
        }

        output
    }
}