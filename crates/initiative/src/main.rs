use app::Initiative;
use iced::application;

fn main() -> iced::Result {
    logging::init();

    application("Initiative", Initiative::update, Initiative::view).run()
}
