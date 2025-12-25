use app::Initiative;
use iced::application;
use tracing::info;

fn main() -> iced::Result {
    logging::init();

    info!("launching initiative application");

    application(Initiative::default, Initiative::update, Initiative::view)
        .subscription(Initiative::subscription)
        .theme(Initiative::theme)
        .run()
}
