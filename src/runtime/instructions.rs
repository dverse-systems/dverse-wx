use crate::runtime::stack::ValueStack;
use crate::runtime::types::{Opcode, Value};
use crate::error::KedgeError;

/// Executes a single WASM opcode using the given value stack
pub fn execute_instruction(op: Opcode, stack: &mut ValueStack) -> Result<(), KedgeError> {
    match op {
        Opcode::Nop => Ok(()),

        Opcode::I32Const(val) => {
            stack.push(Value::I32(val));
            Ok(())
        }

        Opcode::I32Add => {
            let b = stack.pop()?.as_i32();
            let a = stack.pop()?.as_i32();
            stack.push(Value::I32(a + b));
            Ok(())
        }

        Opcode::I32Sub => {
            let b = stack.pop()?.as_i32();
            let a = stack.pop()?.as_i32();
            stack.push(Value::I32(a - b));
            Ok(())
        }

        Opcode::I32Mul => {
            let b = stack.pop()?.as_i32();
            let a = stack.pop()?.as_i32();
            stack.push(Value::I32(a * b));
            Ok(())
        }

        Opcode::I32DivS => {
            let b = stack.pop()?.as_i32();
            if b == 0 {
                return Err(KedgeError::RuntimeError("division by zero"));
            }
            let a = stack.pop()?.as_i32();
            stack.push(Value::I32(a / b));
            Ok(())
        }

        Opcode::Unreachable => Err(KedgeError::RuntimeError("unreachable instruction")),

        Opcode::End | Opcode::Return => Ok(()), // control flow handled at higher level

        _ => Err(KedgeError::InvalidInstruction(opcode_to_byte(&op))),
    }
}

/// Helper function to convert known opcodes back to byte (for debug/errors)
pub fn opcode_to_byte(op: &Opcode) -> u8 {
    match op {
        Opcode::Nop => 0x01,
        Opcode::Unreachable => 0x00,
        Opcode::I32Const(_) => 0x41,
        Opcode::I32Add => 0x6A,
        Opcode::I32Sub => 0x6B,
        Opcode::I32Mul => 0x6C,
        Opcode::I32DivS => 0x6D,
        Opcode::GetLocal(_) => 0x20,
        Opcode::SetLocal(_) => 0x21,
        Opcode::Call(_) => 0x10,
        Opcode::End => 0x0B,
        Opcode::Return => 0x0F,
    }
}
