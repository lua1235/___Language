use std::fmt::Display;

use crate::scanner::token::Token;

pub mod walker;
pub mod format;
pub mod symbol_table;

// Recursive datatype representing an ast of the program. Has no methods of its own. Functions
// acting on this datatype should be implemented through the traits ast_walker and ast_toucher
pub enum Node {
    Empty, // A placeholder node which represents the unparsed program or a parsed, but empty, program.
    Int(i32),
    Char(char),
    Str(Box<String>),
    Array(Box<Vec<Node>>),
    Statement { // An expression statement, of the form EXPR ; 
        expr : Box<Node>,
        next : Box<Node>,
    },
    Block { // A block statement , {...} which is a list of statements
        statements : Box<Node>, // Statements within the block
    },
    Id {
        name : String,
    },
    InfixOp {
        op_type : Token,
        lhs : Box<Node>,
        rhs : Box<Node>
    },
    PrefixOp {
        op_type : Token,
        rhs : Box<Node>
    },
    PostfixOp {
        op_type : Token,
        lhs : Box<Node>
    },
    Funct {
        name : Box<Node>,
        args : Box<Vec<Node>>
    },
    If {
        cond : Box<Node>,
        t_expr : Box<Node>,
        f_expr : Box<Node>,
    }
}

