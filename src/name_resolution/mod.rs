use std::{cell::RefCell, rc::Rc};

use symbol::{Symbol, SymbolTable, Types, };

use crate::ast::{self, Node, toucher::AstToucher};

pub mod symbol;

// Perform name resolution and type checking on the ast, and create a tree of scopes that is bound
// to the ast.
pub struct Resolver {
    // Each different stack frame gets its own symbol table
    frame_tables : Vec<SymbolTable>,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            frame_tables : Vec::from([SymbolTable::new(0)]),
        }
    }
}

// Return the type information of the subast rooted at the node, if the subtree is valid
impl AstToucher<Types> for Resolver {
    fn walk_empty(&mut self) -> Types {
        Types::Int
    }

    fn walk_int(&mut self, inner : &mut ast::Int) -> Types {
        Types::Int
    }

    fn walk_char(&mut self, inner : &mut ast::Char) -> Types {
        Types::Char
    }

    fn walk_str(&mut self, inner : &mut ast::Str) -> Types {
        Types::Pointer(Box::new(Types::Char))
    }

    fn walk_array(&mut self, inner : &mut ast::Array) -> Types {
        let array_line = inner.lnum;
        let inner_type = inner.val.iter_mut().fold(
            None, 
            |acc, element| {
                let mut ret_val = acc.clone();
                let Some(s_table) = self.frame_tables.last() else {
                    panic!("No stack frame")
                };
                s_table.push_scope();
                let Some(curr_type) = acc else {
                    ret_val = Some(self.walk(element));
                    s_table.pop_scope();
                    return ret_val
                };
                let ele_type = self.walk(element);
                if curr_type != ele_type {
                    panic!("Array on line {:?} has elements of different type\n", array_line)
                }
                s_table.pop_scope();
                ret_val
            });
    }

    fn walk_statement(&mut self, inner : &mut ast::Statement) -> Types {
        self.walk(&mut inner.expr);
        self.walk(&mut inner.next)
    }

    fn walk_block(&mut self, inner : &mut ast::Block) -> Types {
        self.frame_tables.last_mut().expect("Left global frame").push_scope();
        self.walk(&mut inner.statements)
    }

    fn walk_id(&mut self, inner : &mut ast::Id) -> Types {


    }

    fn walk_infix(&mut self, inner : &mut ast::InfixOp) -> Types {
        todo!()
    }

    fn walk_prefix(&mut self, inner : &mut ast::PrefixOp) -> Types {
        todo!()
    }

    fn walk_postfix(&mut self, inner : &mut ast::PostfixOp) -> Types {
        todo!()
    }

    fn walk_funct(&mut self, inner : &mut ast::Funct) -> Types {
        todo!()
    }

    fn walk_if(&mut self, inner : &mut ast::If) -> Types {
        todo!()
    }
}

