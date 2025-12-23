use std::sync::LazyLock;

pub mod commands;

pub static TOKIO_RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Critical error. Failed to start tokio runtime")
});
