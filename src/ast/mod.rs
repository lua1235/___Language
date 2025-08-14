use std::{cell::RefCell, rc::Rc};


use crate::{name_resolution::symbol_table::SymbolTable, scanner::token::Token};

pub mod walker;
pub mod toucher;
pub mod format;



// Recursive datatype representing an ast of the program. Has no methods of its own. Functions
// acting on this datatype should be implemented through the traits ast_walker and ast_toucher
// Node is essentially a box to an ast node that also stores what type the ast node is
pub enum Node {
    Empty, // A placeholder node which represents the unparsed program or a parsed, but empty, program.
    Int(Box<Int>),
    Char(Box<Char>),
    Str(Box<Str>),
    Array(Box<Array>),
    Statement(Box<Statement>),
    Block(Box<Block>),
    Id(Box<Id>),
    InfixOp(Box<InfixOp>),
    PrefixOp(Box<PrefixOp>),
    PostfixOp(Box<PostfixOp>),
    Funct(Box<Funct>),
    If(Box<If>),
}
impl Node {
    pub fn new_int(lnum : &u64, val : &i32) -> Node {
        Node::Int(Box::new(Int {
            lnum : *lnum, 
            val : *val,
        }))
    }

    pub fn new_char(lnum : &u64, val : &char) -> Node {
        Node::Char(Box::new(Char {
            lnum : *lnum, 
            val : *val,
        }))
    }

    pub fn new_str(lnum : &u64, val : &str) -> Node {
        Node::Str(Box::new(Str {
            lnum : *lnum, 
            val : val.to_string(),
        }))
    }

    pub fn new_array(lnum : &u64, val : Vec<Node>) -> Node {
        Node::Array(Box::new(Array {
            lnum : *lnum, 
            val : val,
        }))
    }

    pub fn new_statement(lnum : &u64, expr : Node, next : Node) -> Node {
        Node::Statement(Box::new(Statement {
            lnum : *lnum, 
            expr : expr, 
            next : next,
        }))
    }

    pub fn new_block(lnum : &u64, statements : Node, scope : Option<Rc<RefCell<SymbolTable>>>) -> Node {
        Node::Block(Box::new(Block {
            lnum : *lnum,
            statements : statements,
            scope : scope,
        }))
    }

    pub fn new_id(lnum : &u64, name : &str) -> Node {
        Node::Id(Box::new(Id {
            lnum : *lnum, 
            name : name.to_string(),
        }))
    }

    pub fn new_infix(lnum : &u64, op_type : &Token, lhs : Node, rhs : Node) -> Node {
        Node::InfixOp(Box::new(InfixOp {
            lnum : *lnum,
            op_type : op_type.clone(),
            lhs : lhs,
            rhs : rhs,
        }))
    }

    pub fn new_prefix(lnum : &u64, op_type : &Token, rhs : Node) -> Node {
        Node::PrefixOp(Box::new(PrefixOp {
            lnum : *lnum,
            op_type : op_type.clone(),
            rhs : rhs,
        }))
    }

    pub fn new_postfix(lnum : &u64, op_type : &Token, lhs : Node) -> Node {
        Node::PostfixOp(Box::new(PostfixOp {
            lnum : *lnum,
            op_type : op_type.clone(),
            lhs : lhs,
        }))
    }
    pub fn new_funct(lnum : &u64, name : Node, args : Vec<Node>) -> Node {
        Node::Funct(Box::new(Funct {
            lnum : *lnum,
            name : name,
            args : args,
        }))
    }

    pub fn new_if(lnum : &u64, cond : Node, t_expr : Node, f_expr : Node) -> Node {
        Node::If(Box::new(If {
            lnum : *lnum,
            cond : cond,
            t_expr : t_expr,
            f_expr : f_expr,
        }))
    }
}

pub struct Int {
    pub lnum : u64,
    pub val : i32,
}

pub struct Char {
    pub lnum : u64,
    pub val : char,
}

pub struct Str {
    pub lnum : u64,
    pub val : String,
}

pub struct Array {
    pub lnum : u64,
    pub val : Vec<Node>,
}

// An expression statement, of the form EXPR ; NEXT
pub struct Statement { 
    pub lnum : u64,
    pub expr : Node,
    pub next : Node,
}

// A block statement , {...} which contains a statement
pub struct Block { 
    pub lnum : u64,
    pub statements : Node, // Statements within the block
    pub scope : Option<Rc<RefCell<SymbolTable>>>, // pointer to symbol table of current scope
}

pub struct Id {
    pub lnum : u64,
    pub name : String,
}

pub struct InfixOp {
    pub lnum : u64,
    pub op_type : Token,
    pub lhs : Node,
    pub rhs : Node,
}

pub struct PrefixOp {
    pub lnum : u64,
    pub op_type : Token,
    pub rhs : Node
}

pub struct PostfixOp {
    pub lnum : u64,
    pub op_type : Token,
    pub lhs : Node,
}

pub struct Funct {
    pub lnum : u64,
    pub name : Node,
    pub args : Vec<Node>
}

pub struct If {
    pub lnum : u64,
    pub cond : Node,
    pub t_expr : Node,
    pub f_expr : Node,
}
