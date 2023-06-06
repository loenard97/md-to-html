use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
enum Token {
    Headline(usize, String),
    Paragraph(String),
}

#[derive(Debug)]
pub struct Converter {
    tokens: Vec<Token>,
}

impl Converter {
    pub fn from_file(file: fs::File) -> Self {
        let mut tokens = vec![];
        let reader = BufReader::new(file);
        let mut buffer = String::new();
        let mut is_paragraph = false;

        for line in reader.lines() {
            let line = line.unwrap();

            if line == "" {
                if is_paragraph {
                    tokens.push(Token::Paragraph(buffer.clone()));
                    is_paragraph = false;
                    buffer.clear();
                }
                continue;
            }

            if line.starts_with('#') {
                let bangs = line.split(' ').next().unwrap().len();
                let line = &line[bangs + 1..];
                tokens.push(Token::Headline(bangs, line.to_string()));
            
            } else {
                is_paragraph = true;
                buffer.push_str(&line);
                buffer.push('\n');

            }
        }

        Converter { tokens }
    }

    pub fn to_html(&self) -> String {
        let mut ret = String::new();

        for token in self.tokens.iter() {
            match token {
                Token::Headline(level, text) => {
                    ret.push_str("<h");
                    ret.push_str(&level.to_string());
                    ret.push_str(">");
                    ret.push_str(&text);
                    ret.push_str("</h");
                    ret.push_str(&level.to_string());
                    ret.push_str(">\n");
                },
                Token::Paragraph(text) => {
                    ret.push_str("<p>\n");
                    ret.push_str(&text);
                    ret.push_str("</p>\n\n");
                }
            }
        }

        ret
    }
}
