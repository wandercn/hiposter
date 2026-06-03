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
    highlighter::Language,
    *,
};
use gpui_component_assets::Assets;
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq)]
enum RequestTab {
    Params,
    Headers,
    Body,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ResponseView {
    Pretty,
    Raw,
}

struct KeyValueRow {
    key: Entity<InputState>,
    value: Entity<InputState>,
}

struct Hiposter {
    url_input: Entity<InputState>,
    body_input: Entity<InputState>,
    params: Vec<KeyValueRow>,
    headers: Vec<KeyValueRow>,
    request: model::HttpRequest,
    response: Option<model::HttpResponse>,
    loading: bool,
    active_tab: RequestTab,
    active_response_view: ResponseView,
}

impl Hiposter {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let url_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("https://httpbin.org/get")
                .default_value("https://httpbin.org/get")
        });

        let body_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Request body...")
                .multi_line(true)
        });

        Self {
            url_input,
            body_input,
            params: Vec::new(),
            headers: Vec::new(),
            request: model::HttpRequest::default(),
            response: None,
            loading: false,
            active_tab: RequestTab::Params,
            active_response_view: ResponseView::Pretty,
        }
    }

    fn add_param(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let key = cx.new(|cx| InputState::new(window, cx).placeholder("Key"));
        let value = cx.new(|cx| InputState::new(window, cx).placeholder("Value"));
        self.params.push(KeyValueRow { key, value });
        cx.notify();
    }

    fn remove_param(&mut self, index: usize, _cx: &mut Context<Self>) {
        self.params.remove(index);
    }

    fn add_header(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let key = cx.new(|cx| InputState::new(window, cx).placeholder("Key"));
        let value = cx.new(|cx| InputState::new(window, cx).placeholder("Value"));
        self.headers.push(KeyValueRow { key, value });
        cx.notify();
    }

    fn remove_header(&mut self, index: usize, _cx: &mut Context<Self>) {
        self.headers.remove(index);
    }

    fn sync_params_to_url(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let mut url = self.url_input.read(cx).value().to_string();
        if let Some(pos) = url.find('?') {
            url.truncate(pos);
        }

        let mut query = String::new();
        for row in &self.params {
            let k = row.key.read(cx).value();
            let v = row.value.read(cx).value();
            if !k.trim().is_empty() {
                if !query.is_empty() {
                    query.push('&');
                }
                query.push_str(&k);
                query.push('=');
                query.push_str(&v);
            }
        }

        if !query.is_empty() {
            url.push('?');
            url.push_str(&query);
        }
        
        let url_clone = url.clone();
        self.url_input.update(cx, |this, cx| {
            this.set_value(url_clone, window, cx);
        });
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

    fn set_content_type(&mut self, content_type: &str, _window: &mut Window, cx: &mut Context<Self>) {
        self.request.content_type = content_type.to_string();
        if content_type == "application/json" {
            self.body_input.update(cx, |this, cx| {
                this.set_highlighter(Language::Json, cx);
            });
        } else {
            self.body_input.update(cx, |this, cx| {
                this.set_highlighter(Language::Plain, cx);
            });
        }
        cx.notify();
    }

    fn format_request_json(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let body = self.body_input.read(cx).value().to_string();
        if let Ok(v) = serde_json::from_str::<Value>(&body) {
            if let Ok(pretty) = serde_json::to_string_pretty(&v) {
                self.body_input.update(cx, |this, cx| {
                    this.set_value(pretty, window, cx);
                });
            }
        }
    }
}

