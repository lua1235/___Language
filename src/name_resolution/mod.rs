use std::{cell::RefCell, rc::Rc};

use symbol::{Symbol, SymbolTable, Types, };

use crate::ast::{self, Node, toucher::AstToucher};

pub mod symbol;

// Perform name resolution and type checking on the ast, and create a tree of scopes that is bound
// to the ast.
pub struct Resolver {
    table : SymbolTable,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            table : SymbolTable::new(),
        }
    }
}

// Return the type information of the subast rooted at the node, if the subtree is valid
impl AstToucher<Symbol> for Resolver {
    fn walk_empty(&mut self) -> Symbol {
    }

    fn walk_int(&mut self, inner : &mut ast::Int) -> Symbol {
        Symbol::new_int(Some(inner.val), true)
    }

    fn walk_char(&mut self, inner : &mut ast::Char) -> Symbol {
        Symbol::new_char(Some(inner.val), true)
    }

    // In the name resolution stage, don't actually allocate memory, just assign to a dummy address
    fn walk_str(&mut self, inner : &mut ast::Str) -> Symbol {
        Symbol::new_pointer(Some(0), true, Types::Char)
    }

    fn walk_array(&mut self, inner : &mut ast::Array) -> Symbol {
        let ele_type = inner.val.iter_mut().fold(None, |acc, x| self.walk(x));
        todo!()
    }

    fn walk_statement(&mut self, inner : &mut ast::Statement) -> Symbol {
        todo!()
    }

    fn walk_block(&mut self, inner : &mut ast::Block) -> Symbol {
        todo!()
    }

    fn walk_id(&mut self, inner : &mut ast::Id) -> Symbol {
        todo!()
    }

    fn walk_infix(&mut self, inner : &mut ast::InfixOp) -> Symbol {
        todo!()
    }

    fn walk_prefix(&mut self, inner : &mut ast::PrefixOp) -> Symbol {
        todo!()
    }

    fn walk_postfix(&mut self, inner : &mut ast::PostfixOp) -> Symbol {
        todo!()
    }

    fn walk_funct(&mut self, inner : &mut ast::Funct) -> Symbol {
        todo!()
    }

    fn walk_if(&mut self, inner : &mut ast::If) -> Symbol {
        todo!()
    }
}

