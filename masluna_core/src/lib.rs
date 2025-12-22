use std::path::{Path, PathBuf};

use wasmer::{
    Imports, Instance, Module, Store,
    sys::{CompilerConfig, EngineBuilder},
};

pub use wasmer::{CompileError, InstantiationError};

#[derive(Debug, thiserror::Error)]
#[error("Failed IO during {context} for {path}. Reason: {error}")]
pub struct BetterIoError {
    pub context: &'static str,
    pub path: PathBuf,
    pub error: std::io::Error,
}

#[derive(Debug, thiserror::Error)]
pub enum WasmModuleFromFileError {
    #[error(transparent)]
    Io(#[from] BetterIoError),
    #[error(transparent)]
    Compile(#[from] wasmer::CompileError),
}

// TODO: add LLVM compiler for release
#[inline]
fn compiler_config() -> impl CompilerConfig {
    wasmer::sys::Cranelift::default()
}

#[derive(Debug)]
pub struct WasmModule {
    pub store: Store,
    pub module: Module,
}

impl WasmModule {
    pub fn new(wasm_bytes: impl AsRef<[u8]>) -> Result<Self, wasmer::CompileError> {
        let engine = EngineBuilder::new(compiler_config());

        let store = Store::new(engine);

        let module = Module::new(&store, wasm_bytes)?;

        Ok(Self { store, module })
    }

    pub async fn from_file(path: impl AsRef<Path>) -> Result<Self, WasmModuleFromFileError> {
        let file_content = async_fs::read(&path).await.map_err(|error| BetterIoError {
            context: "reading wasm file content",
            path: path.as_ref().to_path_buf(),
            error,
        })?;

        Self::new(file_content).map_err(Into::into)
    }

    pub fn build_instance(
        &mut self,
        imports: Option<&Imports>,
    ) -> Result<Instance, wasmer::InstantiationError> {
        let imports = match imports {
            Some(imports) => imports,
            None => &wasmer::imports! {},
        };

        Instance::new(&mut self.store, &self.module, imports)
    }
}
