use std::{collections::HashMap, ops::Deref, rc::Rc};

// Formal list of defined types in the language
#[derive(Clone)]
// All types have an additional boolean for whether it is assignable
pub enum Types { 
    Undefined(bool),
    Int(bool),
    Char(bool),
    Pointer(bool, Box<Types>),
    Funct(bool, Vec<Types>),
}

impl Types {
    pub fn assignable(&self) -> bool {
        match self {
            Types::Undefined(ass) => ass,
            Types::Int(ass) => ass,
            Types::Char(ass) => ass,
            Types::Pointer(ass, _) => ass,
            Types::Funct(ass, _) => ass,
        }.clone()
    }
}

impl PartialEq for Types {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Types::Undefined(_), Types::Undefined(_)) => true,
            (Types::Int(_), Types::Int(_)) => true,
            (Types::Char(_), Types::Char(_)) => true,
            (Types::Pointer(_, type_self), Types::Pointer(_, type_other)) => type_self.eq(type_other),
            (Types::Funct(_, sig_self), Types::Funct(_, sig_other)) => sig_self == sig_other,
            _ => false,
        }
    }
}

const SIZES : [u8; 4] = [
    32, // Int
    32, // Char
    32, // Ptr
    32, // Funct
];

#[derive(Clone)]
pub struct SymbolPtr {
    inner : Rc<Symbol>,
    pub is_local : bool,
}

impl SymbolPtr {
    pub fn new(inner : Symbol) -> Self {
        SymbolPtr {
            inner : Rc::new(inner),
            is_local : true,
        }

    }
}

impl Deref for SymbolPtr {
    type Target = Symbol;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

// Each variable is represented as a "symbol", which contains type information and the offset
// of that variable from the frame ptr. Name resolution will point all ast::Id nodes to symbols.
// Symbols are also used to represent temporary values, such as the return of a function or the
// value of a scope before it is assigned to a variable.
// structs associated to the symbol table. 
pub struct Symbol {
    pub id : Option<String>,
    // The id of the function that owns the stack frame this var belongs to. None if global frame
    pub frame_id : usize, 
    pub offset : usize, // offset from frame pointer.
    pub is_const : bool,
    pub is_captured : bool,
    pub val_type : Types
}

// A wrapper around a hashmap with a frame_id 
pub struct Scope {
    pub map : HashMap<String, SymbolPtr>,
    pub frame_id : usize,
}

impl Scope {
    pub fn new(fid : usize) -> Self {
        Scope {
            map : HashMap::new(),
            frame_id : fid,
        }
    }
}

// The symbol table is a wrapper around a vec (stack) of hashmaps. Only needed for name resolution.
// Once symbols have been resolved, each ast::Id node will point to its corresponding symbol, so
// lookup via table is no longer required. Each function has its own stack frame, which corresponds
// to a seperate symbol table
pub struct SymbolTable {
    mapping : Vec<Scope>,
    static_frame_ptr : Vec<usize>,
    frame_curr : usize,
    frames : usize,
    size : usize, // Total number of variables defined

}

impl SymbolTable {
    // Create a new symbol table with no parent
    pub fn new() -> Self {
        SymbolTable {
            mapping : Vec::from([Scope::new(0)]), // Initialize with a global scope
            static_frame_ptr : Vec::from([0]),
            frame_curr : 0,
            frames : 1, // Counter of unique frames encountered so far. Used for fid
            size : 0,
        }
    }

    pub fn len(& self) -> usize {
        self.size
    }
    
    // Create a new frame and return its static frameid.
    pub fn push_frame(&mut self) -> usize {
        self.frame_curr = self.frames;
        self.static_frame_ptr.push(self.size);
        self.frames += 1;
        self.frame_curr
    }

    // Pop all scopes belonging to the current static frame, and revert
    pub fn pop_frame(&mut self) -> usize {
        if self.frame_curr == 0 {panic!("Trying to pop global frame (Mismatched push_frame and pop_frame)")};
        while let Some(x) = self.mapping.pop_if(|scope| {scope.frame_id == self.frame_curr}) {
            self.size -= x.map.len();
        }
        let ret = self.frame_curr;
        self.frame_curr = self.mapping.last().expect("No scope (left global scope)").frame_id;
        self.static_frame_ptr.pop();
        ret
    }

    // Create a new scope
    pub fn push_scope(&mut self) {
        self.mapping.push(Scope::new(self.frame_curr))
    }

    // Pop a scope
    pub fn pop_scope(&mut self) -> Option<Scope> {
        let top = self.mapping.pop()?;
        self.size -= top.map.len();
        Some(top)
    }

    // Check if an identifier name corresponds to a variable local to the current scope
    pub fn is_local(&self, name : &str) -> bool {
        self.mapping
            .last()
            .expect("No scope (left global scope)")
            .map.contains_key(name)
    }


    // Return a new SymbolPtr pointing to the symbol currently mapped to the name (If it exists)
    pub fn get_symbol(&self, name : &str) -> Option<SymbolPtr> {
        let mut new_ptr = self.mapping
            .iter()
            .rev()
            .find_map(|scope| {scope.map.get(name).cloned()})?;
        new_ptr.is_local = self.is_local(name);
        Some(new_ptr)
    }

    // Insert into the current scope if it exists
    pub fn insert(&mut self, name : &str, val_type : &Types, is_const : bool) {
        self.mapping.last_mut()
            .expect("No scope (left global scope)")
            .map.insert(name.to_string(), SymbolPtr::new(Symbol { 
                id : Some(name.to_string()),
                // The id of the function that owns the stack frame this var belongs to. 0 if global frame
                frame_id : self.frame_curr, 
                offset : self.size - self.static_frame_ptr.last().expect("No frame (left global frame)"), // offset from current static frame pointer.
                is_const : is_const,
                is_captured : false,
                val_type : val_type.clone()
            }));
        // Increase size afterwards
        self.size += 1;
    }

}
