use myhealth::configuration::get_configuration;
use myhealth::routes::Application;
use myhealth::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("myhealth".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    
    application.run_until_stopped().await?;
    Ok(())
}