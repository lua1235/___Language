use std::{cell::RefCell, collections::HashMap, rc::Rc};


// structs associated to the symbol table.
#[derive(Clone)] 
pub enum TypeBase {
    Int,
    Char,
    Pointer,
    Funct,
}

#[derive(Clone)] 
pub struct Symbol {
    pub base_type : TypeBase,
    pub is_const : bool,
    // Additional type info for special types
    pub modifiers : Vec<TypeBase>,
}

// Each symbolTable points to its enclosing scope unless it is the table corresponding to the
// global scope
pub struct SymbolTable {
    mapping : HashMap<String, Symbol>,
    parent : Option<Rc<RefCell<SymbolTable>>>
}

impl SymbolTable {
    // Create a new symbol table with no parent
    pub fn new() -> Self {
        SymbolTable {
            mapping : HashMap::new(),
            parent : None,
        }
    }

    // Create a symbol table pointing to an enclosing scope
    fn new_child(parent : &Rc<RefCell<Self>>) -> Self {
        SymbolTable {
            mapping : HashMap::new(),
            parent : Some(Rc::clone(parent))
        }
    }

    // Check if an identifier name corresponds to a variable local to the current scope
    fn is_local(&self, name : &str) -> bool {
        self.mapping.contains_key(name)
    }

    // Check if an identifier is a defined variable
    fn is_defined(&self, name : &str) -> bool {
        self.mapping.contains_key(name) || 
            if let Some(x) = &self.parent {
                (*x).borrow().is_defined(name)
            } else {
                false
            }
    }

    // Return a copy of the symbol currently mapped to the name 
    fn get_symbol(&self, name : &str) -> Option<Symbol> {
        if let Some(sym) = self.mapping.get(name) {
            Some(sym.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().get_symbol(name)
        } else {
            None
        }
    }

    fn insert(&mut self, name : &str, sym : &Symbol) {
        self.mapping.insert(name.to_string(), sym.clone());
    }

}
