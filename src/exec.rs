use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::decode::module_decoder::parse_module;
use crate::runtime::engine::Engine;
use crate::error::KedgeError;

/// Run a .wasm file and return the result (if any)
pub fn run_file<P: AsRef<Path>>(path: P) -> Result<(), KedgeError> {
    let mut file = File::open(&path).map_err(|_| KedgeError::DecodeError("Cannot open file"))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|_| KedgeError::DecodeError("Cannot read file"))?;

    let module = parse_module(&buffer)?;
    let mut engine = Engine::new();
    let result = engine.run(&module)?;

    println!("Executed {} -> {:?}", path.as_ref().display(), result);
    Ok(())
}
