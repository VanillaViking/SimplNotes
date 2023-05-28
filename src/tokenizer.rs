pub mod tokenizer {
    #[derive(Debug, PartialEq)]
    pub enum Tokens {
        HASHTAG,
        ASTERISK,
        UNDERSCORE,
        HYPHEN,
        TEXT(String),
        NEWLINE,
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
                    '\n' => self.list.push(Tokens::NEWLINE),
                    _ => {
                        if let Some(Tokens::TEXT(txt)) = self.list.last_mut() {
                            txt.push(char);
                        } else {
                            self.list.push(Tokens::TEXT(String::from(char)))
                        }
                    }
                }
            });
        }

    }

}

#[cfg(test)]
mod tests {
    use crate::tokenizer::tokenizer::Tokens::*;

    use super::tokenizer::TokenList;

    #[test]
    fn tokenize_correctly() {
        let mut tokens = TokenList {
            list: vec![]
        };

        tokens.tokenize("# Hello World
the quick brown **fox** jumps over the _lazy_ dog.");

        assert_eq!(tokens.list[..], [
            HASHTAG, 
            TEXT(String::from(" Hello World")),
            NEWLINE,
            TEXT(String::from("the quick brown ")),
            ASTERISK,
            ASTERISK,
            TEXT(String::from("fox")),
            ASTERISK,
            ASTERISK,
            TEXT(String::from(" jumps over the ")),
            UNDERSCORE,
            TEXT(String::from("lazy")),
            UNDERSCORE,
            TEXT(String::from(" dog.")),
        ])
    }
}

