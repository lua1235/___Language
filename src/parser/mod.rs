use std::{collections::HashSet, io::{BufRead, Read}};
use ast::Node;
use crate::scanner::{token::Token, Scanner};

mod ast;

pub struct Parser {
    eof_read : bool,
    ast_head : Box<Node>,
    paren_stack : Vec<u32>, // Used to match brackets. 
    open_bracks : u32,

}

impl Parser {
    pub fn new() -> Self {
        Self {
            eof_read : false,
            ast_head : Box::new(Node::Empty),
            paren_stack : vec![0],
            open_bracks : 0,
        }
    }

    pub fn gen_ast<T : Read>(&mut self, tokens : &mut Scanner<T>) {
        self.ast_head = self.parse(tokens, 0, &HashSet::new());
    }

    pub fn print_ast(&mut self) {
        println!("{}", *self.ast_head);
    }

    // This parser uses pratt parsing, which works somewhat similarly to recursive descent. It will
    // return the current ast upon encountering the provided match_tok, which cleanly handles
    // matching of brackets and parentheses
    fn parse<T : Read>(&mut self, tok_it : &mut Scanner<T>, min_bp : u32, match_tok : &HashSet<Token>) -> Box<Node> {
        // Invariant: The left of the current position of the parser in the token stream has
        // been fully parsed into a single ast.
        if self.eof_read {
            return Box::new(Node::Empty)
        }
        let Some(x) = tok_it.peek() else {
            self.eof_read = true;
            return Box::new(Node::Empty)
        };
        if match_tok.contains(&x) {
            return Box::new(Node::Empty)
        }
        // Don't advance if we encountered a match_tok: Instead, return until the parse which
        // started the match can handle it
        tok_it.next();
        let mut left = match x {
            // Program-level patterns
            // Encounter the counterpart to an open Token pair
            Token::EOF => {
                self.eof_read = true;
                return Box::new(Node::Empty)
            },
            Token::LCurly => { // Start scope
                self.paren_stack.push(0);
                let mut mt = HashSet::new();
                mt.insert(Token::RCurly);
                let expr = self.parse(tok_it, 0, &mt);
                let Some(Token::RCurly) = tok_it.next() else {
                    panic!("Unmatched curly braces")
                };
                let Some(x) = self.paren_stack.pop() else {
                    panic!("No current stack frame (WTF)")
                };
                if x != 0 {
                    panic!("Unclosed parentheses in current scope")
                }
                let next;
                if let Some(Token::Semi) = tok_it.peek() {
                    next = Box::new(Node::Empty);
                } else {
                    next = self.parse(tok_it, 0, match_tok);
                }
                return Box::new(Node::Block {
                    statements : expr,
                    next : next,
                });
            },
            // Expression-level patterns
            Token::IntConst(i) => Box::new(Node::Int(i)),
            Token::Id(s) => Box::new(Node::Id{
                name : s.to_string(),
                val_type : Token::IntKey, // Placeholder, need lookup table 
            }),
            Token::LParen => {
                *self.paren_stack.last_mut().unwrap() += 1;
                // Parse the inside of the paren
                let mut mt = HashSet::new();
                mt.insert(Token::RParen);
                let l = self.parse(tok_it, 0, &mt);
                // Consume the close bracket
                let Some(Token::RParen) = tok_it.next() else {
                    panic!("Unmatched open parenthesis")
                };
                *self.paren_stack.last_mut().unwrap() -= 1;
                l
            },
            // Check if is prefix operator. 
            op => {
                let Some(((), rbp)) = self.get_prefix_bp(&op, match_tok) else {
                    panic!("Error, bad prefix operator")
                };
                Box::new(Node::PrefixOp{
                    op_type : op.clone(),
                    rhs : self.parse(tok_it, rbp, match_tok)
                })
            },
        };
        // Each iteration, the iterator is positioned at an operator which 
        // will join the current left sub tree with a new, recursively calculated right subtree. 
        // We also advance the iterator past the right subtree, and set the tree with this operator as root.
        // let mut lookahead = tok_it.clone();
        while let Some(op) = tok_it.peek() {
            // First check if it is a postfix operator
            if let Some((lbp, ())) = self.get_postfix_bp(&op, match_tok) {
                // The subtree to the left of this is more strongly attracted to the previous operator
                if lbp < min_bp {
                    return left;
                }
                tok_it.next();
                left = match op {
                    // Array indexing
                    Token::LBrack => {
                        self.open_bracks += 1;
                        let mut mt = HashSet::new();
                        mt.insert(Token::RBrack);
                        let ind = self.parse(tok_it, 0, &mt);
                        let Some(Token::RBrack) = tok_it.next() else {
                            panic!("Unmatched open bracket")
                        };
                        self.open_bracks -= 1;
                        Box::new(Node::InfixOp {
                            op_type : op.clone(),
                            lhs : left,
                            rhs : ind
                        })
                    },
                    // Function call
                    Token::LParen => {
                        let mut mt = HashSet::new();
                        mt.insert(Token::RParen);
                        mt.insert(Token::Comma);
                        let mut args = Box::new(Vec::new());
                        let mut ai = self.parse(tok_it, 0, &mt);
                        while let Some(Token::Comma) = tok_it.peek() {
                            tok_it.next();
                            args.push(*ai);
                            ai = self.parse(tok_it, 0, &mt);
                        }
                        let Some(Token::RParen) = tok_it.next() else {
                            panic!("Unmatched open paren")
                        };
                        args.push(*ai);
                        Box::new(Node::Funct {
                            name : left,
                            args : args
                        })

                    },
                    _ => Box::new(Node::PostfixOp {
                        op_type : op.clone(),
                        lhs : left
                    }),
                };
                continue;
            }
            // Get the binding power, or return if a close-bracket is detected
            if let Some((lbp, rbp)) = self.get_infix_bp(&op, match_tok) {
                // The subtree to the left of this is more strongly attracted to the previous operator
                if lbp < min_bp {
                    return left;
                }
                // Advance the main iterator once we are sure we will consume this token
                tok_it.next();
                // Now that we have consumed it, we can increment our number of open brackets
                // Calculate the right subtree
                let right = self.parse(tok_it, rbp, match_tok);
                if let Token::Semi = op {
                    if self.open_bracks != 0 {
                        panic!("Unmatched open brackets in this expression")
                    }
                    left = Box::new(Node::Expr {
                        expr : left,
                        next : right,
                    });
                } else {
                    left = Box::new(Node::InfixOp {
                        op_type : op.clone(), // This is fast enough for tokens
                        lhs : left,
                        rhs : right,
                    });
                }
                continue;
            }
            println!("Returning on token {:?}", op);
            // If we reach here, it means the token is something we want to ignore
            return left;
        }
        self.eof_read = true;
        left
    }

