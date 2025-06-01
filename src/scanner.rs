use crate::common::Token;
use std::io::{BufRead, Lines};
use std::iter::Peekable;
use std::slice::Iter;
use std::str::Chars;

pub struct Scanner {
    tokens : Vec<Token>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            tokens : Vec::new()
        }
    }

    // Return an iterator to the underlying vector of tokens
    pub fn get_token_iter(&self) -> Iter<Token> {
        self.tokens.iter()
    }

    // Scan the input character by character, and produce tokens
    pub fn scan<T : BufRead>(&mut self, file : &mut Lines<T>) {
        for (lnum, line) in file.enumerate() {
            println!("Line number {lnum:?}");
            
            let lbuf = match line {
                Err(e) => panic!("Problem reading file on line {lnum:?}: {e:?}"),
                Ok(l) => l,
            };
            let mut lpeek = lbuf.chars().peekable();
            loop {
                match self.read_token(&mut lpeek) {
                    Token::EOL => break, // For now, we just ignore end of line tokens.
                    other => self.tokens.push(other),
                }
            }
        }
        self.tokens.push(Token::EOF);
    }

    // Check if the identifier matches one of the recognized keywords
    fn match_keyword(&self, buffer : &str) -> Token {
        match buffer {
            "while" => Token::While,
            "if" => Token::If,
            "else" => Token::Else,
            "int" => Token::IntKey,
            "return" => Token::Ret,
            _ => Token::Id(buffer.to_string()),
        }
    }

    // For now, just support base 10 ints
    fn read_const(&self, reader : &mut Peekable<Chars>, buffer : &mut String) -> Token {
        while let Some(x) = reader.peek() {
            if !x.is_ascii_digit() {
                break;
            }
            buffer.push(*x);
            reader.next();
        }
        Token::IntConst(buffer.parse().expect("IntConst parsed incorrectly"))
    }

    fn read_id_or_key(&self, reader : &mut Peekable<Chars>, buffer : &mut String) -> Token {
            // Lookahead to only consume characters which can go in an identifier
        while let Some(x) = reader.peek() {
            if !(x.is_ascii_alphanumeric() || *x == '_') {
                break;
            }
            buffer.push(*x);
            reader.next();
        }
        self.match_keyword(buffer)
    }

    fn read_token(&self, reader : &mut Peekable<Chars>) -> Token {
        // Strip whitespace from the iterator
        while let Some(x) = reader.peek() {
            if !x.is_whitespace() {
                break;
            } 
            reader.next();
        }
        // Simple 2 layer switch case to read character by character
        let tok : Token = match reader.next() {
            None => Token::EOL,
            Some(x) => match x {
                '{' => Token::LCurly,
                '}' => Token::RCurly,
                '(' => Token::LParen,
                ')' => Token::RParen,
                '[' => Token::LBrack,
                ']' => Token::RBrack,
                ';' => Token::Semi,
                ':' => Token::Colon,
                ',' => Token::Comma,
                '+' => match reader.peek() {
                    Some('+') => {
                        reader.next();
                        Token::Inc
                    },
                    Some('=') => {
                        reader.next();
                        Token::AddAss
                    }
                    _ => Token::Add,
                },
                '-' => match reader.peek() {
                    Some('-') => {
                        reader.next();
                        Token::Dec
                    },
                    Some('=') => {
                        reader.next();
                        Token::SubAss
                    }
                    _ => Token::Sub,
                },
                '*' => match reader.peek() {
                    Some('=') => {
                        reader.next();
                        Token::MulAss
                    }
                    _ => Token::Star,
                },
                '/' => match reader.peek() {
                    Some('=') => {
                        reader.next();
                        Token::DivAss
                    }
                    _ => Token::Div,
                },
                '=' => match reader.peek() {
                    Some('=') => {
                        reader.next();
                        Token::Equal
                    }
                    _ => Token::Assign,
                },
                '!' => match reader.peek() {
                    Some('=') => {
                        reader.next();
                        Token::NotEq
                    }
                    _ => Token::Not,
                },
                '>' => match reader.peek() {
                    Some('=') => {
                        reader.next();
                        Token::GE
                    }
                    _ => Token::GT,
                },
                '<' => match reader.peek() {
                    Some('=') => {
                        reader.next();
                        Token::LE
                    }
                    _ => Token::LT,
                },
                other => {
                    if other.is_ascii_alphabetic() || other == '_' {
                        self.read_id_or_key(reader, &mut other.to_string())
                    } else if other.is_ascii_digit() {
                        self.read_const(reader,  &mut other.to_string())
                    } else {
                        panic!("unrecognized token {other:?}")
                    }
                },
            }
        };
        tok
    }

}
