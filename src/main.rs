use dolar_hoy::{create_pool, load_sentry};
use sentry::{capture_message, ClientInitGuard};

fn main() {
    let _sentry_guard: ClientInitGuard = load_sentry();

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            create_pool()
                .await
                .unwrap_or_else(|_| panic!("Failed to connect to database"));
            capture_message("Dolar Hoy API started", sentry::Level::Info);
        });
}
