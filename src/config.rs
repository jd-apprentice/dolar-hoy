use std::env;

/// # Panics
/// Will panic if `key` is not set in the .env file
pub fn get_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("Missing {} environment variable", key))
}

/// # Panics
/// Will panic if .env file is not found
pub fn load_env() {
    dotenvy::dotenv().unwrap_or_else(|_| {
        panic!("Missing .env file");
    });
}

/// # Panics
/// Will panic if `SENTRY_DSN` is not set in the .env file
#[inline]
pub fn load_sentry() {
    load_env();

    let _guard = sentry::init((
        get_env("SENTRY_DSN"),
        sentry::ClientOptions {
            debug: false,
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    sentry::configure_scope(|scope| {
        scope.set_tag("app", "dolar_hoy");
        scope.set_tag("version", env!("CARGO_PKG_VERSION"));
    });
}
