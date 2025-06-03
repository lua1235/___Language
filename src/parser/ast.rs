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


