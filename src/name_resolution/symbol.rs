use std::{cell::RefCell, collections::HashMap, rc::Rc};

// Formal list of defined types in the language
#[derive(Clone, PartialEq, Eq)]
pub enum Types {
    Int,
    Char,
    Pointer(Box<Types>),
    Funct(Vec<Types>),
}

const SIZES : [u8; 4] = [
    32, // Int
    32, // Char
    32, // Ptr
    32, // Funct
];

// Each variable is represented as a "symbol", which contains type information and the offset
// of that variable from the frame ptr. Name resolution will point all ast::Id nodes to symbols.
// Symbols are also used to represent temporary values, such as the return of a function or the
// value of a scope before it is assigned to a variable.
// structs associated to the symbol table. 
#[derive(Clone)]
pub struct Symbol {
    pub id : String,
    pub offset : u32, // offset from frame pointer
    pub is_const : bool,
    pub is_captured : bool,
    pub val_type : Types
}

// The symbol table is a wrapper around a vec (stack) of hashmaps. Only needed for name resolution.
// Once symbols have been resolved, each ast::Id node will point to its corresponding symbol, so
// lookup via table is no longer required.
pub struct SymbolTable {
    mapping : Vec<HashMap<String, Symbol>>
}

impl SymbolTable {
    // Create a new symbol table with no parent
    pub fn new() -> Self {
        SymbolTable {
            mapping : Vec::new(),
        }
    }

    // Create a new scope
    fn push_scope(&mut self) {
        self.mapping.push(HashMap::new())
    }

    // Pop a scope. The Symbols of that scope are still owned by matching ast::Id nodes, so those
    // are not destroyed
    fn pop_scope(&mut self) -> Option<HashMap<String, Symbol>> {
        self.mapping.pop()
    }

    // Check if an identifier name corresponds to a variable local to the current scope
    fn is_local(&self, name : &str) -> bool {
        self.mapping
            .last()
            .unwrap_or_else(|| panic!("No scope (left global scope)"))
            .contains_key(name)
    }

    // Check if an identifier is a defined variable
    fn is_defined(&self, name : &str) -> bool {
        todo!()
    }

    // Return a SymbolPtr pointing to the symbol currently mapped to the name (If it exists)
    fn get_symbol(&self, name : &str) -> Option<Symbol> {
        self.mapping
            .iter()
            .rev()
            .find_map(|scope| {scope.get(name).cloned()})
    }

    // Insert into the current scope if it exists
    fn insert(&mut self, name : &str, sym : &Symbol) {
        self.mapping.last_mut()
            .unwrap_or_else(|| panic!("No scope (left global scope)"))
            .insert(name.to_string(), sym.clone());
    }

}
