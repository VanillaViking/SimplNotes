pub mod tokenizer {
    #[derive(Debug)]
    pub enum Tokens {
        HASHTAG,
        ASTERISK,
        UNDERSCORE,
        HYPHEN,
        TEXT(String),
    }

    #[derive(Debug)]
    pub struct TokenList {
        pub list: Vec<Tokens>
    }

    impl TokenList {

        pub fn tokenize(&mut self, md_line: &str) {
            md_line.chars().for_each(|char| {
                match char {
                    '#' => self.list.push(Tokens::HASHTAG),
                    '*' => self.list.push(Tokens::ASTERISK),
                    '_' => self.list.push(Tokens::UNDERSCORE),
                    '-' => self.list.push(Tokens::HYPHEN),
                    _ => {
                        if let Some(Tokens::TEXT(s)) = self.list.last() {
                            self.list.pop();
                            self.list.push(Tokens::TEXT(String::from("{s}{char}")))
                        }
                    }
                }
            })
        }

    }

}

#[cfg(test)]
mod tests {
}

