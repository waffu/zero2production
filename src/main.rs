use secrecy::ExposeSecret;
use sqlx::{postgres::PgPoolOptions};
use std::net::TcpListener;
use zero2prod::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);

    init_subscriber(subscriber);

    let configuration = get_configuration().expect("failed to read configuration");

    let connection_pool = PgPoolOptions::new()
        .idle_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(&configuration.database.connection_string().expose_secret()).expect("failed to connect to postgres");

    //let connection_pool =
        //PgPool::connect_lazy(configuration.database.connection_string().expose_secret())
            //.expect("failed to connect to postgres");
    let address = format!("0.0.0.0:{}", configuration.application_port);

    let listener = TcpListener::bind(address).expect("failed to bind randomly assigned port");

    run(listener, connection_pool)?.await
}
