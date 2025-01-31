use dolar_hoy::{create_pool, get_values, load_sentry, parse_document, URL};
use sentry::{capture_message, ClientInitGuard};

fn main() {
    let _sentry_guard: ClientInitGuard = load_sentry();

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let document = parse_document(URL).await;
            create_pool()
                .await
                .unwrap_or_else(|_| panic!("Failed to connect to database"));
            capture_message("Dolar Hoy API started", sentry::Level::Info);
            get_values(&document.unwrap()).unwrap_or_else(|_| {
                capture_message("Failed to get values", sentry::Level::Error);
            });
        });
}
