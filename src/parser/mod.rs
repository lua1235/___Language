use std::{iter::Peekable, slice::Iter};

use ast::Node;
use crate::scanner::token::Token;

mod ast;

struct Parser<'a> {
    tokens: &'a Vec<Token>,
    ast_head : Box<Node>
}

impl Parser<'_> {
    pub fn new(tokens : & Vec<Token>) -> Self {
        Self {
            tokens : tokens,
            ast_head : Box::new(Node::Empty),
        }
    }

    pub fn gen_ast(&self) {
        self.ast_head = self.parse(&mut self.tokens.iter().peekable(), 0);
    }

    // This parser uses pratt parsing, which works somewhat similarly to recursive descent. 
    fn parse(& self, tok_it : &mut Peekable<Iter<Token>>, min_bp : i32) -> Box<Node> {
        // Invariant: The left of the current position of the parser in the token stream has
        // been fully parsed into a single ast.
        let left = if let Some(x) = tok_it.next() {
            match x {
                Token::EOF => Box::new(Node::Empty),
                Token::IntConst(i) => Box::new(Node::Int(*i)),
                Token::Id(s) => Box::new(Node::Id{
                                name : s.to_string(),
                                val_type : Token::IntKey, // Placeholder, need lookup table
                            }),
                x => panic!("Bad token {x:?}"),
            }
        } else {
            panic!("Error, empty token stream");
        };
        // Each iteration, we read some (infix) operator which joins the current left sub tree with
        // A new, recursively calculated right subtree. We also advance the iterator past the
        // right subtree, and set the tree with this operator as root.
        loop {
            let Some(op) = tok_it.peek() else {
                panic!("Bad token stream: End reached without finding Token::EOF")
            };
            if let Token::EOF = op {
                break
            }
            let (lbp, rbp) = self.get_binding_powers(op);
            


        }

        left
    }

    // Return the left and right binding powers of an infix operator. Different precedence levels
    // correspond to even binding power values. Odd values are used to represent associativity
    fn get_binding_powers(& self, tok : & Token) -> (u32, u32) {
        match tok {
            // Treat semicolons as a left associative infix operator which joins expressions
            Token::Semi => (0, 1),
            Token::Assign 
                | Token::AddAss 
                | Token::SubAss 
                | Token::MulAss 
                | Token::DivAss => (5, 4), // Right assoc
            Token::Equal | Token::NotEq => (18, 19),
            Token::GT | Token::GE | Token::LT | Token::LE => (20, 21),
            Token::Add | Token::Sub => (24, 25),
            Token::Star | Token::Div => (26, 27),
            _ => panic!("Bad operator token {tok:?}"),
        }
    }

}



