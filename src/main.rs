use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // printing all logs at info-level or above,
    // unless the RUST_LOG environmental
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // panic if we cannot get a config file for the database
    let configuration = get_configuration().expect("Failed to read configuration file");

    // setting up database connection
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    // address coming from config file
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind to random port.");
    run(listener, connection_pool)?.await
}
