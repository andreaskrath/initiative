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

    debug!("launching initiative application with settings:");
    debug!("               id: {:?}", settings.id);
    debug!("     antialiasing: {}", settings.antialiasing);
    debug!("            vsync: {}", settings.vsync);
    debug!("     default font: {:?}", settings.default_font.family);
    debug!("default text size: {} pixels", settings.default_text_size.0);
    debug!("     loaded fonts: {}", settings.fonts.len());

    application(Initiative::default, Initiative::update, Initiative::view)
        .settings(settings)
        .subscription(Initiative::subscription)
        .theme(Initiative::theme)
        .run()
}
