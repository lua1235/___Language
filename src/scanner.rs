use crate::common::Token;
use std::io::{self, BufRead, Bytes, Lines, Read};
use std::io::BufReader;
use std::iter::Peekable;
use std::str::Chars;

pub fn scan<T : BufRead>(file : &mut Lines<T>) -> Vec<Token> {
    let mut tokens : Vec<Token> = Vec::new();
    for (lnum, line) in file.enumerate() {
        let tok = match line {
            Ok(l) => parse_token(&mut l.chars().peekable()),
            Err(e) => panic!("Problem reading file on line {lnum:?}: {e:?}"),
        };
        tokens.push(tok);
    }

    tokens
}

fn match_keyword(buffer : &str) -> Token {
    match buffer {
        "while" => Token::While,
        "if" => Token::If,
        "else" => Token::Else,
        "int" => Token::IntKey,
        "return" => Token::Ret,
        _ => Token::Id(buffer.to_string()),
    }
}

fn parse_id_or_key(reader : &mut Peekable<Chars>, buffer : &mut String) -> Token {
    loop {
        // Lookahead to only consume characters which can go in an identifier
        match reader.peek() {
            None => break,
            Some(x) => if x.is_ascii_alphanumeric() || *x == '_' {
                buffer.push(*x);
            } else {
                break;
            },
        }
        reader.next();
    }
    match_keyword(buffer)
}

fn parse_token(reader : &mut Peekable<Chars>) -> Token {
    // Strip whitespace from the iterator
    while reader.peek().is_some() {
        if reader.peek().unwrap().is_whitespace() {
            reader.next();
        } else {
            break;
        }
    }
    // Simple 2 layer switch case to parse character by character
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
                    parse_id_or_key(reader, &mut other.to_string())
                } else if other.is_ascii_hexdigit() {
                    parse_const(reader  )

                }
            },
        }


    };
    tok
}
