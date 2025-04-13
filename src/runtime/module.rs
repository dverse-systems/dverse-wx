use crate::runtime::types::{FuncType, Value};

/// Represents a compiled & validated function
pub struct Function {
    pub type_index: usize,     // Index into the module’s types[]
    pub locals: Vec<Value>,    // Local variables initialized
    pub body: Vec<u8>,         // Raw bytecode to interpret
}

/// A WASM module in its decoded and loaded state
pub struct Module {
    pub types: Vec<FuncType>,         // All func type signatures (params/results)
    pub functions: Vec<Function>,     // Concrete function definitions (index = func index)
    pub start_function: Option<usize> // Optional start function
}

impl Module {
    pub fn new() -> Self {
        Self {
            types: Vec::new(),
            functions: Vec::new(),
            start_function: None,
        }
    }

    pub fn add_function(&mut self, type_index: usize, body: Vec<u8>) {
        self.functions.push(Function {
            type_index,
            locals: Vec::new(), // can be extended later
            body,
        });
    }

    pub fn get_function(&self, index: usize) -> Option<&Function> {
        self.functions.get(index)
    }

    pub fn get_func_type(&self, index: usize) -> Option<&FuncType> {
        self.types.get(index)
    }
}
