use std::error::Error;

use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};

fn main() -> Result<(), Box<dyn Error>> {
    let bytes = include_bytes!("../component.wasm");

    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let mut linker = Linker::new(&engine);
    let mut store = Store::new(&engine, Foo {});

    wasm::Myapp::add_to_linker(&mut linker, |i| i)?;

    let component = Component::from_binary(&engine, bytes)?;

    let (my_app, _) = wasm::Myapp::instantiate(&mut store, &component, &linker).unwrap();

    my_app
        .call_do_the_thing(&mut store, wasm::Foo { bar: 123, baz: 321 })
        .unwrap();

    Ok(())
}

struct Foo {}

impl wasm::MyappImports for Foo {
    fn some_host_func(&mut self, s: String) -> wasmtime::Result<()> {
        println!("WASM called a host function: {s}");
        Ok(())
    }
}

mod wasm {
    wasmtime::component::bindgen!("myapp");
}
