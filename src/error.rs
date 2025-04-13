// A minimal, crate-free custom error system for kedge-wx

#[derive(Debug)]
pub enum KedgeError {
    DecodeError(&'static str),
    RuntimeError(&'static str),
    MemoryViolation,
    StackUnderflow,
    InvalidInstruction(u8),
    HostcallError(&'static str),
    UnsupportedFeature(&'static str),
}

impl core::fmt::Display for KedgeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            KedgeError::DecodeError(msg) => write!(f, "Decode error: {}", msg),
            KedgeError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
            KedgeError::MemoryViolation => write!(f, "Memory access violation"),
            KedgeError::StackUnderflow => write!(f, "Stack underflow"),
            KedgeError::InvalidInstruction(op) => write!(f, "Invalid instruction: 0x{:X}", op),
            KedgeError::HostcallError(msg) => write!(f, "Hostcall failed: {}", msg),
            KedgeError::UnsupportedFeature(msg) => write!(f, "Unsupported: {}", msg),
        }
    }
}

impl std::error::Error for KedgeError {}
