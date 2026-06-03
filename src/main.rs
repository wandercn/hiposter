mod model;
mod http;
mod components;

use gpui::*;
use gpui_platform::application;
use crate::components::text_input::TextInput;

#[derive(Debug, Clone, Copy, PartialEq)]
enum RequestTab {
    Headers,
    Body,
}

struct HeaderRow {
    key: Entity<TextInput>,
    value: Entity<TextInput>,
}

struct Hiposter {
    url_input: Entity<TextInput>,
    body_input: Entity<TextInput>,
    headers: Vec<HeaderRow>,
    request: model::HttpRequest,
    response: Option<model::HttpResponse>,
    loading: bool,
    active_tab: RequestTab,
}

impl Hiposter {
    fn new(cx: &mut Context<Self>) -> Self {
        let url_input = cx.new(|cx| {
            let mut input = TextInput::new(cx, "https://httpbin.org/get");
            input.set_content("https://httpbin.org/get".to_string(), cx);
            input
        });

        let body_input = cx.new(|cx| {
            TextInput::new(cx, "Request body...")
        });

        Self {
            url_input,
            body_input,
            headers: Vec::new(),
            request: model::HttpRequest::default(),
            response: None,
            loading: false,
            active_tab: RequestTab::Headers,
        }
    }

    fn add_header(&mut self, cx: &mut Context<Self>) {
        let key = cx.new(|cx| TextInput::new(cx, "Header Key"));
        let value = cx.new(|cx| TextInput::new(cx, "Header Value"));
        self.headers.push(HeaderRow { key, value });
        cx.notify();
    }

    fn remove_header(&mut self, index: usize, cx: &mut Context<Self>) {
        self.headers.remove(index);
        cx.notify();
    }

