use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinSet;
use wasmtime::component::{Component, Linker, Val};
use wasmtime::{Config, Engine, Store};


mod fake_wasi;

use fake_wasi::*;

#[tokio::main]
async fn main() {
    let mut config = Config::default();

    config.wasm_multi_value(true);
    config.async_support(true);
    config.wasm_component_model(true);
    config.epoch_interruption(true);

    let engine = Arc::new(Engine::new(&config).unwrap());
    let mut linker = Linker::new(&engine);

    wall_clock::add_to_linker(&mut linker, |x| x).unwrap();
    poll::add_to_linker(&mut linker, |x| x).unwrap();
    monotonic_clock::add_to_linker(&mut linker, |x| x).unwrap();
    streams::add_to_linker(&mut linker, |x| x).unwrap();
    filesystem::add_to_linker(&mut linker, |x| x).unwrap();
    environment::add_to_linker(&mut linker, |x| x).unwrap();
    preopens::add_to_linker(&mut linker, |x| x).unwrap();
    exit::add_to_linker(&mut linker, |x| x).unwrap();

    let mut epoch_interval = tokio::time::interval(Duration::from_millis(1));
    let engine_ref: Arc<Engine> = engine.clone();
    let _ = tokio::spawn(async move {
        loop {
            epoch_interval.tick().await;
            engine_ref.increment_epoch();
        }
    });

    let component = Component::from_file(&engine, "../component/target/wasm32-wasi/debug/lib.wasm").unwrap();

    let mut join_set = JoinSet::new();

    let linker_ref = Arc::new(linker);
    let component_ref = Arc::new(component);
    for _ in 0..100 {
        join_set.spawn(run_component(engine.clone(), linker_ref.clone(), component_ref.clone()));
    }
    while let Some(_) = join_set.join_next().await {
        print!(".");
    }
}

async fn run_component(engine: Arc<Engine>, linker: Arc<Linker<WasiCtx>>, component: Arc<Component>) {
    let instance = linker.instantiate_pre(&component).unwrap();

    let ctx = WasiCtx {};
    let mut store = Store::new(&engine, ctx);
    store.epoch_deadline_async_yield_and_update(1);

    let instance = instance.instantiate_async(&mut store).await.unwrap();

    let fun = instance.get_func(&mut store, "run").unwrap();
    let mut results: Vec<Val> = vec![Val::Bool(false)];
    let _ = fun.call_async(&mut store, &[], &mut results).await.unwrap();
    let _ = fun.post_return_async(&mut store).await.unwrap();

    println!("{:?}", results);
}
