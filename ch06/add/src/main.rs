use log::{info, debug, trace};
use std::error::Error;
use std::fs::File;
use wasmi::{Engine, Extern, Linker, Module, Store};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let engine = Engine::default();
    trace!("{engine:?}");

    // load the wasm module.
    let f = File::open("./add.wasm")?;
    let module = Module::new(&engine, f)?;
    trace!("{module:?}");

    // prepare the unit type host store.
    let mut store = Store::new(&engine, &());
    trace!("{store:?}");

    // prepare the linker.
    let linker = Linker::<()>::new();
    trace!("{linker:?}");

    // instanciate the module.
    let instance = linker.instantiate(&mut store, &module)?.start(&mut store)?;
    trace!("{instance:?}");

    // get the add export function.
    let add = instance
        .get_export(&store, "add")
        .and_then(Extern::into_func)
        .ok_or_else(|| Box::<dyn Error>::from("migging add export"))?
        .typed::<(i32, i32), i32>(&mut store)?;
    debug!("{add:?}");

    // call it!
    let args = (42, 1);
    let result = add.call(&mut store, args)?;
    info!("add{args:?} = {result}");
    assert_eq!(result, args.0 + args.1);

    Ok(())
}
