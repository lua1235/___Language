use super::{Node};

// Mutable Ast traversal
pub trait AstToucher<T> {
    // Traverses the sub-AST rooted at n, possibly updates state
    fn walk(&mut self, n : &mut Node) -> T {
        self.match_variant(n)
    }

    // Wrapper around the match block to facilitate custom implementations of walk
    fn match_variant(&mut self, n : &mut Node) -> T {
        match n {
            Node::Empty => self.walk_empty(),
            Node::Int(val) => self.walk_int(val),
            Node::Char(val) => self.walk_char(val),
            Node::Str(val) => self.walk_str(val),
            Node::Array(val) => self.walk_array(val),
            Node::Statement(val) => self.walk_statement(val),
            Node::Block(val) => self.walk_block(val),
            Node::Id(val) => self.walk_id(val),
            Node::InfixOp(val) => self.walk_infix(val),
            Node::PrefixOp(val) => self.walk_prefix(val),
            Node::PostfixOp(val) => self.walk_postfix(val),
            Node::Funct(val) => self.walk_funct(val),
            Node::If(val) => self.walk_if(val),
        }

    }

    fn walk_empty(&mut self) -> T;
    fn walk_int(&mut self, inner : &mut super::Int) -> T;
    fn walk_char(&mut self, inner : &mut super::Char) -> T;
    fn walk_str(&mut self, inner : &mut super::Str) -> T;
    fn walk_array(&mut self, inner : &mut super::Array) -> T;
    fn walk_statement(&mut self, inner : &mut super::Statement) -> T;
    fn walk_block(&mut self, inner : &mut super::Block) -> T;
    fn walk_id(&mut self, inner : &mut super::Id) -> T;
    fn walk_infix(&mut self, inner : &mut super::InfixOp) -> T;
    fn walk_prefix(&mut self, inner : &mut super::PrefixOp) -> T;
    fn walk_postfix(&mut self, inner : &mut super::PostfixOp) -> T;
    fn walk_funct(&mut self, inner : &mut super::Funct) -> T;
    fn walk_if(&mut self, inner : &mut super::If) -> T;
}