    fn send_request(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if self.loading {
            return;
        }

        let url = self.url_input.read(cx).content();
        if url.trim().is_empty() {
            return;
        }
        self.request.url = url;
        self.request.body = self.body_input.read(cx).content();
        
        // Collect headers
        self.request.headers.clear();
        for row in &self.headers {
            let key = row.key.read(cx).content();
            let value = row.value.read(cx).content();
            if !key.trim().is_empty() {
                self.request.headers.push(model::Header { key, value });
            }
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

    fn toggle_method(&mut self, cx: &mut Context<Self>) {
        self.request.method = match self.request.method {
            model::HttpMethod::GET => model::HttpMethod::POST,
            model::HttpMethod::POST => model::HttpMethod::PUT,
            model::HttpMethod::PUT => model::HttpMethod::DELETE,
            model::HttpMethod::DELETE => model::HttpMethod::GET,
            _ => model::HttpMethod::GET,
        };
        cx.notify();
    }

    fn select_tab(&mut self, tab: RequestTab, cx: &mut Context<Self>) {
        self.active_tab = tab;
        cx.notify();
    }
}

impl Render for Hiposter {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(0x2e3440))
            .size_full()
            .text_color(rgb(0xffffff)) // 强化全局文字为纯白
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
                            .px_3()
                            .py_1()
                            .bg(rgb(0x3b4252))
                            .rounded_md()
                            .cursor_pointer()
                            .text_color(rgb(0x88c0d0)) // 冰蓝色高亮方法
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                this.toggle_method(cx);
                            }))
                            .child(format!("{:?}", self.request.method))
                    )
                    .child(
                        div()
                            .flex_1()
                            .child(self.url_input.clone())
                    )
                    .child(
                        div()
                            .px_6()
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
                            .flex()
                            .flex_col()
                            .child(
                                // Tabs
                                div()
                                    .flex()
                                    .border_b_1()
                                    .border_color(rgb(0x4c566a))
                                    .child(
                                        div()
                                            .px_4()
                                            .py_2()
                                            .cursor_pointer()
                                            .bg(if self.active_tab == RequestTab::Headers { rgb(0x3b4252) } else { rgb(0x2e3440) })
                                            .border_b_2()
                                            .border_color(if self.active_tab == RequestTab::Headers { rgb(0x81a1c1) } else { rgb(0x2e3440) })
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                this.select_tab(RequestTab::Headers, cx);
                                            }))
                                            .child("Headers")
                                    )
                                    .child(
                                        div()
                                            .px_4()
                                            .py_2()
                                            .cursor_pointer()
                                            .bg(if self.active_tab == RequestTab::Body { rgb(0x3b4252) } else { rgb(0x2e3440) })
                                            .border_b_2()
                                            .border_color(if self.active_tab == RequestTab::Body { rgb(0x81a1c1) } else { rgb(0x2e3440) })
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                this.select_tab(RequestTab::Body, cx);
                                            }))
                                            .child("Body")
                                    )
                            )
                            .child(
                                div()
                                    .id("request-panel-content")
                                    .flex_1()
                                    .p_4()
                                    .overflow_y_scroll()
                                    .child(
                                        match self.active_tab {
                                            RequestTab::Headers => {
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap_4()
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .justify_between()
                                                            .items_center()
                                                            .child(div().text_color(rgb(0xffffff)).child("Request Headers"))
                                                            .child(
                                                                div()
                                                                    .px_2()
                                                                    .py_0p5()
                                                                    .bg(rgb(0x81a1c1))
                                                                    .text_color(rgb(0x2e3440))
                                                                    .rounded_md()
                                                                    .cursor_pointer()
                                                                    .text_xs()
                                                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| {
                                                                        this.add_header(cx);
                                                                    }))
                                                                    .child("+ Add Header")
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .gap_2()
                                                            .children(self.headers.iter().enumerate().map(|(i, row)| {
                                                                div()
                                                                    .flex()
                                                                    .gap_2()
                                                                    .items_center()
                                                                    .child(div().flex_1().child(row.key.clone()))
                                                                    .child(div().flex_1().child(row.value.clone()))
                                                                    .child(
                                                                        div()
                                                                            .px_2()
                                                                            .bg(rgb(0xbf616a))
                                                                            .text_color(rgb(0xffffff))
                                                                            .rounded_md()
                                                                            .cursor_pointer()
                                                                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| {
                                                                                this.remove_header(i, cx);
                                                                            }))
                                                                            .child("X")
                                                                    )
                                                            }))
                                                    )
                                            }
                                            RequestTab::Body => {
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap_2()
                                                    .size_full()
                                                    .child(div().text_color(rgb(0xffffff)).child("Request Body (JSON/Text)"))
                                                    .child(div().flex_1().child(self.body_input.clone()))
                                            },
                                        }
                                    )
                            )
                    )
                    .child(
                        // Response Panel (Right)
                        div()
                            .flex_1()
                            .flex()
                            .flex_col()
                            .p_4()
                            .child(div().text_color(rgb(0xffffff)).child("Response"))
                            .child(
                                div()
                                    .id("response-content")
                                    .flex_1()
                                    .mt_2()
                                    .overflow_y_scroll()
                                    .child(
                                        if let Some(resp) = &self.response {
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap_2()
                                                .child(
                                                    div()
                                                        .flex()
                                                        .gap_4()
                                                        .child(format!("Status: {} {}", resp.status_code, resp.status_text))
                                                        .child(format!("Size: {} bytes", resp.size))
                                                )
                                                .child(
                                                    div()
                                                        .mt_2()
                                                        .p_3()
                                                        .bg(rgb(0x3b4252))
                                                        .rounded_md()
                                                        .text_sm()
                                                        .child(resp.body.clone())
                                                )
                                        } else {
                                            div()
                                                .mt_4()
                                                .text_color(rgb(0x616e88))
                                                .child("No response yet. Enter URL and click Send.")
                                        }
                                    )
                            )
                    )
            )
    }
}

fn main() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1024.), px(768.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|cx| Hiposter::new(cx))
            },
        )
        .unwrap();
        cx.activate(true);
    });
}
