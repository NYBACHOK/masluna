use std::{collections::HashMap, path::PathBuf};

use javy_codegen::{Generator, JS};

pub use javy_codegen::{LinkingKind, Plugin, SourceEmbedding, WitOptions};

/// A collection of property names to whether they are enabled.
#[derive(Clone, Debug, Default)]
pub struct JsConfig(HashMap<String, bool>);

impl JsConfig {
    /// Create from a hash.
    pub fn from_hash(configs: impl IntoIterator<Item = (String, bool)>) -> Self {
        JsConfig(configs.into_iter().collect())
    }

    /// Encode as JSON.
    pub fn to_json(&self) -> serde_json::Result<Vec<u8>> {
        Ok(serde_json::to_vec(&self.0)?)
    }
}

#[derive(Debug)]
pub struct CompileOptions {
    /// Path of the JavaScript input file
    pub input: PathBuf,
    /// A Javy plugin.
    pub plugin: Plugin,
    /// Source code embedding options for the generated Wasm module.
    pub source: SourceEmbedding,
    pub wit: WitOptions,
    pub js_config: JsConfig,
    pub linking: LinkingKind,
}

pub fn compile(
    CompileOptions {
        input,
        plugin,
        source,
        wit,
        js_config,
        linking,
    }: CompileOptions,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let js = JS::from_file(&input)?;

    let mut generator = Generator::new(plugin);

    // Configure the generator with the provided options.
    generator
        .wit_opts(wit)
        .js_runtime_config(js_config.to_json()?)
        .source_embedding(source)
        .producer_version(env!("CARGO_PKG_VERSION").to_string())
        .linking(linking);

    let wasm = generator.generate(&js)?;

    Ok(wasm)
}
