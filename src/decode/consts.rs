// ────────────────
// Section IDs (WASM Spec)
// ────────────────

pub const SECTION_CUSTOM: u8 = 0;
pub const SECTION_TYPE: u8 = 1;
pub const SECTION_IMPORT: u8 = 2;
pub const SECTION_FUNCTION: u8 = 3;
pub const SECTION_TABLE: u8 = 4;
pub const SECTION_MEMORY: u8 = 5;
pub const SECTION_GLOBAL: u8 = 6;
pub const SECTION_EXPORT: u8 = 7;
pub const SECTION_START: u8 = 8;
pub const SECTION_ELEMENT: u8 = 9;
pub const SECTION_CODE: u8 = 10;
pub const SECTION_DATA: u8 = 11;

// ────────────────
// Value Type Tags (signed LEB128)
// ────────────────

pub const TYPE_I32: u8 = 0x7F;
pub const TYPE_I64: u8 = 0x7E;
pub const TYPE_F32: u8 = 0x7D;
pub const TYPE_F64: u8 = 0x7C;
pub const TYPE_FUNC: u8 = 0x60;

// ────────────────
// Opcode Bytes
// ────────────────

pub const OPCODE_UNREACHABLE: u8 = 0x00;
pub const OPCODE_NOP: u8 = 0x01;
pub const OPCODE_RETURN: u8 = 0x0F;

pub const OPCODE_GET_LOCAL: u8 = 0x20;
pub const OPCODE_SET_LOCAL: u8 = 0x21;
pub const OPCODE_CALL: u8 = 0x10;

pub const OPCODE_I32_CONST: u8 = 0x41;
pub const OPCODE_I32_ADD: u8 = 0x6A;
pub const OPCODE_I32_SUB: u8 = 0x6B;
pub const OPCODE_I32_MUL: u8 = 0x6C;
pub const OPCODE_I32_DIV_S: u8 = 0x6D;

pub const OPCODE_END: u8 = 0x0B;
