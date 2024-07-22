use anyhow::Result;
use wasmtime::component::{Component, Linker, Val};
use wasmtime::{Engine, Store};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxBuilder, WasiView};

fn main() -> Result<()> {
    // Create an engine and store
    let engine = Engine::default();
    let wasi = WasiCtxBuilder::new().inherit_stdio().build();
    let mut store = Store::new(&engine, ServerWasiView::new(wasi));

    // Load the component
    let component = Component::from_file(&engine, "target/wasm32-wasip1/release/add.wasm")?;

    // Create a linker
    let mut linker = Linker::<ServerWasiView>::new(&engine);

    // Add WASI to the linker
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;

    // Instantiate the component
    let instance = linker.instantiate(&mut store, &component)?;

    // Try to get the add function with different possible names
    let add = instance
        .exports(&mut store)
        .instance("docs:adder/add@0.1.0")
        .expect("instance 'docs:adder/add@0.1.0' not found")
        .func("add")
        .expect("add function not found");

    // Prepare the arguments and results
    let mut results = [Val::S32(0)];

    // Call the function
    add.call(&mut store, &[Val::S32(5), Val::S32(3)], &mut results)?;

    // Extract the result
    let Val::S32(result) = results[0] else {
        panic!("Unexpected result type");
    };

    println!("5 + 3 = {}", result);

    Ok(())
}

struct ServerWasiView {
    table: ResourceTable,
    ctx: WasiCtx,
}

impl ServerWasiView {
    fn new(ctx: WasiCtx) -> Self {
        let table = ResourceTable::new();
        Self { table, ctx }
    }
}

impl WasiView for ServerWasiView {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
