use std::fmt::Display;

use crate::scanner::token::Token;

pub enum Node {
    Empty, // A placeholder node which represents the unparsed program or a parsed, but empty, program.
    Int(i32),
    Expr { // An expression statement, of the form EXPR ; 
        expr : Box<Node>,
        next : Box<Node>,
    },
    Block { // A block statement , {...} which is a list of statements
        statements : Box<Node>, // Statements within the block
        next : Box<Node>, // The statement directly following this block
    },
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
    },
    PostfixOp {
        op_type : Token,
        lhs : Box<Node>
    },
    Funct {
        name : Box<Node>,
        args : Box<Vec<Node>>
    }
}
impl Node {
    fn fstr(&self, indent: u32) -> String {
        let offset = 2;
        let mut s : String = (0..indent).map(|_| " ").collect();
        match self {
            Node::Empty => s.push_str("empty"),
            Node::Int(val) => s.push_str(&val.to_string()),
            Node::Id { name, val_type : _ } => s.push_str(&name.to_string()),
            Node::Block { statements, next } => s.push_str(
                &format!("(BLOCK\n{}\n{})", statements.fstr(indent + offset), next.fstr(indent + offset))),
            Node::Expr {expr, next} => s.push_str(
                &format!("(EXPR\n{}\n{})", expr.fstr(indent + offset), next.fstr(indent + offset))),
            Node::InfixOp { op_type, lhs, rhs} => s.push_str(
                &format!("({:?}\n{}\n{})", op_type, lhs.fstr(indent + offset), rhs.fstr(indent + offset))),
            Node::PrefixOp {op_type, rhs} => s.push_str(
                &format!("({:?}\n{})", op_type, rhs.fstr(indent + offset))),
            Node::PostfixOp {op_type, lhs} => s.push_str(
                &format!("({:?}\n{})", op_type, lhs.fstr(indent + offset))),
            Node::Funct {name, args} => s.push_str(
                &format!("({:?}\n{})", 
                    name.fstr(indent), 
                    args
                    .iter()
                    .map(|x| format!("{}\n", x.fstr(indent)))
                    .collect::<String>())),
            _ => todo!(),
        }
        s
    }
}

// For now, just printing as reverse polish is enough
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fstr(0))
    }
}

