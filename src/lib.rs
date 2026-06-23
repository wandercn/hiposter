#![recursion_limit = "256"]

pub mod model;
pub mod http;
pub mod theme;
pub mod assets;
pub mod api_tab;
pub mod app;
pub mod about;

use gpui::*;
use assets::AppAssets;
use app::Hiposter;
use about::AboutWindow;
use serde_json::Value;
use serde::Serialize;

pub fn format_json(value: &Value) -> String {
    let mut buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    if value.serialize(&mut ser).is_ok() {
        String::from_utf8(buf).unwrap_or_default()
    } else {
        serde_json::to_string_pretty(value).unwrap_or_default()
    }
}

actions!(hiposter, [OpenAbout, Quit]);

fn build_mac_menus() -> Vec<Menu> {
    vec![
        Menu {
            name: "HiPoster".into(),
            items: vec![
                MenuItem::action("About HiPoster", OpenAbout),
                MenuItem::separator(),
                MenuItem::action("Quit HiPoster", Quit),
            ],
            disabled: false,
        },
        Menu {
            name: "Edit".into(),
            items: vec![
                MenuItem::action("Undo", gpui_component::input::Undo),
                MenuItem::action("Redo", gpui_component::input::Redo),
                MenuItem::separator(),
                MenuItem::action("Cut", gpui_component::input::Cut),
                MenuItem::action("Copy", gpui_component::input::Copy),
                MenuItem::action("Paste", gpui_component::input::Paste),
                MenuItem::separator(),
                MenuItem::action("Select All", gpui_component::input::SelectAll),
            ],
            disabled: false,
        },
    ]
}

pub fn open_about_window(cx: &mut App) {
    let options = WindowOptions {
        window_bounds: Some(WindowBounds::centered(size(px(400.), px(300.)), cx)),
        titlebar: Some(gpui_component::TitleBar::title_bar_options()),
        window_decorations: Some(WindowDecorations::Server),
        kind: WindowKind::Normal,
        is_movable: true,
        ..Default::default()
    };
    
    cx.open_window(options, |window, cx| {
        let view = cx.new(|cx| AboutWindow::new(cx));
        cx.new(|cx| gpui_component::Root::new(view, window, cx))
    }).ok();
}

pub fn run() {
    let _runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to build tokio runtime");
    let _guard = _runtime.enter();

    let app = gpui_platform::application().with_assets(AppAssets);

    app.run(move |cx| {
        gpui_component::init(cx);

        cx.set_menus(build_mac_menus());
        
        cx.on_action(|_: &OpenAbout, cx: &mut App| {
            open_about_window(cx);
        });
        
        cx.on_action(|_: &Quit, cx: &mut App| {
            cx.quit();
        });
        
        cx.bind_keys([
            KeyBinding::new("cmd-q", Quit, None),
            KeyBinding::new("ctrl-q", Quit, None),
            KeyBinding::new("alt-f4", Quit, None),
        ]);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(1400.), px(900.)), cx)),
            titlebar: Some(gpui_component::TitleBar::title_bar_options()),
            ..Default::default()
        };

        cx.spawn(async move |cx| {
            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|cx| Hiposter::new(window, cx));
                cx.new(|cx| gpui_component::Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}


