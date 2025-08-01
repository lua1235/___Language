use std::collections::HashMap;


pub enum TypeBase {
    Int,
    Char,
    Pointer,
}

pub enum TypeMods {
    Const,
}


pub struct Symbol {
    base_type : TypeBase,
    is_const : bool,
    modifiers : Vec<TypeMods>,
}

pub struct SymbolTable {
    inner_stack : Vec<HashMap<String, Symbol>>
}
