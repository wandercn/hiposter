mod model;
mod http;

use gpui::*;
use gpui_platform::application;

struct Hiposter {
    request: model::HttpRequest,
    response: Option<model::HttpResponse>,
    loading: bool,
}

impl Hiposter {
    fn send_request(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if self.loading {
            return;
        }

        self.loading = true;
        self.response = None;
        cx.notify();

        let request = self.request.clone();

        cx.spawn(move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let mut cx = cx.clone();
            let request = request.clone();
            async move {
                let result = http::execute_request(&request).await;
                this.update(&mut cx, |this, cx| {
                    this.loading = false;
                    match result {
                        Ok(resp) => {
                            this.response = Some(resp);
                        }
                        Err(e) => {
                            this.response = Some(model::HttpResponse {
                                status_code: 0,
                                status_text: format!("Error: {}", e),
                                ..Default::default()
                                });
                            }
                        }
                    cx.notify();
                })
                .ok();
            }
        })
        .detach();
    }
}

impl Render for Hiposter {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(0x2e3440))
            .size_full()
            .text_color(rgb(0xd8dee9))
            .child(
                // Header / URL Bar area
                div()
                    .flex()
                    .p_4()
                    .border_b_1()
                    .border_color(rgb(0x4c566a))
                    .gap_4()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .px_2()
                            .bg(rgb(0x3b4252))
                            .rounded_md()
                            .child(format!("{:?}", self.request.method))
                    )
                    .child(
                        div()
                            .flex_1()
                            .px_2()
                            .py_1()
                            .bg(rgb(0x3b4252))
                            .rounded_md()
                            .child(self.request.url.clone())
                    )
                    .child(
                        div()
                            .px_4()
                            .py_1()
                            .bg(if self.loading { rgb(0x4c566a) } else { rgb(0x81a1c1) })
                            .text_color(rgb(0x2e3440))
                            .rounded_md()
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, window, cx| {
                                this.send_request(window, cx);
                            }))
                            .child(if self.loading { "Sending..." } else { "Send" })
                    )
            )
            .child(
                // Main Content area
                div()
                    .flex_1()
                    .flex()
                    .child(
                        // Request Panel (Left)
                        div()
                            .flex_1()
                            .border_r_1()
                            .border_color(rgb(0x4c566a))
                            .p_4()
                            .child("Request Headers / Body (TBD)")
                    )
                    .child(
                        // Response Panel (Right)
                        div()
                            .flex_1()
                            .p_4()
                            .child(
                                if let Some(resp) = &self.response {
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap_2()
                                        .child(format!("Status: {} {}", resp.status_code, resp.status_text))
                                        .child(format!("Size: {} bytes", resp.size))
                                        .child(
                                            div()
                                                .mt_4()
                                                .child("Body:")
                                        )
                                        .child(
                                            div()
                                                .p_2()
                                                .bg(rgb(0x3b4252))
                                                .rounded_md()
                                                .child(resp.body.clone())
                                        )
                                } else {
                                    div().child("No response yet")
                                }
                            )
                    )
            )
    }
}

fn main() {
    application().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_cx| Hiposter {
                request: model::HttpRequest::default(),
                response: None,
                loading: false,
            })
        })
        .unwrap();
    });
}
