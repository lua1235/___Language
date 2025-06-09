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
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Empty => todo!(),
            Node::Int(_) => todo!(),
            Node::Id { name, val_type } => todo!(),
            Node::InfixOp { op_type, lhs, rhs } => todo!(),
        }
    }
}

