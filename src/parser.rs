pub mod parser {
    use crate::tokenizer::tokenizer::{Tokens, TokenList};

    pub enum Level {
        ONE,
        TWO,
        THREE,
    }

    pub enum Sentences {
        BOLD(String),
        ITALIC(String),
        NORMAL(String),
    }

    pub struct Heading {
        level: Level,
        text: String,
    }

    pub struct Paragraph {
        sentences: Vec<Sentences>
    }

    pub enum MdElements {
        Heading{ 
            level: Level,
            text: String,
        },
        Paragraph(Paragraph),
    }

    pub fn parse(tokens: &TokenList) -> Vec<MdElements> {
        let mut intermediate_md: Vec<MdElements> = vec![];
        
        let mut idx = 0;
        let mut jump_indexes = 1;

        while idx < tokens.list.len() {
            jump_indexes = match tokens.list.get(idx).expect("This shouldn't panic") {
                Tokens::NEWLINE => check_for_heading(tokens, idx, &mut intermediate_md),
                Tokens::HASHTAG => todo!(),
                Tokens::ASTERISK => todo!(),
                Tokens::UNDERSCORE => todo!(),
                Tokens::HYPHEN => todo!(),
                Tokens::TEXT(_) => todo!(),
            };

            idx += jump_indexes;

        }

        intermediate_md
    }

    fn check_for_heading(tokens: &TokenList, idx: usize, im: &mut Vec<MdElements>) -> usize {
        let list = match tokens.list.get(idx..idx+6) {
            Some(value) => value,
            None => tokens.list.get(idx..).unwrap_or_default(),
        };

        use Tokens::*;

        match &list[..] {
            [NEWLINE, HASHTAG, HASHTAG, HASHTAG, SPACE, TEXT(txt)] => {
                im.push(MdElements::Heading{ 
                    level: Level::THREE,
                    text: txt.to_string(),
                });
                6
            },
            [NEWLINE, HASHTAG, HASHTAG, SPACE, TEXT(txt), _] => {
                im.push(MdElements::Heading{ 
                    level: Level::TWO,
                    text: txt.to_string(),
                });
                5
            },
            [NEWLINE, HASHTAG, SPACE, TEXT(txt), _, _] => {
                im.push(MdElements::Heading{ 
                    level: Level::ONE,
                    text: txt.to_string(),
                });
                4
            },
            _ => 1,
        }
    }

    
    fn check_for_paragraphs(tokens: &TokenList) {
        todo!();
    }
}

