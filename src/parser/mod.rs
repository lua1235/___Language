use std::{iter::Peekable, slice::Iter};

use ast::Node;
use crate::scanner::token::Token;

mod ast;

struct Parser {
    ast_head : Box<Node>
}

impl Parser {
    pub fn new() -> Self {
        Self {
            ast_head : Box::new(Node::Empty),
        }
    }

    pub fn gen_ast(&mut self, tokens : & Vec<Token>) {
        self.ast_head = self.parse(&mut tokens.iter().peekable(), 0);
    }

    // This parser uses pratt parsing, which works somewhat similarly to recursive descent. 
    fn parse(& self, tok_it : &mut Peekable<Iter<Token>>, min_bp : u32) -> Box<Node> {
        // Invariant: The left of the current position of the parser in the token stream has
        // been fully parsed into a single ast.
        let mut left = match tok_it.next() {
            None => panic!("Error, empty token stream"),
            Some(x) => match x {
                Token::EOF => Box::new(Node::Empty),
                Token::IntConst(i) => Box::new(Node::Int(*i)),
                Token::Id(s) => Box::new(Node::Id{
                                name : s.to_string(),
                                val_type : Token::IntKey, // Placeholder, need lookup table
                            }),
                x => panic!("Bad token {x:?}"),
            }
        };
        // Each iteration, the iterator is positioned at an (infix) operator which 
        // will join the current left sub tree with A new, recursively calculated right subtree. 
        // We also advance the iterator past the right subtree, and set the tree with this operator as root.
        loop {
            let Some(op) = tok_it.peek() else {
                panic!("Bad token stream: End reached without finding Token::EOF")
            };
            if let Token::EOF = op {
                break
            }
            // Return the binding power if op 
            let (lbp, rbp) = self.get_binding_powers(op);
            // The subtree to the left of this is more strongly attracted to the previous operator
            if lbp < min_bp {
                break
            }
            tok_it.next();
            // Calculate the right subtree
            let right = self.parse(tok_it, rbp);
            left = Box::new(Node::InfixOp {
                op_type : **op,
                lhs : left,
                rhs : right,
            })

        }
        // Return left, which is now the 
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
                | Token::DivAss => (5, 4), // assignment operators are right associative
            Token::Equal | Token::NotEq => (18, 19),
            Token::GT | Token::GE | Token::LT | Token::LE => (20, 21),
            Token::Add | Token::Sub => (24, 25),
            Token::Star | Token::Div => (26, 27),
            _ => panic!("Bad operator token {tok:?}"),
        }
    }

}



