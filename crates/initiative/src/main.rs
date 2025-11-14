use assets::Assets;
use gpui::{App, AppContext, Application, WindowOptions};
use gpui_component::Root;
use parking_lot::Mutex;
use state::AppState;
use tracing::debug;
use views::RootView;

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        logging::init();
        gpui_component::init(cx);
        load_embedded_fonts(cx);

        cx.set_global(AppState::new());

        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions {
                    app_id: Some(String::from("Initiative")),
                    ..Default::default()
                },
                |window, cx| {
                    let root_view = cx.new(RootView::new);

                    cx.new(|cx| Root::new(root_view.into(), window, cx))
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}

fn load_embedded_fonts(cx: &App) {
    let asset_source = cx.asset_source();
    let font_paths = asset_source
        .list("fonts")
        .expect("fonts should always exist");
    let embedded_fonts = Mutex::new(Vec::new());

    let executor = cx.background_executor();

    let scoped_executor = executor.scoped(|scope| {
        for font_path in &font_paths {
            if !font_path.ends_with(".ttf") {
                continue;
            }

            scope.spawn(async {
                let font_bytes = asset_source
                    .load(font_path)
                    .expect("load should always return Ok")
                    .expect("Ok should always contain a font");
                embedded_fonts.lock().push(font_bytes);
            });
        }
    });

    executor.block(scoped_executor);

    cx.text_system()
        .add_fonts(embedded_fonts.into_inner())
        .expect("fonts should always be valid");

    debug!(fonts = ?&cx.text_system().all_font_names());
}
