use email_newsletter::configuration::get_configuration;
use email_newsletter::startup::Application;
use email_newsletter::telemetry::{get_subscriber, init_subscriber};

// TODO: Host a Jaeger instance on AWS and use OpenTelemetry with this project
// TODO: Use tracing-error for better tracing/rust error integration

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("email_newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
