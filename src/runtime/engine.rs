use crate::runtime::types::{Opcode, Value};
use crate::runtime::stack::ValueStack;
use crate::runtime::instructions::execute_instruction;
use crate::runtime::module::{Module, Function};
use crate::decode::reader::Reader;
use crate::error::KedgeError;
use crate::decode::consts::*;

/// The WASM execution engine — runs code from parsed modules
pub struct Engine {
    pub stack: ValueStack,
}

impl Engine {
    /// Create a new engine with an empty stack
    pub fn new() -> Self {
        Self {
            stack: ValueStack::new(),
        }
    }

    /// Run the first exported function (e.g., func[0])
    pub fn run(&mut self, module: &Module) -> Result<Option<Value>, KedgeError> {
        let func = module.get_function(0).ok_or(KedgeError::RuntimeError("Missing function[0]"))?;

        let mut reader = Reader::new(&func.body);

        // Parse local decls (ignored for now)
        let local_count = reader.read_u32_leb128()? as usize;
        for _ in 0..local_count {
            let _n = reader.read_u32_leb128()?; // count of locals
            let _valtype = reader.read_u8()?;   // local type (e.g., 0x7F = i32)
            // In the future: allocate locals
        }

        // Read instructions and execute until End
        loop {
            if reader.is_done() {
                break;
            }

            let byte = reader.read_u8()?;

            let opcode = match byte {
                OPCODE_I32_CONST => {
                    let val = reader.read_i32_leb128()?;
                    Opcode::I32Const(val)
                }
                OPCODE_I32_ADD => Opcode::I32Add,
                OPCODE_I32_SUB => Opcode::I32Sub,
                OPCODE_I32_MUL => Opcode::I32Mul,
                OPCODE_I32_DIV_S => Opcode::I32DivS,
                OPCODE_NOP => Opcode::Nop,
                OPCODE_UNREACHABLE => Opcode::Unreachable,
                OPCODE_RETURN => Opcode::Return,
                OPCODE_END => break, // End of function
                other => return Err(KedgeError::InvalidInstruction(other)),
            };

            execute_instruction(opcode, &mut self.stack)?;
        }

        Ok(self.stack.peek())
    }

    /// Resets the stack
    pub fn reset(&mut self) {
        self.stack.reset();
    }
}
