use dolar_hoy::{create_pool, load_sentry};
use sentry::capture_message;

#[tokio::main]
async fn main() {
    load_sentry();
    let pool = create_pool();

    match pool.await {
        Ok(pool) => {
            capture_message("Dolar Hoy API started", sentry::Level::Info);
            println!("Pool created: {:?}", pool);
        }
        Err(e) => {
            println!("Error creating pool: {}", e);
            panic!("Error creating pool: {}", e);
        }
    }
}
