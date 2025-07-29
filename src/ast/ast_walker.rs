use crate::scanner::token::Token;

use super::Node;

// Immutable Ast traversal
pub trait AstWalker<T> {
    // Traverses the sub-AST rooted at n, possibly updates state
    fn walk(&mut self, n : &Node) -> T {
        self.match_variant(n)
    }

    // Wrapper around the match block to facilitate custom implementations of walk
    fn match_variant(&mut self, n : &Node) -> T {
        match n {
            Node::Empty => self.walk_empty(),
            Node::Int(i) => self.walk_int(i),
            Node::Char(c) => self.walk_char(c),
            Node::Str(s) => self.walk_str(s),
            Node::Array(a) => self.walk_array(a),
            Node::Statement { expr, next } => self.walk_statement(expr, next),
            Node::Block { statements } => self.walk_block(statements),
            Node::Id { name, val_type } => self.walk_id(name, val_type),
            Node::InfixOp { op_type, lhs, rhs } => self.walk_infix(op_type, lhs, rhs),
            Node::PrefixOp { op_type, rhs } => self.walk_prefix(op_type, rhs),
            Node::PostfixOp { op_type, lhs } => self.walk_postfix(op_type, lhs),
            Node::Funct { name, args } => self.walk_funct(name, args),
            Node::If { cond, t_expr, f_expr } => self.walk_if(cond, t_expr, f_expr),
        }

    }

    fn walk_empty(&mut self) -> T;
    fn walk_int(&mut self, val : &i32) -> T;
    fn walk_char(&mut self, val : &char) -> T;
    fn walk_str(&mut self, val : &str) -> T;
    fn walk_array(&mut self, elements : &Box<Vec<Node>>) -> T;
    fn walk_statement(&mut self, expr : &Node, next : &Node) -> T;
    fn walk_block(&mut self, statements : &Node) -> T;
    fn walk_id(&mut self, name : &str, val_type : &Token) -> T;
    fn walk_infix(&mut self, op_type : &Token, lhs : &Node, rhs : &Node) -> T;
    fn walk_prefix(&mut self, op_type : &Token, rhs : &Node) -> T;
    fn walk_postfix(&mut self, op_type : &Token, lhs : &Node) -> T;
    fn walk_funct(&mut self, name : &Node, args : &Box<Vec<Node>>) -> T;
    fn walk_if(&mut self, cond : &Node, t_expr : &Node, f_expr : &Node) -> T;
}