    fn get_prefix_bp(&mut self, tok : & Token, end_tok : & HashSet<Token>) -> Option<((), u32)> {
        if end_tok.contains(tok) {
            return None;
        }
        let ret = match tok {
            Token::Inc | Token::Dec => ((), 28),
            Token::IntKey => ((), 34),
            _ => panic!("Bad prefix operator : {:?}\nCurrent end_tok : {:?}", tok, end_tok),
        };
        Some(ret)
    }

    fn get_postfix_bp(&mut self, tok : & Token, end_tok : & HashSet<Token>) -> Option<(u32, ())> {
        if end_tok.contains(tok) {
            return None;
        }
        let ret = match tok {
            Token::LBrack => (30, ()), // Think of array subscript as an postfix operator
            Token::LParen => (36, ()), // Function call postfix operator has high precedence
            Token::Inc | Token::Dec => (30, ()),
            _ => return None,
        };
        Some(ret)
    }

    // Return the left and right binding powers of an infix operator. Different precedence levels
    // correspond to even binding power values. Odd values are used to represent associativity
    fn get_infix_bp(&mut self, tok : & Token, end_tok : & HashSet<Token>) -> Option<(u32, u32)> {
        if end_tok.contains(tok) {
            return None;
        }
        let ret = match tok {
            // Treat semicolons as a left associative infix operator which joins expressions
            Token::Semi => (0, 1),
            Token::Assign 
                | Token::AddAss 
                | Token::SubAss 
                | Token::MulAss 
                | Token::DivAss => (32, 4), // assignment operators are right associative
            Token::Equal | Token::NotEq => (18, 19),
            Token::GT | Token::GE | Token::LT | Token::LE => (20, 21),
            Token::Add | Token::Sub => (24, 25),
            Token::Star | Token::Div => (26, 27),
            _ => panic!("Bad binary operator : {:?}\nCurrent end_tok : {:?}", tok, end_tok),
        };
        Some(ret)
    }

}



