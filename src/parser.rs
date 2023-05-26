pub mod parser {
    use crate::tokenizer::tokenizer::{Tokens, TokenList};

    enum Level {
        ONE,
        TWO,
        THREE,
    }

    struct Bold {
        text: String,
    }

    struct Italic {
        text: String,
    }

    pub struct Heading {
        level: Level,
        text: String,
    }

    enum Sentences {
        BOLD(Bold),
        ITALIC(Italic),
        NORMAL(String),
    }

    struct Paragraph {
        sentences: Vec<Sentences>
    }

    pub fn parse(tokens: &TokenList) {
        tokens.list.iter().for_each(|token| {
            match token {
                Tokens::HASHTAG => ()
            }
        })
    }
}

