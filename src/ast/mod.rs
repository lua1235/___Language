use std::fmt::Display;

use crate::scanner::token::Token;

pub mod ast_visitor;

pub enum Node {
    Empty, // A placeholder node which represents the unparsed program or a parsed, but empty, program.
    Int(i32),
    Char(char),
    Str(Box<String>),
    Array(Box<Vec<Node>>),
    Expr { // An expression statement, of the form EXPR ; 
        expr : Box<Node>,
        next : Box<Node>,
    },
    Block { // A block statement , {...} which is a list of statements
        statements : Box<Node>, // Statements within the block
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
    },
    If {
        cond : Box<Node>,
        t_expr : Box<Node>,
        f_expr : Box<Node>,
    }
}

impl Node {
    fn fstr(&self, prefix : &mut String, islast : bool) -> String {
        let mut s = prefix.clone();
        if let Node::Expr {expr: _, next: _} = self {} else if islast {
            s.push('┗');
            prefix.push_str("   ");
        } else {
            s.push('┣');
            prefix.push_str("┃  ");
        }
        match self {
            Node::Empty => s.push_str("empty"),
            Node::Int(val) => s.push_str(&val.to_string()),
            Node::Char(val) => s.push_str(&format!("'{0}'", val)),
            Node::Str(val) => s.push_str(&format!("\"{0}\"", val)),
            Node::Id { name, val_type : _ } => s.push_str(&name.to_string()),
            Node::Expr {expr, next} => s.push_str(
                &format!("EXPR\n{}\n{}", expr.fstr(prefix, false), next.fstr(prefix, true))),
            Node::InfixOp { op_type, lhs, rhs} => s.push_str(
                &format!("━{:?}\n{}\n{}", op_type, lhs.fstr(prefix, false), rhs.fstr(prefix, true))),
            Node::PrefixOp {op_type, rhs} => s.push_str(
                &format!("━{:?}\n{}", op_type, rhs.fstr(prefix, true))),
            Node::PostfixOp {op_type, lhs} => s.push_str(
                &format!("━{:?}\n{}", op_type, lhs.fstr(prefix, true))),
            Node::Block {statements} => s.push_str(
                &format!("━BLOCK\n{}", statements.fstr(prefix, true))),
            Node::Funct {name, args} => s.push_str(
                &format!("━FUNCTION\n{}{}", 
                    name.fstr(prefix, false), 
                    args
                    .iter()
                    .map(|x| format!("\n{}", x.fstr(prefix, false)))
                    .collect::<String>())),
            Node::Array(elements) => s.push_str(
                &format!("━ARRAY{}", 
                    elements
                    .iter()
                    .map(|x| format!("\n{}", x.fstr(prefix, false)))
                    .collect::<String>())),
            Node::If {cond, t_expr, f_expr} => s.push_str(
                &format!("━IF\n{}\n{}\n{}", cond.fstr(prefix, false), t_expr.fstr(prefix, false), f_expr.fstr(prefix, true)  )
                ),
                _ => todo!(),
        }
        if let Node::Expr {expr: _, next: _} = self {} else {
            for _ in 0..3 {
                prefix.pop();
            }
        }
        s
    }
}

// For now, just printing as reverse polish is enough
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fstr(&mut String::new(), true))
    }
}

