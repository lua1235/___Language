use std::iter::Peekable;
use ast::Node;
use crate::scanner::token::Token;

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

    pub fn gen_ast(&mut self, tokens : & Vec<Token>) {
        self.ast_head = self.parse(&mut tokens.iter().peekable(), 0);
    }

    pub fn print_ast(&mut self) {
        println!("{}", *self.ast_head);
    }

    // This parser uses pratt parsing, which works somewhat similarly to recursive descent. It will
    // automatically stop when it encounters a right brace, paren, or bracket
    fn parse<'a, T : Iterator<Item=&'a Token>>(&mut self, tok_it : &mut Peekable<T>, min_bp : u32) -> Box<Node> {
        // Invariant: The left of the current position of the parser in the token stream has
        // been fully parsed into a single ast.
        if self.eof_read {
            return Box::new(Node::Empty)
        }
        let Some(x) = tok_it.next() else {
            panic!("Error, empty token stream")
        };
        let mut left = match x {
            // Program-level patterns
            Token::EOF => {
                self.eof_read = true;
                return Box::new(Node::Empty)
            },
            Token::LCurly => { // Start scope
                self.paren_stack.push(0);
                Box::new(Node::Block{
                    statements : self.parse(tok_it, 0), // Evaluate inside of block
                    next : self.parse(tok_it, 0), // Evaluate rest of code
                })
            },
            Token::RCurly => { // End scope
                let Some(parens) = self.paren_stack.pop() else {
                    panic!("Unmatched end of scope }}");
                };
                if parens != 0 {
                    panic!("Not all parenthesis closed at end of scope");
                }
                return Box::new(Node::Empty)
            }
            // Expression-level patterns
            Token::IntConst(i) => Box::new(Node::Int(*i)),
            Token::Id(s) => Box::new(Node::Id{
                name : s.to_string(),
                val_type : Token::IntKey, // Placeholder, need lookup table 
            }),
            Token::LParen => {
                *self.paren_stack.last_mut().unwrap() += 1;
                // Parse the inside of the paren
                let l = self.parse(tok_it, 0);
                // Consume the close bracket
                tok_it.next();
                *self.paren_stack.last_mut().unwrap() -= 1;
                l
            },
            // Check if is prefix operator. 
            op => {
                let Some(((), rbp)) = self.get_prefix_bp(op) else {
                    panic!("Error, bad prefix operator")
                };
                Box::new(Node::PrefixOp{
                    op_type : op.clone(),
                    rhs : self.parse(tok_it, rbp)
                })
            },
        };
        // Each iteration, the iterator is positioned at an operator which 
        // will join the current left sub tree with a new, recursively calculated right subtree. 
        // We also advance the iterator past the right subtree, and set the tree with this operator as root.
        // let mut lookahead = tok_it.clone();
        while let Some(op) = tok_it.peek().cloned() {
            if let Token::EOF = *op {
                self.eof_read = true;
                break;
            }
            // First check if it is a postfix operator
            if let Some((lbp, ())) = self.get_postfix_bp(op) {
                // The subtree to the left of this is more strongly attracted to the previous operator
                if lbp < min_bp {
                    return left;
                }
                tok_it.next();
                left = Box::new(Node::PostfixOp {
                    op_type : op.clone(),
                    lhs : left
                });
                continue;
            }
            // Get the binding power, or return if a close-bracket is detected
            if let Some((lbp, rbp)) = self.get_infix_bp(op) {
                // The subtree to the left of this is more strongly attracted to the previous operator
                if lbp < min_bp {
                    return left;
                }
                // Advance the main iterator once we are sure we will consume this token
                tok_it.next();
                // Now that we have consumed it, we can increment our number of open brackets
                if let Token::LBrack = op {
                    self.open_bracks += 1;
                }
                // Calculate the right subtree
                let right = self.parse(tok_it, rbp);
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
            println!("Close bracket");
            // If we reach here, it means the token is something we want to ignore
            // tok_it.next(); // Consume the close-bracket
            return left;
        }
        if !self.eof_read {
            panic!("Bad token stream: end reached without encountering Token::EOF");
        }
        left
    }

    fn get_prefix_bp(&mut self, tok : & Token) -> Option<((), u32)> {
        let ret = match tok {
            Token::Inc | Token::Dec => ((), 28),
            Token::IntKey => ((), 34),
            _ => panic!("Bad prefix operator")
        };
        Some(ret)
    }

    fn get_postfix_bp(&mut self, tok : & Token) -> Option<(u32, ())> {
        let ret = match tok {
            Token::Inc | Token::Dec => (30, ()),
            _ => return None,
        };
        Some(ret)
    }

    // Return the left and right binding powers of an infix operator. Different precedence levels
    // correspond to even binding power values. Odd values are used to represent associativity
    fn get_infix_bp(&mut self, tok : & Token) -> Option<(u32, u32)> {

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
            Token::LBrack => (30, 0), // Think of array subscript as an infix operator
            Token::RBrack => {
                if self.open_bracks < 1 {
                    panic!("Too many right brackets: ]")
                }
                self.open_bracks -= 1;
                return None;
            },
            Token::RParen => {
                return None
            }, // This  
            _ => panic!("Bad binary operator"),
        };
        Some(ret)
    }

}



