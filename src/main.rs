mod model;
mod http;

use gpui::prelude::*;
use gpui::*;
use gpui_component::{
    button::*,
    input::{Input, InputState},
    tab::{Tab, TabBar},
    label::Label,
    menu::{DropdownMenu, PopupMenuItem},
    separator::Separator,
    scroll::ScrollableElement,
    resizable::*,
    *,
};
use gpui_component_assets::Assets;

#[derive(Debug, Clone, Copy, PartialEq)]
enum RequestTab {
    Headers,
    Body,
}

struct HeaderRow {
    key: Entity<InputState>,
    value: Entity<InputState>,
}

struct Hiposter {
    url_input: Entity<InputState>,
    body_input: Entity<InputState>,
    headers: Vec<HeaderRow>,
    request: model::HttpRequest,
    response: Option<model::HttpResponse>,
    loading: bool,
    active_tab: RequestTab,
}

impl Hiposter {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let url_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("https://httpbin.org/get")
                .default_value("https://httpbin.org/get")
        });

        let body_input = cx.new(|cx| {
            InputState::new(window, cx).placeholder("Request body...")
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

    fn add_header(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let key = cx.new(|cx| InputState::new(window, cx).placeholder("Key"));
        let value = cx.new(|cx| InputState::new(window, cx).placeholder("Value"));
        self.headers.push(HeaderRow { key, value });
        cx.notify();
    }

    fn remove_header(&mut self, index: usize, _cx: &mut Context<Self>) {
        self.headers.remove(index);
    }

    fn send_request(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if self.loading {
            return;
        }

        let url = self.url_input.read(cx).value();
        if url.trim().is_empty() {
            return;
        }
        self.request.url = url.to_string();
        self.request.body = self.body_input.read(cx).value().to_string();
        
        // Collect headers
        self.request.headers.clear();
        for row in &self.headers {
            let key = row.key.read(cx).value();
            let value = row.value.read(cx).value();
            if !key.trim().is_empty() {
                self.request.headers.push(model::Header { 
                    key: key.to_string(), 
                    value: value.to_string() 
                });
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

    fn set_method(&mut self, method: model::HttpMethod, _cx: &mut Context<Self>) {
        self.request.method = method;
    }

    fn select_tab(&mut self, tab: RequestTab, _cx: &mut Context<Self>) {
        self.active_tab = tab;
    }
}

impl Render for Hiposter {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.weak_entity();
        
        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .child(
                // Header / URL Bar
                h_flex()
                    .p_4()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .gap_3()
                    .child(
                        Button::new("method-dropdown")
                            .label(format!("{:?}", self.request.method))
                            .dropdown_menu({
                                let view = view.clone();
                                move |menu, _, _| {
                                    let methods = [
                                        model::HttpMethod::GET,
                                        model::HttpMethod::POST,
                                        model::HttpMethod::PUT,
                                        model::HttpMethod::DELETE,
                                        model::HttpMethod::PATCH,
                                        model::HttpMethod::HEAD,
                                    ];
                                    
                                    let mut menu = menu;
                                    for method in methods {
                                        let method_clone = method.clone();
                                        let view = view.clone();
                                        menu = menu.item(
                                            PopupMenuItem::new(format!("{:?}", method))
                                                .on_click(move |_, _, cx| {
                                                    view.update(cx, |this, cx| {
                                                        this.set_method(method_clone.clone(), cx);
                                                        cx.notify();
                                                    }).ok();
                                                })
                                        );
                                    }
                                    menu
                                }
                            })
                    )
                    .child(Input::new(&self.url_input).flex_1())
                    .child(
                        Button::new("send")
                            .primary()
                            .label(if self.loading { "Sending..." } else { "Send" })
                            .disabled(self.loading)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.send_request(window, cx);
                            }))
                    )
            )
            .child(
                // Main Content - Vertical Split
                div()
                    .flex_1()
                    .child(
                        v_resizable("main-split")
                            .child(
                                resizable_panel()
                                    .child(
                                        // Request Panel (Top)
                                        v_flex()
                                            .size_full()
                                            .child(
                                                TabBar::new("tabs")
                                                    .child(
                                                        Tab::new()
                                                            .label("Headers")
                                                            .selected(self.active_tab == RequestTab::Headers)
                                                            .on_click(cx.listener(|this, _, _, cx| {
                                                                this.select_tab(RequestTab::Headers, cx);
                                                            })),
                                                    )
                                                    .child(
                                                        Tab::new()
                                                            .label("Body")
                                                            .selected(self.active_tab == RequestTab::Body)
                                                            .on_click(cx.listener(|this, _, _, cx| {
                                                                this.select_tab(RequestTab::Body, cx);
                                                            })),
                                                    )
                                            )
                                            .child(
                                                v_flex()
                                                    .flex_1()
                                                    .p_4()
                                                    .child(
                                                        match self.active_tab {
                                                            RequestTab::Headers => {
                                                                v_flex()
                                                                    .gap_3()
                                                                    .child(
                                                                        h_flex()
                                                                            .justify_between()
                                                                            .child(Label::new("Request Headers").text_color(cx.theme().foreground))
                                                                            .child(
                                                                                Button::new("add-header")
                                                                                    .label("+ Add Header")
                                                                                    .on_click(cx.listener(|this, _, window, cx| {
                                                                                        this.add_header(window, cx);
                                                                                    }))
                                                                            )
                                                                    )
                                                                    .child(
                                                                        v_flex()
                                                                            .gap_2()
                                                                            .children(self.headers.iter().enumerate().map(|(i, row)| {
                                                                                h_flex()
                                                                                    .gap_2()
                                                                                    .child(Input::new(&row.key).flex_1())
                                                                                    .child(Input::new(&row.value).flex_1())
                                                                                    .child(
                                                                                        Button::new(format!("remove-{}", i))
                                                                                            .label("X")
                                                                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                                                                this.remove_header(i, cx);
                                                                                                cx.notify();
                                                                                            }))
                                                                                    )
                                                                            }))
                                                                    )
                                                            }
                                                            RequestTab::Body => {
                                                                v_flex()
                                                                    .size_full()
                                                                    .gap_2()
                                                                    .child(Label::new("Request Body").text_color(cx.theme().foreground))
                                                                    .child(Input::new(&self.body_input).flex_1())
                                                            }
                                                        }
                                                    )
                                            )
                                    )
                            )
                            .child(
                                resizable_panel()
                                    .child(
                                        // Response Panel (Bottom)
                                        v_flex()
                                            .size_full()
                                            .p_4()
                                            .border_t_1()
                                            .border_color(cx.theme().border)
                                            .child(Label::new("Response").text_color(cx.theme().foreground))
                                            .child(
                                                v_flex()
                                                    .flex_1()
                                                    .mt_4()
                                                    .child(
                                                        if let Some(resp) = &self.response {
                                                            v_flex()
                                                                .gap_2()
                                                                .size_full()
                                                                .child(
                                                                    h_flex()
                                                                        .gap_4()
                                                                        .child(Label::new(format!("Status: {} {}", resp.status_code, resp.status_text)).text_color(cx.theme().foreground))
                                                                        .child(Label::new(format!("Size: {} bytes", resp.size)).text_color(cx.theme().foreground))
                                                                )
                                                                .child(Separator::horizontal())
                                                                .child(
                                                                    v_flex()
                                                                        .flex_1()
                                                                        .overflow_y_scrollbar()
                                                                        .child(
                                                                            div()
                                                                                .mt_2()
                                                                                .p_3()
                                                                                .bg(cx.theme().muted)
                                                                                .rounded_md()
                                                                                .child(resp.body.clone())
                                                                        )
                                                                )
                                                        } else {
                                                            v_flex()
                                                                .child(
                                                                    Label::new("No response yet. Enter URL and click Send.")
                                                                        .text_color(cx.theme().muted_foreground)
                                                                )
                                                        }
                                                    )
                                            )
                                    )
                            )
                    )
            )
    }
}

fn main() {
    let _runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to build tokio runtime");
    let _guard = _runtime.enter();

    let app = gpui_platform::application().with_assets(Assets);

    app.run(move |cx| {
        gpui_component::init(cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(1024.), px(768.)), cx)),
            ..Default::default()
        };

        cx.spawn(async move |cx| {
            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|cx| Hiposter::new(window, cx));
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}
