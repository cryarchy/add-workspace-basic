# add-workspace-basic

This is a basic example that shows how to run a function exported from a WebAssembly component via an interface using wasmtime.

This Rust workspace contains two packages:

-   add:
    -   defines a function named `add`,
    -   is meant to be compiled into a wasm component.
-   add-runner:
    -   contains the host application's code which invokes the `add` function exported from the wasm component using wasmtime.

To run this example:

-   Run the following command fromt the `add` folder:
    `cargo component build --release`
-   From the workspace root folder run:
    `cargo run --bin add-runner`
