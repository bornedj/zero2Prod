use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // panic if we cannot get a config file for the database
    let configuration = get_configuration().expect("Failed to read configuration file");
    // address coming from config file
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind to random port.");
    run(listener)?.await
}
