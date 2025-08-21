use std::{cell::RefCell, rc::Rc};

use crate::{
    ast::{self, toucher::AstToucher, Node}, 
    name_resolution::symbol::{SymbolPtr, SymbolTable, Types,}
};




// Perform dynamic name resolution and type checking on the ast, and 
// to the ast.
pub struct Interpreter {
    table : SymbolTable,
}

impl Interpreter {
    pub fn new() -> Self {
        Resolver {
            table : SymbolTable::new(),
        }
    }
}

// Return the type information of the subast rooted at the node, if the subtree is valid
impl AstToucher<SymbolPtr> for Resolver {
    fn walk_empty(&mut self) -> SymbolPtr {
        SymbolPtr::new_int(Some(0), true)
    }

    fn walk_int(&mut self, inner : &mut ast::Int) -> SymbolPtr {
        SymbolPtr::new_int(Some(inner.val), true)
    }

    fn walk_char(&mut self, inner : &mut ast::Char) -> SymbolPtr {
        SymbolPtr::new_char(Some(inner.val), true)
    }

    // In the name resolution stage, don't actually allocate memory, just assign to a dummy address
    fn walk_str(&mut self, inner : &mut ast::Str) -> SymbolPtr {
        SymbolPtr::new_pointer(Some(0), true, Types::Char)
    }

    fn walk_array(&mut self, inner : &mut ast::Array) -> SymbolPtr {
        //let ele_type = inner.val.iter_mut().fold(None, |acc, x| self.walk(x));
        todo!()
    }

    fn walk_statement(&mut self, inner : &mut ast::Statement) -> SymbolPtr {
        todo!()
    }

    fn walk_block(&mut self, inner : &mut ast::Block) -> SymbolPtr {
        todo!()
    }

    fn walk_id(&mut self, inner : &mut ast::Id) -> SymbolPtr {
        todo!()
    }

    fn walk_infix(&mut self, inner : &mut ast::InfixOp) -> SymbolPtr {
        todo!()
    }

    fn walk_prefix(&mut self, inner : &mut ast::PrefixOp) -> SymbolPtr {
        todo!()
    }

    fn walk_postfix(&mut self, inner : &mut ast::PostfixOp) -> SymbolPtr {
        todo!()
    }

    fn walk_funct(&mut self, inner : &mut ast::Funct) -> SymbolPtr {
        todo!()
    }

    fn walk_if(&mut self, inner : &mut ast::If) -> SymbolPtr {
        todo!()
    }
}

