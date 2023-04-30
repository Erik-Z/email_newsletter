use email_newsletter::configuration::get_configuration;
use email_newsletter::startup::run;
use email_newsletter::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;

// TODO: Host a Jaeger instance on AWS and use OpenTelemetry with this project
// TODO: Use tracing-error for better tracing/rust error integration

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("email_newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    // let connection_pool = PgPool::connect_lazy(&connection_string.expose_secret())
    //     .expect("Failed to connect to Postgres.");
    // let connection_pool = PgPoolOptions::new()
    //     .connect_timeout(std::time::Duration::from_secs(2))
    //     .connect_lazy(&configuration.database.connection_string());
    let connection_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    run(listener, connection_pool)?.await
}
