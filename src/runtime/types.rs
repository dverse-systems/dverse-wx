/// Core types used in the Kedge-WX runtime
/// Minimal WASM 1.0 support (i32, i64)

/// Value types supported by WASM
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValType {
    I32,
    I64,
    // Add more (F32, F64) as needed
}

/// Runtime values passed on the stack
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    I32(i32),
    I64(i64),
}

impl Value {
    pub fn as_i32(&self) -> i32 {
        match self {
            Value::I32(v) => *v,
            _ => panic!("Expected i32 value"),
        }
    }

    pub fn as_i64(&self) -> i64 {
        match self {
            Value::I64(v) => *v,
            _ => panic!("Expected i64 value"),
        }
    }
}

/// Function type signature
#[derive(Debug, Clone, PartialEq)]
pub struct FuncType {
    pub params: Vec<ValType>,
    pub results: Vec<ValType>,
}

impl FuncType {
    pub fn new(params: Vec<ValType>, results: Vec<ValType>) -> Self {
        Self { params, results }
    }
}

/// Value type for locals and globals
#[derive(Debug, Clone)]
pub struct Local {
    pub val_type: ValType,
    pub value: Value,
}

/// Instructions supported (expandable)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    Nop,
    Unreachable,
    I32Const(i32),
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    GetLocal(u32),
    SetLocal(u32),
    Call(u32),
    End,
    Return,
}
