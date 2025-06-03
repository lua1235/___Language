use crate::scanner::token::Token;

pub enum Node<'a> {
    Empty, // A placeholder node which represents the unparsed program or a parsed, but empty, program.
    Int(i32),
    Id {
        name : &'a String,
        val_type : Token, 
    },
    InfixOp {
        op_type : Token,
        lhs : Box<Node<'a>>,
        rhs : Box<Node<'a>>
    }
}


