use std::{cell::RefCell, rc::Rc};

use crate::{name_resolution::symbol_table::SymbolTable, scanner::token::Token};

use super::{ walker::AstWalker, Node};

// Provide methods to print out a formatted AST
pub struct AstFormat {
    prefix_stack : String,
    last_child : bool,
}

impl AstFormat {
    pub fn new() -> Self {
        Self {
            prefix_stack : String::new(),
            last_child : false,
        }
    }
}

impl AstWalker<String> for AstFormat {
    fn walk(&mut self, n : &Node) -> String {
        let mut s = self.prefix_stack.clone();
        if let Node::Statement(_) = n {} else if self.last_child {
            s.push('┗');
            self.prefix_stack.push_str("   ");
        } else {
            s.push('┣');
            self.prefix_stack.push_str("┃  ");
        }
        self.last_child = false;
        s.push_str(&self.match_variant(n));
        if let Node::Statement(_) = n {} else {
            for _ in 0..3 {
                self.prefix_stack.pop();
            }
        }
        s
    }

    fn walk_empty(&mut self) -> String {
        "EMPTY".to_string()
    }

    fn walk_int(&mut self, inner : &super::Int) -> String {
        inner.val.to_string()
    }

    fn walk_char(&mut self, inner : &super::Char) -> String {
        format!("'{0}'", inner.val)
    }

    fn walk_str(&mut self, inner : &super::Str) -> String {
        format!("\"{0}\"", inner.val)
    }

    fn walk_array(&mut self, inner : &super::Array) -> String {
        format!("━ARRAY{}", 
            inner.val
            .iter()
            .map(|x| format!("\n{}", self.walk(x)))
            .collect::<String>())
    }

    fn walk_statement(&mut self, inner : &super::Statement) -> String {
        let expr = self.walk(&inner.expr);
        self.last_child = true;
        let next = self.walk(&inner.next);
        format!("EXPR\n{}\n{}", expr, next)
    }

    fn walk_block(&mut self, inner : &super::Block) -> String {
        self.last_child = true;
        format!("━BLOCK\n{}", self.walk(&inner.statements))
    }

    fn walk_id(&mut self, inner : &super::Id) -> String {
        format!("━{}", inner.name)
    }

    fn walk_infix(&mut self, inner : &super::InfixOp) -> String {
        let lhs = self.walk(&inner.lhs);
        self.last_child = true;
        let rhs = self.walk(&inner.rhs);
        format!("━{:?}\n{}\n{}",inner.op_type, lhs, rhs)
    }

    fn walk_prefix(&mut self, inner : &super::PrefixOp) -> String {
        self.last_child = true;
        format!("━{:?}\n{}", inner.op_type, self.walk(&inner.rhs))
    }

    fn walk_postfix(&mut self, inner : &super::PostfixOp) -> String {
        self.last_child = true;
        format!("━{:?}\n{}", inner.op_type, self.walk(&inner.lhs))
    }

    fn walk_funct(&mut self, inner : &super::Funct) -> String {
        format!("━FUNCTION\n{}{}", 
            self.walk(&inner.name), 
            inner.args
            .iter()
            .map(|x| format!("\n{}", self.walk(x)))
            .collect::<String>())
    }

    fn walk_if(&mut self, inner : &super::If) -> String {
        let c = self.walk(&inner.cond);
        let t = self.walk(&inner.t_expr);
        self.last_child = true;
        let f = self.walk(&inner.f_expr);
        format!("━IF\n{}\n{}\n{}", c, t, f)
    }

}

