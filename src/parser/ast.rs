use std::fmt::Display;

use crate::scanner::token::Token;

pub enum Node {
    Empty, // A placeholder node which represents the unparsed program or a parsed, but empty, program.
    Int(i32),
    Id {
        name : String,
        val_type : Token, 
    },
    InfixOp {
        op_type : Token,
        lhs : Box<Node>,
        rhs : Box<Node>
    },
    PrefixOp {
        op_type : Token,
        rhs : Box<Node>
    }
}

// For now, just printing as reverse polish is enough
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Empty => write!(f, "empty"),
            Node::Int(val) => write!(f, "{}", val),
            Node::Id { name, val_type } => write!(f, "{}", name),
            Node::InfixOp { op_type, lhs, rhs} => write!(f, "({:?} {} {})", op_type, lhs, rhs),
            Node::PrefixOp {op_type, rhs} => write!(f, "({:?} {})", op_type, rhs)
        }
    }
}

