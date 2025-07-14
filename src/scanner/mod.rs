use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Lines, Read};
use std::iter::Peekable;
use std::str::Chars;

use token::Token;

pub mod token;

pub struct Scanner<T : Read> {
    tokens : VecDeque<Token>,
    lines : Lines<BufReader<T>>,
    pub lnum : u64,
}

impl<T : Read> Iterator for Scanner<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // Fill the token buffer if it is empty
        if self.tokens.is_empty() {
            self.lnum += 1;
            // Scan this line if queue empty
            let Some(line) = self.lines.next() else {return None};
            let lbuf = match line {
                Err(e) => panic!("Problem reading file on line {:?}: {:?}", self.lnum, e),
                Ok(l) => l,
            };
            let mut lpeek = lbuf.chars().peekable();
            loop {
                match self.read_token(&mut lpeek) {
                    Token::EOL => break, // For now, we just ignore end of line tokens.
                    other => self.tokens.push_back(other),
                }
            }
        }
        return self.tokens.pop_front()
    }
}

impl<T : Read> Scanner<T> {
    pub fn new(input : T) -> Self {
        Self {
            tokens : VecDeque::new(),
            lnum : 0,
            lines : BufReader::new(input).lines(),
        }
    }

    // Check next token without advancing
    pub fn peek(&mut self) -> Option<Token> {
        if let Some(t) = self.next()  {
            // Don't actually consume an element
            self.tokens.push_front(t.clone());
            return Some(t);
        } else {
            return None;
        }
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
