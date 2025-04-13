use crate::runtime::types::Value;
use crate::error::KedgeError;

/// A simple LIFO stack for WASM values
pub struct ValueStack {
    stack: Vec<Value>,
}

impl ValueStack {
    /// Create a new empty value stack
    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(1024),
        }
    }

    /// Push a value onto the stack
    pub fn push(&mut self, val: Value) {
        self.stack.push(val);
    }

    /// Pop a value off the stack
    pub fn pop(&mut self) -> Result<Value, KedgeError> {
        self.stack.pop().ok_or(KedgeError::StackUnderflow)
    }

    /// Peek at the top value without removing it
    pub fn peek(&self) -> Option<Value> {
        self.stack.last().copied()
    }

    /// Check current stack size
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    /// Whether the stack is empty
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Clear the stack (used between function calls or resets)
    pub fn reset(&mut self) {
        self.stack.clear();
    }
}
