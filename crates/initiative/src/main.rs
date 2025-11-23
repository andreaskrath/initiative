use app::Initiative;
use iced::application;
use tracing::info;

fn main() -> iced::Result {
    logging::init();

    info!("launching initiative application");

    application("Initiative", Initiative::update, Initiative::view).run()
}
