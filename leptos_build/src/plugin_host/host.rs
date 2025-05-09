use std::collections::HashMap;

use camino::Utf8PathBuf;
use wasmtime::{Engine, Result, Store, Config};
use wasmtime::component::{ResourceTable, Linker, bindgen, Component};
use wasmtime_wasi::{IoView, WasiCtx, WasiCtxBuilder, WasiView};
use eyre::Result as EyreResult;
use plugin_api::Plugin;

use crate::errors::LeptosBuildError;

pub fn generate_wasi_engine()-> Result<Engine>{
    let mut config = Config::new();
    config.async_support(true);
    config.wasm_component_model(true);
    config.debug_info(true);
    Engine::new(&config)

}
/// Plugin wrapper that contains additional info the host needs
pub struct PluginWrapper{
    plugin: Plugin,
    path: Utf8PathBuf,
}

/// Global storage for wasmtime constants, plugins, and components
pub struct PluginHost{
    linker: Linker<PluginHostState>,
    store: Store<PluginHostState>,
    plugins:HashMap<String, PluginWrapper>,
    components: HashMap<String,Component>,


}
/// Should be able to call plugins like this
/// ```rust
/// //Here our `greet` function takes one name parameter,
/// //but in the Wasmtime embedding API the first argument is always a `Store`.
/// let greeting = bindings.call_greeting(&mut store, "Ben").await?;
/// ``````
pub async fn setup_plugin_host(engine: &Engine)-> EyreResult<PluginHost>{
    bindgen!({world: "leptos-build-plugin", path: "../plugin_api/wit/leptos-build-plugin.wit", async: true});

    let mut linker = Linker::new(&engine);

    // Add all the WASI extensions to the linker
    wasmtime_wasi::add_to_linker_async(&mut linker).map_err(|e| LeptosBuildError::WasmtimeError(e))?;

    // ... configure `builder` more to add env vars, args, etc ...
    let mut builder = WasiCtxBuilder::new();
    builder.inherit_stdio();
    let store = Store::new(
        &engine,
        PluginHostState {
            ctx: builder.build(),
            table: ResourceTable::new(),
        },
    );
    // TODO: Replace component here with some form of discovery
    let component = Component::from_file(&engine, "./plugins/custom_plugin.wasm").map_err(|e| LeptosBuildError::WasmtimeError(e))?;
    // TODO: This will have to be done in the main because I can't pass bindings
    //let bindings = LeptosBuildPlugin::instantiate_async(&mut store, &component, &linker).await;

    Ok(PluginHost { linker, store, components: Default::default(), plugins: Default::default() })
}


struct PluginHostState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl IoView for PluginHostState {
    fn table(&mut self) -> &mut ResourceTable { &mut self.table }
}
impl WasiView for PluginHostState {
    fn ctx(&mut self) -> &mut WasiCtx { &mut self.ctx }
}

// Scan a folder for plugins and add them to a list for clap matching
async fn load_plugins(plugins_dir: &Utf8PathBuf) -> Result<()>{
    //let loaded_plugins = HashMap::new();

    // for entry in std::fs::read_dir(plugins_dir).unwrap() {
    //     let path = entry.unwrap().path();
    //     if path.extension().map(|e| e == "so").unwrap_or(false) {
    //         if let Ok(lp) = load_plugin(&path) {
    //             app = app.subcommand((lp.plugin.clap_command)());
    //             loaded_plugins.push(lp);
    //         }
    //     }
    //}
    Ok(())
}