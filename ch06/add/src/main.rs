use std::fs::File;
use std::error::Error;
use wasmi::{Engine, Module};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("./add.wasm")?;
    let engine = Engine::default();
    let module = Module::new(&engine, f)?;
    println!("{module:?}");
    Ok(())
}
