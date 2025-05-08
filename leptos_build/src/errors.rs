use thiserror::Error;

#[derive(Error, Debug)]
pub enum LeptosBuildError{
    #[error("wasi plugin could not be instantiated")]
   WasmtimeError(#[from] wasmtime::Error),
}