impl Render for Hiposter {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                                                TabBar::new("request-tabs")
                                                    .child(Tab::new().label("Params").selected(self.active_tab == RequestTab::Params).on_click(cx.listener(|this, _, _, cx| this.select_tab(RequestTab::Params, cx))))
                                                    .child(Tab::new().label("Headers").selected(self.active_tab == RequestTab::Headers).on_click(cx.listener(|this, _, _, cx| this.select_tab(RequestTab::Headers, cx))))
                                                    .child(Tab::new().label("Body").selected(self.active_tab == RequestTab::Body).on_click(cx.listener(|this, _, _, cx| this.select_tab(RequestTab::Body, cx))))
                                            )
                                            .child(
                                                v_flex()
                                                    .flex_1()
                                                    .p_4()
                                                    .child(
                                                        match self.active_tab {
                                                            RequestTab::Params => {
                                                                v_flex().gap_3().child(
                                                                    h_flex().justify_between().child(Label::new("Query Parameters").text_color(cx.theme().foreground))
                                                                    .child(Button::new("add-param").label("+ Add Param").on_click(cx.listener(|this, _, window, cx| this.add_param(window, cx))))
                                                                )
                                                                .child(v_flex().gap_2().children(self.params.iter().enumerate().map(|(i, row)| {
                                                                    h_flex().gap_2().child(Input::new(&row.key).flex_1()).child(Input::new(&row.value).flex_1())
                                                                    .child(Button::new(format!("rem-p-{}", i)).label("X").on_click(cx.listener(move |this, _, window, cx| { this.remove_param(i, cx); this.sync_params_to_url(window, cx); cx.notify(); })))
                                                                })))
                                                            }
                                                            RequestTab::Headers => {
                                                                v_flex().gap_3().child(
                                                                    h_flex().justify_between().child(Label::new("Request Headers").text_color(cx.theme().foreground))
                                                                    .child(Button::new("add-header").label("+ Add Header").on_click(cx.listener(|this, _, window, cx| this.add_header(window, cx))))
                                                                )
                                                                .child(v_flex().gap_2().children(self.headers.iter().enumerate().map(|(i, row)| {
                                                                    h_flex().gap_2().child(Input::new(&row.key).flex_1()).child(Input::new(&row.value).flex_1())
                                                                    .child(Button::new(format!("rem-h-{}", i)).label("X").on_click(cx.listener(move |this, _, _, cx| { this.remove_header(i, cx); cx.notify(); })))
                                                                })))
                                                            }
                                                            RequestTab::Body => {
                                                                v_flex().size_full().gap_2()
                                                                    .child(
                                                                        h_flex().gap_4().items_center()
                                                                            .child(Label::new("Body Type:").text_color(cx.theme().foreground))
                                                                            .child(Button::new("body-type-dropdown").label(self.request.content_type.clone()).dropdown_menu({
                                                                                let view = view.clone();
                                                                                move |menu, _, _| {
                                                                                    let types = [("None", "application/none"), ("JSON", "application/json"), ("Text", "text/plain"), ("XML", "application/xml"), ("HTML", "text/html")];
                                                                                    let mut menu = menu;
                                                                                    for (label, val) in types {
                                                                                        let view = view.clone();
                                                                                        menu = menu.item(PopupMenuItem::new(label).on_click(move |_, window, cx| {
                                                                                            view.update(cx, |this, cx| this.set_content_type(val, window, cx)).ok();
                                                                                        }));
                                                                                    }
                                                                                    menu
                                                                                }
                                                                            }))
                                                                            .when(self.request.content_type == "application/json", |this| {
                                                                                this.child(Button::new("format-json").label("Format JSON").on_click(cx.listener(|this, _, window, cx| this.format_request_json(window, cx))))
                                                                            })
                                                                    )
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
                                            .child(
                                                h_flex().justify_between().items_center()
                                                    .child(Label::new("Response").text_color(cx.theme().foreground))
                                                    .child(
                                                        TabBar::new("res-view-tabs").child(Tab::new().label("Pretty").selected(self.active_response_view == ResponseView::Pretty).on_click(cx.listener(|this, _, _, cx| { this.active_response_view = ResponseView::Pretty; cx.notify(); })))
                                                        .child(Tab::new().label("Raw").selected(self.active_response_view == ResponseView::Raw).on_click(cx.listener(|this, _, _, cx| { this.active_response_view = ResponseView::Raw; cx.notify(); })))
                                                    )
                                            )
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
                                                                                .child({
                                                                                    let content = resp.body.clone();
                                                                                    if self.active_response_view == ResponseView::Pretty {
                                                                                        if let Ok(v) = serde_json::from_str::<Value>(&content) {
                                                                                            serde_json::to_string_pretty(&v).unwrap_or(content)
                                                                                        } else {
                                                                                            content
                                                                                        }
                                                                                    } else {
                                                                                        content
                                                                                    }
                                                                                })
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
