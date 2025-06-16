use std::iter::Peekable;
use ast::Node;
use crate::scanner::token::Token;

mod ast;

pub struct Parser {
    ast_head : Box<Node>,
    open_paren : u32, // Used to match brackets. 
}

impl Parser {
    pub fn new() -> Self {
        Self {
            ast_head : Box::new(Node::Empty),
            open_paren : 0,
        }
    }

    pub fn gen_ast(&mut self, tokens : & Vec<Token>) {
        self.ast_head = self.parse(&mut tokens.iter().peekable(), 0);
    }

    pub fn print_ast(&mut self) {
        println!("{}", *self.ast_head);
    }

    // This parser uses pratt parsing, which works somewhat similarly to recursive descent. 
    fn parse<'a, T : Iterator<Item=&'a Token>>(&mut self, tok_it : &mut Peekable<T>, min_bp : u32) -> Box<Node> {
        // Invariant: The left of the current position of the parser in the token stream has
        // been fully parsed into a single ast.
        let mut left = match tok_it.next() {
            None => panic!("Error, empty token stream"),
            Some(x) => match x {
                Token::EOF => return Box::new(Node::Empty),
                Token::IntConst(i) => Box::new(Node::Int(*i)),
                Token::Id(s) => Box::new(Node::Id{
                                name : s.to_string(),
                                val_type : Token::IntKey, // Placeholder, need lookup table
                            }),
                Token::LParen => {
                    self.open_paren += 1;
                    self.parse(tok_it, 0)
                },
                x => panic!("Bad token {x:?}"),
            }
        };
        // Each iteration, the iterator is positioned at an (infix) operator which 
        // will join the current left sub tree with A new, recursively calculated right subtree. 
        // We also advance the iterator past the right subtree, and set the tree with this operator as root.
        // let mut lookahead = tok_it.clone();
        while let Some(op) = tok_it.peek().cloned() {
            if let Token::EOF = *op {
                return left;
            }
            // Return the binding power, or return if a close-bracket is detected
            let Some((lbp, rbp)) = self.get_infix_binding_powers(op) else {
                return left;
            };
            // The subtree to the left of this is more strongly attracted to the previous operator
            if lbp < min_bp {
                return left;
            }
            // Advance the main iterator once we are sure we will consume this token
            tok_it.next();
            // Calculate the right subtree
            let right = self.parse(tok_it, rbp);
            if let Node::Empty = *right {
                return Box::new(Node::InfixOp {
                    op_type : op.clone(), // This is fast enough for tokens
                    lhs : left,
                    rhs : right,
                })
            }
            left = Box::new(Node::InfixOp {
                op_type : op.clone(), // This is fast enough for tokens
                lhs : left,
                rhs : right,
            });
        }
        panic!("Bad token stream: end reached without encountering Token::EOF");
    }

    // Return the left and right binding powers of an infix operator. Different precedence levels
    // correspond to even binding power values. Odd values are used to represent associativity
    fn get_infix_binding_powers(&mut self, tok : & Token) -> Option<(u32, u32)> {

        let ret = match tok {
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
            Token::RParen => {
                self.open_paren -= 1;
                return None
            }, // This  
            _ => panic!("Bad binary operator"),
        };
        Some(ret)
    }

}



