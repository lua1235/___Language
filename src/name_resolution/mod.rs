use std::{cell::RefCell, rc::Rc};

use symbol_table::{Symbol, SymbolTable, TypeBase};

use crate::ast::{self, Node, toucher::AstToucher};

pub mod symbol_table;

// Perform name resolution and type checking on the ast, and create a tree of scopes that is bound
// to the ast.
pub struct Resolver {
    scope_stack : Vec<Rc<RefCell<SymbolTable>>>
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            scope_stack : Vec::from([Rc::new(RefCell::new(SymbolTable::new()))])
        }
    }
}

impl AstToucher<Symbol> for Resolver {
    fn walk_empty(&mut self) -> Symbol {
        todo!()
    }

    fn walk_int(&mut self, inner : &mut ast::Int) -> Symbol {
        todo!()
    }

    fn walk_char(&mut self, inner : &mut ast::Char) -> Symbol {
        todo!()
    }

    fn walk_str(&mut self, inner : &mut ast::Str) -> Symbol {
        todo!()
    }

    fn walk_array(&mut self, inner : &mut ast::Array) -> Symbol {
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

