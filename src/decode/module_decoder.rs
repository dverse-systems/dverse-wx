use crate::decode::reader::Reader;
use crate::decode::consts::*;
use crate::error::KedgeError;
use crate::runtime::types::{FuncType, ValType};
use crate::runtime::module::Module;

/// Parses a binary `.wasm` module into a structured in-memory `Module`
pub fn parse_module(bytes: &[u8]) -> Result<Module, KedgeError> {
    let mut reader = Reader::new(bytes);
    let mut module = Module::new();

    // Check magic number and version
    if reader.read_bytes(4)? != [0x00, 0x61, 0x73, 0x6D] {
        return Err(KedgeError::DecodeError("Invalid WASM magic header"));
    }

    if reader.read_bytes(4)? != [0x01, 0x00, 0x00, 0x00] {
        return Err(KedgeError::DecodeError("Unsupported WASM version"));
    }

    let mut func_type_indices: Vec<usize> = Vec::new();
    let mut raw_code_bodies: Vec<Vec<u8>> = Vec::new();

    // Parse all sections
    while !reader.is_done() {
        let section_id = reader.read_u8()?;
        let section_size = reader.read_u32_leb128()? as usize;
        let section_data = reader.read_bytes(section_size)?;

        match section_id {
            SECTION_TYPE => parse_type_section(section_data, &mut module)?,
            SECTION_FUNCTION => {
                func_type_indices = parse_function_section(section_data)?;
            }
            SECTION_CODE => {
                raw_code_bodies = parse_code_section(section_data)?;
            }
            _ => {
                // Skip unsupported sections
                continue;
            }
        }
    }

    // Link function bodies to types
    if func_type_indices.len() != raw_code_bodies.len() {
        return Err(KedgeError::DecodeError("Mismatch between function and code sections"));
    }

    for (i, body) in raw_code_bodies.into_iter().enumerate() {
        let type_index = func_type_indices[i];
        module.add_function(type_index, body);
    }

    Ok(module)
}

/// Parses the type section and adds `FuncType`s to the module
fn parse_type_section(data: &[u8], module: &mut Module) -> Result<(), KedgeError> {
    let mut r = Reader::new(data);
    let count = r.read_u32_leb128()? as usize;

    for _ in 0..count {
        let form = r.read_u8()?;
        if form != TYPE_FUNC {
            return Err(KedgeError::DecodeError("Expected function type (0x60)"));
        }

        let param_count = r.read_u32_leb128()? as usize;
        let mut params = Vec::with_capacity(param_count);
        for _ in 0..param_count {
            let val_type = parse_valtype(r.read_u8()?)?;
            params.push(val_type);
        }

        let result_count = r.read_u32_leb128()? as usize;
        let mut results = Vec::with_capacity(result_count);
        for _ in 0..result_count {
            let val_type = parse_valtype(r.read_u8()?)?;
            results.push(val_type);
        }

        module.types.push(FuncType::new(params, results));
    }

    Ok(())
}

/// Parses the function section to collect type indices for each function
fn parse_function_section(data: &[u8]) -> Result<Vec<usize>, KedgeError> {
    let mut r = Reader::new(data);
    let count = r.read_u32_leb128()? as usize;
    let mut type_indices = Vec::with_capacity(count);

    for _ in 0..count {
        let index = r.read_u32_leb128()? as usize;
        type_indices.push(index);
    }

    Ok(type_indices)
}

/// Parses the code section to extract function bodies
fn parse_code_section(data: &[u8]) -> Result<Vec<Vec<u8>>, KedgeError> {
    let mut r = Reader::new(data);
    let count = r.read_u32_leb128()? as usize;
    let mut bodies = Vec::with_capacity(count);

    for _ in 0..count {
        let body_size = r.read_u32_leb128()? as usize;
        let body = r.read_bytes(body_size)?.to_vec();
        bodies.push(body);
    }

    Ok(bodies)
}

/// Parses a single byte into a WASM `ValType`
fn parse_valtype(byte: u8) -> Result<ValType, KedgeError> {
    match byte {
        TYPE_I32 => Ok(ValType::I32),
        TYPE_I64 => Ok(ValType::I64),
        _ => Err(KedgeError::DecodeError("Unsupported value type")),
    }
}
