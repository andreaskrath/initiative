use app::Initiative;
use iced::{Settings, application};
use tracing::debug;

fn main() -> iced::Result {
    logging::init();
    debug!("setup logging successfully");

    debug!("loading embedded fonts");
    let fonts = assets::fonts::get();

    let settings = Settings {
        id: Some(String::from("initiative")),
        fonts,
        ..Default::default()
    };

    debug!("launching initiative application with settings: {settings:?}");

    application(Initiative::default, Initiative::update, Initiative::view)
        .settings(settings)
        .subscription(Initiative::subscription)
        .theme(Initiative::theme)
        .run()
}
