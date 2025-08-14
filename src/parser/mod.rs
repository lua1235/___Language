use std::{collections::HashSet, io::Read};
use crate::scanner::{token::Token, Scanner};
use crate::ast::{self, Node};


pub struct Parser {
    eof_read : bool,
    open_bracks : u32,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            eof_read : false,
            open_bracks : 0,
        }
    }

    pub fn gen_ast<T : Read>(&mut self, tokens : &mut Scanner<T>) -> Node {
        return self.parse(tokens, 0, &HashSet::new());
    }

    // Return the ast representing a parenthesised expression. The open parenthesis should have
    // already been consumed
    fn parse_paren<T : Read>(&mut self, tok_it : &mut Scanner<T>) ->Node {
        // Parse the inside of the paren
        let lnum = tok_it.lnum;
        let l = self.parse(tok_it, 2, &HashSet::from([Token::RParen]));
        // Consume the close bracket
        match tok_it.next() {
            Some(Token::RParen) => l,
            Some(x) => panic!("No closing parenthesis for open parenthesis on line {lnum} : Encountered {x:?}"),
            None => panic!("No closing parenthesis for open parenthesis on line {lnum} : EOF"),
        }
    }

    // Return the ast representing a scoped expression. The open curly bracket should have already
    // been consumed
    fn parse_scope<T : Read>(&mut self, tok_it : &mut Scanner<T>) -> Node {
        let lnum = tok_it.lnum;
        let expr = self.parse(tok_it, 0, &HashSet::from([Token::RCurly]));
        let Some(Token::RCurly) = tok_it.next() else {
            panic!("No closing brace for open curly braces on line {lnum}")
        };
        Node::Block(Box::new(ast::Block {
            lnum : lnum,
            statements : expr,
            scope : None
        }))
    }

    // Return the ast representing a parenthesised expression. The open parenthesis should have
    // already been consumed
    fn parse_array<T : Read>(&mut self, tok_it : &mut Scanner<T>) ->Node {
        let lnum = tok_it.lnum;
        let mt = HashSet::from([Token::RBrack, Token::Comma]);
        let mut elements = Vec::new();
        let mut e = self.parse(tok_it, 2, &mt);
        while let Some(Token::Comma) = tok_it.peek() {
            tok_it.next();
            elements.push(e);
            e = self.parse(tok_it, 0, &mt);
        }
        let Some(Token::RBrack) = tok_it.next() else {
            panic!("Array on line {lnum} not closed")
        };
        elements.push(e);
        Node::new_array(&lnum, elements)
    }

    // This parser uses pratt parsing, which works somewhat similarly to recursive descent. It will
    // return the current ast upon encountering the provided match_tok, which cleanly handles
    // matching of brackets and parentheses.
    // There are 2 reserved minimum binding powers. min_bp 0 parses at the statement level. (Return
    // ast of the entire program)
    // min_bp 2 parses at the expression level (Return ast of the next expression)
    fn parse<T : Read>(&mut self, tok_it : &mut Scanner<T>, min_bp : u32, match_tok : &HashSet<Token>) -> Node {
        // Invariant: The left of the current position of the parser in the token stream has
        // been fully parsed into a single ast.
        if self.eof_read {
            return Node::Empty
        }
        let Some(x) = tok_it.peek() else {
            self.eof_read = true;
            return Node::Empty
        };
        if match_tok.contains(&x) {
            return Node::Empty
        }
        // Don't advance if we encountered a match_tok: Instead, return until the parse which
        // started the match can handle it
        tok_it.next();
        let mut left = match x {
            // Program-level patterns
            // Encounter the counterpart to an open Token pair
            Token::EOF => {
                self.eof_read = true;
                return Node::Empty
            },
            // Primary Expressions
            Token::IntConst(i) => Node::new_int(&tok_it.lnum, &i), // Int constant
            Token::CharConst(c) => Node::new_char(&tok_it.lnum, &c), // Char constant
            Token::StrConst(s) => Node::new_str(&tok_it.lnum, &s), // String constant
            Token::Id(s) => Node::new_id(&tok_it.lnum, &s),
            Token::LCurly => self.parse_scope(tok_it),
            // Parenthesis expressions
            Token::LParen => self.parse_paren(tok_it),
            // Constant Array expressions
            Token::LBrack => self.parse_array(tok_it), 
            Token::If => { // If expressions
                let lnum = tok_it.lnum;
                let Some(Token::LParen) = tok_it.next() else {
                    panic!("Expected parenthesis after if on line {lnum}")
                };
                // Parse the expression on the inside of the paren
                let condition = self.parse_paren(tok_it);
                // Consume the close bracket
                let mut tmatch_tok = match_tok.clone(); // Not too bad since this is only ever 2-3
                tmatch_tok.insert(Token::Else); // Doesn't matter to us if Else is already being
                                                // matched
                let tbranch = self.parse(tok_it, 2, &tmatch_tok);
                let fbranch = if let Some(Token::Else) = tok_it.peek() {
                    tok_it.next();
                    self.parse(tok_it, 2, match_tok)
                } else {
                    Node::Empty
                };
                Node::new_if(&lnum, condition, tbranch, fbranch)
            },
            // Prefix expressions. 
            op => {
                let Some(((), rbp)) = self.get_prefix_bp(&op, match_tok) else {
                    panic!("Error, bad prefix operator")
                };
                let right = self.parse(tok_it, rbp, match_tok);
                Node::new_prefix(&tok_it.lnum, &op, right)
            },
        };
        // Each iteration, the iterator is positioned at an operator which 
        // will join the current left sub tree with a new, recursively calculated right subtree. 
        // We also advance the iterator past the right subtree, and set the tree with this operator as root.
        // let mut lookahead = tok_it.clone();
        while let Some(op) = tok_it.peek() {
            let lnum = tok_it.lnum;
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
                        Node::new_infix(&lnum, &op, left, ind)
                    },
                    // Function call
                    Token::LParen => {
                        let mut mt = HashSet::new();
                        mt.insert(Token::RParen);
                        mt.insert(Token::Comma);
                        let mut args = Vec::new();
                        let mut ai = self.parse(tok_it, 0, &mt);
                        while let Some(Token::Comma) = tok_it.peek() {
                            tok_it.next();
                            args.push(ai);
                            ai = self.parse(tok_it, 0, &mt);
                        }
                        let Some(Token::RParen) = tok_it.next() else {
                            panic!("Unmatched open paren")
                        };
                        args.push(ai);
                        Node::new_funct(&lnum, left, args)
                    },
                    _ => Node::new_postfix(&lnum, &op, left),
                };
                continue;
            }
            // Get the binding power, or return if a close-bracket is detected
            if let Some((lbp, rbp)) = self.get_infix_bp(&op, match_tok, lnum) {
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
                    left = Node::new_statement(&lnum, left, right);
                } else {
                    left = Node::new_infix(&lnum, &op, left, right);
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
            Token::Ret => ((), 4),
            Token::Inc | Token::Dec => ((), 28),
            Token::IntKey | Token::CharKey => ((), 34),
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
    fn get_infix_bp(&mut self, tok : & Token, end_tok : & HashSet<Token>, lnum : u64) -> Option<(u32, u32)> {
        if end_tok.contains(tok) {
            return None;
        }
        let ret = match tok {
            // Treat semicolons as a left associative infix operator which joins expressions
            Token::Semi => (1, 0),
            Token::Assign 
                | Token::AddAss 
                | Token::SubAss 
                | Token::MulAss 
                | Token::DivAss => (32, 4), // assignment operators are right associative
            Token::Equal | Token::NotEq => (18, 19),
            Token::GT | Token::GE | Token::LT | Token::LE => (20, 21),
            Token::Add | Token::Sub => (24, 25),
            Token::Star | Token::Div => (26, 27),
            _ => panic!("Bad binary operator on line {}: {:?}\nCurrent end_tok : {:?}",lnum, tok, end_tok),
        };
        Some(ret)
    }

}



