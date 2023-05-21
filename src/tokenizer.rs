pub mod tokenizer {
    #[derive(Debug)]
    pub enum Tokens {
        Heading1(String),
        Heading2(String),
        Heading3(String),
        Normal(String),
    }

    #[derive(Debug)]
    pub struct TokenList {
        pub list: Vec<Tokens>
    }

    impl TokenList {

        pub fn tokenize(&mut self, md_line: &str) {
            match (md_line.chars().nth(0), md_line.chars().nth(1), md_line.chars().nth(2)) {
                (Some('#'), Some('#'), Some('#')) =>    self.list.push(Tokens::Heading3(String::from(&md_line[3..]))),
                (Some('#'), Some('#'), _) =>            self.list.push(Tokens::Heading2(String::from(&md_line[2..]))),
                (Some('#'), _, _) =>                    self.list.push(Tokens::Heading1(String::from(&md_line[1..]))),
                (_,_,_) =>                              self.list.push(Tokens::Normal(String::from(md_line))),
            }
        }

        pub fn parse(&self) -> String {
            let mut html_result = String::from("<html>\n");

            self.list.iter().for_each(|token| {
                match token {
                    Tokens::Heading1(txt) =>    html_result.push_str(&format!("<h1>{}</h1>\n", txt)),
                    Tokens::Heading2(txt) =>    html_result.push_str(&format!("<h2>{}</h2>\n", txt)),
                    Tokens::Heading3(txt) =>    html_result.push_str(&format!("<h3>{}</h3>\n", txt)),
                    Tokens::Normal(txt) =>      html_result.push_str(&format!("<p>{}</p>\n", txt)),
                };
            });

            html_result.push_str("</html>\n");

            html_result
        }
    }

}

#[cfg(test)]
mod tests {
    use super::tokenizer::TokenList;
    use super::tokenizer::Tokens;

    #[test]
    fn heading_works() {
        let mut tokens = TokenList {
            list: vec![],
        };

        tokens.tokenize("# This is a heading");
        tokens.tokenize("## This is a heading2");
        tokens.tokenize("### This is a heading3");
        assert_eq!(tokens.list.len(), 3);
    }

    #[test]
    fn heading_shouldnt_run() {
        let mut tokens = TokenList {
            list: vec![],
        };

        tokens.tokenize("This is not a heading");
        // TODO: fix up this test
        assert_eq!(tokens.list.len(), 1);
    }

    #[test]
    fn parse_headings() {
        let tokens = TokenList {
            list: vec![Tokens::Heading1(String::from("Hello, World"))],
        };
        
        assert_eq!(&tokens.parse(), "<h1>Hello, World</h1>
");
    }
}

