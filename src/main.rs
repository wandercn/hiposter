mod model;
mod http;

use gpui::*;
use gpui_platform::application;

struct Hiposter {
    request: model::HttpRequest,
    response: Option<model::HttpResponse>,
    loading: bool,
}

impl Render for Hiposter {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e3440))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xd8dee9))
            .child("Hiposter GPUI")
    }
}

fn main() {
    application().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| Hiposter {
                request: model::HttpRequest::default(),
                response: None,
                loading: false,
            })
        })
        .unwrap();
    });
}
