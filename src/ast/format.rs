use crate::scanner::token::Token;

use super::{walker::AstWalker, Node};

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
        if let Node::Statement {expr: _, next: _} = n {} else if self.last_child {
            s.push('┗');
            self.prefix_stack.push_str("   ");
        } else {
            s.push('┣');
            self.prefix_stack.push_str("┃  ");
        }
        self.last_child = false;
        s.push_str(&self.match_variant(n));
        if let Node::Statement {expr: _, next: _} = n {} else {
            for _ in 0..3 {
                self.prefix_stack.pop();
            }
        }
        s
    }

    fn walk_empty(&mut self) -> String {
        "EMPTY".to_string()
    }

    fn walk_int(&mut self, val : &i32) -> String {
        val.to_string()
    }

    fn walk_char(&mut self, val : &char) -> String {
        format!("'{0}'", val)
    }

    fn walk_str(&mut self, val : &str) -> String {
        format!("\"{0}\"", val)
    }

    fn walk_array(&mut self, elements : &Box<Vec<Node>>) -> String {
        format!("━ARRAY{}", 
            elements
            .iter()
            .map(|x| format!("\n{}", self.walk(x)))
            .collect::<String>())
    }

    fn walk_statement(&mut self, expr : &Node, next : &Node) -> String {
        let expr = self.walk(expr);
        self.last_child = true;
        let next = self.walk(next);
        format!("EXPR\n{}\n{}", expr, next)
    }

    fn walk_block(&mut self, statements : &Node) -> String {
        self.last_child = true;
        format!("━BLOCK\n{}", self.walk(statements))
    }

    fn walk_id(&mut self, name : &str) -> String {
        format!("━{}", name)
    }

    fn walk_infix(&mut self, op_type : &Token, lhs : &Node, rhs : &Node) -> String {
        let lhs = self.walk(lhs);
        self.last_child = true;
        let rhs = self.walk(rhs);
        format!("━{:?}\n{}\n{}",op_type, lhs, rhs)
    }

    fn walk_prefix(&mut self, op_type : &Token, rhs : &Node) -> String {
        self.last_child = true;
        format!("━{:?}\n{}", op_type, self.walk(rhs))
    }

    fn walk_postfix(&mut self, op_type : &Token, lhs : &Node) -> String {
        self.last_child = true;
        format!("━{:?}\n{}", op_type, self.walk(lhs))
    }

    fn walk_funct(&mut self, name : &Node, args : &Box<Vec<Node>>) -> String {
        format!("━FUNCTION\n{}{}", 
            self.walk(name), 
            args
            .iter()
            .map(|x| format!("\n{}", self.walk(x)))
            .collect::<String>())
    }

    fn walk_if(&mut self, cond : &Node, t_expr : &Node, f_expr : &Node) -> String {
        let c = self.walk(cond);
        let t = self.walk(t_expr);
        self.last_child = true;
        let f = self.walk(f_expr);
        format!("━IF\n{}\n{}\n{}", c, t, f)
    }


}

