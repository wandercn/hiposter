use gpui::prelude::*;
use gpui::*;
use gpui_component::{
    button::*,
    input::{Input, InputState},
    tab::{Tab, TabBar},
    label::Label,
    menu::{DropdownMenu, PopupMenuItem},
    scroll::ScrollableElement,
    resizable::*,
    highlighter::Language,
    *,
};
use crate::model;
use crate::theme::{AppTheme, ThemeColors};
use serde_json::Value;
use crate::format_json;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RequestTab {
    Params,
    Headers,
    Body,
    Auth,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResponseTab {
    Body,
    Headers,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResponseView {
    Pretty,
    Raw,
}

pub struct KeyValueRow {
    pub key: Entity<InputState>,
    pub value: Entity<InputState>,
}

pub struct ApiTab {
    pub url_input: Entity<InputState>,
    pub body_input: Entity<InputState>,
    pub auth_token_input: Entity<InputState>,
    pub auth_username_input: Entity<InputState>,
    pub auth_password_input: Entity<InputState>,
    pub response_body_input: Entity<InputState>,
    pub params: Vec<KeyValueRow>,
    pub headers: Vec<KeyValueRow>,
    pub form_data: Vec<KeyValueRow>,
    pub urlencoded: Vec<KeyValueRow>,
    pub request: model::HttpRequest,
    pub response: Option<model::HttpResponse>,
    pub dirty_response: bool,
    pub loading: bool,
    pub active_tab: RequestTab,
    pub active_response_tab: ResponseTab,
    pub active_response_view: ResponseView,
    pub theme: AppTheme,
    pub url_dirty: bool,
    pub params_dirty: bool,
}

impl EventEmitter<model::HttpRequest> for ApiTab {}

impl ApiTab {
    pub fn with_request(request: model::HttpRequest, theme: AppTheme, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut url = request.url.clone();
        let auth = request.auth.clone();

        if !url.contains('?') && !request.params.is_empty() {
            let mut query = String::new();
            for p in &request.params {
                if !p.key.is_empty() {
                    if !query.is_empty() {
                        query.push('&');
                    }
                    query.push_str(&url_encode(&p.key));
                    if !p.value.is_empty() {
                        query.push('=');
                        query.push_str(&url_encode(&p.value));
                    }
                }
            }
            if !query.is_empty() {
                url = format!("{}?{}", url, query);
            }
        }

        let url_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Enter URL...")
                .default_value(url)
        });

        cx.observe(&url_input, |this, _, cx| {
            this.params_dirty = true;
            cx.notify();
        })
        .detach();

        let mut headers = Vec::new();
        for h in &request.headers {
            let key = cx.new(|cx| InputState::new(window, cx).placeholder("Key").default_value(h.key.clone()));
            let value = cx.new(|cx| InputState::new(window, cx).placeholder("Value").default_value(h.value.clone()));
            headers.push(KeyValueRow { key, value });
        }

        let mut form_data = Vec::new();
        let mut urlencoded = Vec::new();
        let mut body_str = String::new();

        match &request.body {
            model::HttpBody::Raw(raw) => body_str = raw.clone(),
            model::HttpBody::FormData(form) => {
                for item in form {
                    let key = cx.new(|cx| InputState::new(window, cx).placeholder("Key").default_value(item.key.clone()));
                    let value = cx.new(|cx| InputState::new(window, cx).placeholder("Value").default_value(item.value.clone()));
                    form_data.push(KeyValueRow { key, value });
                }
            }
            model::HttpBody::UrlEncoded(form) => {
                for item in form {
                    let key = cx.new(|cx| InputState::new(window, cx).placeholder("Key").default_value(item.key.clone()));
                    let value = cx.new(|cx| InputState::new(window, cx).placeholder("Value").default_value(item.value.clone()));
                    urlencoded.push(KeyValueRow { key, value });
                }
            }
            model::HttpBody::None => {}
        }

        let body_input = cx.new(|cx| {
            let mut state = InputState::new(window, cx)
                .placeholder("Request body...")
                .multi_line(true)
                .code_editor(Language::Json);
            state.set_value(body_str, window, cx);
            state
        });

        let auth_token_input = cx.new(|cx| {
            let mut state = InputState::new(window, cx).placeholder("Bearer Token");
            state.set_value(auth.token, window, cx);
            state
        });
        let auth_username_input = cx.new(|cx| {
            let mut state = InputState::new(window, cx).placeholder("Username");
            state.set_value(auth.username, window, cx);
            state
        });
        let auth_password_input = cx.new(|cx| {
            let mut state = InputState::new(window, cx).placeholder("Password").masked(true);
            state.set_value(auth.password, window, cx);
            state
        });

        let response_body_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Response body...")
                .multi_line(true)
                .code_editor(Language::Json)
        });

        Self {
            url_input,
            body_input,
            auth_token_input,
            auth_username_input,
            auth_password_input,
            response_body_input,
            params: Vec::new(),
            headers,
            form_data,
            urlencoded,
            request,
            response: None,
            dirty_response: false,
            loading: false,
            active_tab: RequestTab::Params,
            active_response_tab: ResponseTab::Body,
            active_response_view: ResponseView::Pretty,
            theme,
            url_dirty: false,
            params_dirty: true,
        }
    }

    fn add_param(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let key = cx.new(|cx| InputState::new(window, cx).placeholder("Key"));
        let value = cx.new(|cx| InputState::new(window, cx).placeholder("Value"));
        
        cx.observe(&key, |this, _, cx| {
            this.url_dirty = true;
            cx.notify();
        }).detach();
        
        cx.observe(&value, |this, _, cx| {
            this.url_dirty = true;
            cx.notify();
        }).detach();
        
        self.params.push(KeyValueRow { key, value });
        cx.notify();
    }

    fn remove_param(&mut self, index: usize, _cx: &mut Context<Self>) {
        self.params.remove(index);
        self.url_dirty = true;
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

    fn add_form_data(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let key = cx.new(|cx| InputState::new(window, cx).placeholder("Key"));
        let value = cx.new(|cx| InputState::new(window, cx).placeholder("Value"));
        self.form_data.push(KeyValueRow { key, value });
        cx.notify();
    }

    fn remove_form_data(&mut self, index: usize, _cx: &mut Context<Self>) {
        self.form_data.remove(index);
    }

    fn add_urlencoded(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let key = cx.new(|cx| InputState::new(window, cx).placeholder("Key"));
        let value = cx.new(|cx| InputState::new(window, cx).placeholder("Value"));
        self.urlencoded.push(KeyValueRow { key, value });
        cx.notify();
    }

    fn remove_urlencoded(&mut self, index: usize, _cx: &mut Context<Self>) {
        self.urlencoded.remove(index);
    }

    fn update_request_state(&mut self, cx: &mut Context<Self>) {
        self.request.url = self.url_input.read(cx).value().to_string();
        
        match self.request.content_type.as_str() {
            "application/none" => self.request.body = model::HttpBody::None,
            "multipart/form-data" => {
                let items = self.form_data.iter().filter_map(|row| {
                    let key = row.key.read(cx).value().to_string();
                    let value = row.value.read(cx).value().to_string();
                    (!key.is_empty()).then_some(model::Header { key, value })
                }).collect();
                self.request.body = model::HttpBody::FormData(items);
            }
            "application/x-www-form-urlencoded" => {
                let items = self.urlencoded.iter().filter_map(|row| {
                    let key = row.key.read(cx).value().to_string();
                    let value = row.value.read(cx).value().to_string();
                    (!key.is_empty()).then_some(model::Header { key, value })
                }).collect();
                self.request.body = model::HttpBody::UrlEncoded(items);
            }
            _ => {
                self.request.body = model::HttpBody::Raw(self.body_input.read(cx).value().to_string());
            }
        }
        
        self.request.headers = self.headers.iter().filter_map(|row| {
            let key = row.key.read(cx).value().trim().to_string();
            let value = row.value.read(cx).value().to_string();
            (!key.is_empty()).then_some(model::Header { key, value })
        }).collect();

        self.request.params = self.params.iter().filter_map(|row| {
            let key = row.key.read(cx).value().trim().to_string();
            let value = row.value.read(cx).value().to_string();
            (!key.is_empty()).then_some(model::Header { key, value })
        }).collect();

        self.request.auth.token = self.auth_token_input.read(cx).value().to_string();
        self.request.auth.username = self.auth_username_input.read(cx).value().to_string();
        self.request.auth.password = self.auth_password_input.read(cx).value().to_string();
    }

    fn set_method(&mut self, method: model::HttpMethod, _cx: &mut Context<Self>) {
        self.request.method = method;
    }

    fn select_tab(&mut self, tab: RequestTab, _cx: &mut Context<Self>) {
        self.active_tab = tab;
    }

    pub fn set_response(&mut self, resp: model::HttpResponse, cx: &mut Context<Self>) {
        self.response = Some(resp);
        self.dirty_response = true;
        cx.notify();
    }

    pub fn set_response_view(&mut self, view: ResponseView, cx: &mut Context<Self>) {
        if self.active_response_view != view {
            self.active_response_view = view;
            self.dirty_response = true;
            cx.notify();
        }
    }

    pub fn update_response_display(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(resp) = &self.response {
            let display_content = if self.active_response_view == ResponseView::Pretty {
                serde_json::from_str::<Value>(&resp.body).ok().map(|v| format_json(&v)).unwrap_or_else(|| resp.body.clone())
            } else {
                resp.body.clone()
            };
            
            let view_pretty = self.active_response_view == ResponseView::Pretty;
            self.response_body_input.update(cx, |this, cx| {
                this.set_value(display_content, window, cx);
                this.set_highlighter(if view_pretty { Language::Json } else { Language::Plain }, cx);
            });
            self.dirty_response = false;
        }
    }

    pub fn set_content_type(&mut self, content_type: &str, cx: &mut Context<Self>) {
        self.request.content_type = content_type.to_string();
        let lang = if content_type == "application/json" { Language::Json } else { Language::Plain };
        self.body_input.update(cx, |this, cx| {
            this.set_highlighter(lang, cx);
        });
        cx.notify();
    }

    fn format_request_json(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let body = self.body_input.read(cx).value().to_string();
        if let Ok(v) = serde_json::from_str::<Value>(&body) {
            let pretty = format_json(&v);
            self.body_input.update(cx, |this, cx| {
                this.set_value(pretty, window, cx);
            });
        }
    }

    fn get_current_params_from_ui(&self, cx: &App) -> Vec<(String, String)> {
        self.params.iter().map(|row| {
            let k = row.key.read(cx).value().to_string();
            let v = row.value.read(cx).value().to_string();
            (k, v)
        }).collect()
    }

    fn sync_params_from_url(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.params_dirty = false;
        
        let url_val = self.url_input.read(cx).value().to_string();
        let (_base, parsed) = parse_url_params(&url_val);
        
        let decoded_parsed: Vec<(String, String)> = parsed.into_iter()
            .map(|(k, v)| (url_decode(&k), url_decode(&v)))
            .collect();
            
        let current_ui = self.get_current_params_from_ui(cx);
        
        if decoded_parsed == current_ui {
            return;
        }
        
        self.params.clear();
        for (k, v) in decoded_parsed {
            let key = cx.new(|cx| InputState::new(window, cx).placeholder("Key").default_value(k));
            let value = cx.new(|cx| InputState::new(window, cx).placeholder("Value").default_value(v));
            
            cx.observe(&key, |this, _, cx| {
                this.url_dirty = true;
                cx.notify();
            }).detach();
            
            cx.observe(&value, |this, _, cx| {
                this.url_dirty = true;
                cx.notify();
            }).detach();
            
            self.params.push(KeyValueRow { key, value });
        }
        cx.notify();
    }

    fn sync_url_from_params(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.url_dirty = false;
        
        let current_url = self.url_input.read(cx).value().to_string();
        let (base, _parsed) = parse_url_params(&current_url);
        
        let ui_params = self.get_current_params_from_ui(cx);
        
        let mut query = String::new();
        for (k, v) in ui_params {
            if !k.is_empty() {
                if !query.is_empty() {
                    query.push('&');
                }
                query.push_str(&url_encode(&k));
                if !v.is_empty() {
                    query.push('=');
                    query.push_str(&url_encode(&v));
                }
            }
        }
        
        let new_url = if query.is_empty() {
            base
        } else {
            format!("{}?{}", base, query)
        };
        
        if new_url != current_url {
            self.url_input.update(cx, |input, cx| {
                input.set_value(new_url, window, cx);
            });
        }
    }

    fn render_url_bar(&self, colors: &ThemeColors, view: WeakEntity<Self>, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let method = self.request.method.clone();
        let method_color = match method {
            model::HttpMethod::GET => colors.green,
            model::HttpMethod::POST => colors.yellow,
            _ => colors.text,
        };

        h_flex()
            .px_4().py_2().border_b_1().border_color(colors.border).gap_3()
            .child(
                h_flex().flex_1().bg(colors.sidebar).rounded_md().border_1().border_color(colors.border).items_center()
                    .child(
                        Button::new("method-dropdown")
                            .label(format!("{:?}", method))
                            .ghost()
                            .text_color(method_color)
                            .dropdown_caret(true)
                            .dropdown_menu({
                                let view = view.clone();
                                move |menu, _, _| {
                                    let methods = [
                                        model::HttpMethod::GET, model::HttpMethod::POST,
                                        model::HttpMethod::PUT, model::HttpMethod::DELETE,
                                        model::HttpMethod::PATCH, model::HttpMethod::HEAD,
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
                    .child(div().w_px().h_5().bg(colors.border).mx_1())
                    .child(Input::new(&self.url_input).flex_1().bordered(false).focus_bordered(false))
            )
            .child(
                Button::new("send")
                    .primary()
                    .icon(IconName::ArrowRight)
                    .label(if self.loading { "Sending..." } else { "Send" })
                    .disabled(self.loading)
                    .on_click(move |_, _, cx| {
                        view.update(cx, |this, cx| {
                            this.update_request_state(cx);
                            let request = this.request.clone();
                            this.loading = true;
                            this.response = None;
                            cx.notify();
                            cx.emit(request); 
                        }).ok();
                    })
            )
    }

    fn render_request_panel(&self, colors: &ThemeColors, _view: WeakEntity<Self>, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let request_content = match self.active_tab {
            RequestTab::Params => {
                v_flex().gap_3().child(
                    h_flex().justify_between().items_center()
                        .child(Label::new("Query Parameters").text_color(colors.text))
                        .child(Button::new("add-param").icon(IconName::Plus).label("Add").small().ghost().on_click(cx.listener(|this, _, window, cx| this.add_param(window, cx))))
                )
                .child(v_flex().gap_2().children(self.params.iter().enumerate().map(|(i, row)| {
                    h_flex().gap_2().child(Input::new(&row.key).flex_1()).child(Input::new(&row.value).flex_1())
                    .child(Button::new(format!("rem-p-{}", i)).icon(IconName::Close).ghost().small().on_click(cx.listener(move |this, _, _, cx| { this.remove_param(i, cx); cx.notify(); })))
                })))
                .into_any_element()
            }
            RequestTab::Headers => {
                v_flex().gap_3().child(
                    h_flex().justify_between().items_center()
                        .child(Label::new("Request Headers").text_color(colors.text))
                        .child(Button::new("add-header").icon(IconName::Plus).label("Add").small().ghost().on_click(cx.listener(|this, _, window, cx| this.add_header(window, cx))))
                )
                .child(v_flex().gap_2().children(self.headers.iter().enumerate().map(|(i, row)| {
                    h_flex().gap_2().child(Input::new(&row.key).flex_1()).child(Input::new(&row.value).flex_1())
                    .child(Button::new(format!("rem-h-{}", i)).icon(IconName::Close).ghost().small().on_click(cx.listener(move |this, _, _, cx| { this.remove_header(i, cx); cx.notify(); })))
                })))
                .into_any_element()
            }
            RequestTab::Body => {
                let view = cx.weak_entity();
                v_flex().size_full().gap_2()
                    .child(
                        h_flex().gap_4().items_center()
                            .child(Label::new("Body Type:").text_color(colors.text))
                            .child(Button::new("body-type-dropdown")
                                .label(self.request.content_type.clone())
                                .ghost()
                                .small()
                                .dropdown_caret(true)
                                .dropdown_menu({
                                    let view = view.clone();
                                    move |menu, _, _| {
                                        let types = [
                                            ("None", "application/none"), ("JSON", "application/json"), 
                                            ("Text", "text/plain"), ("Form-data", "multipart/form-data"),
                                            ("Urlencoded", "application/x-www-form-urlencoded"),
                                            ("XML", "application/xml"), ("HTML", "text/html")
                                        ];
                                        let mut menu = menu;
                                        for (label, val) in types {
                                            let view = view.clone();
                                            menu = menu.item(PopupMenuItem::new(label).on_click(move |_, _, cx| {
                                                view.update(cx, |this, cx| this.set_content_type(val, cx)).ok();
                                            }));
                                        }
                                        menu
                                    }
                                })
                            )
                            .when(self.request.content_type == "application/json", |this| {
                                this.child(Button::new("format-json")
                                    .label("Format JSON")
                                    .ghost()
                                    .small()
                                    .on_click(cx.listener(|this, _, window, cx| this.format_request_json(window, cx))))
                            })
                            .when(self.request.content_type == "multipart/form-data", |this| {
                                this.child(Button::new("add-form-data").icon(IconName::Plus).label("Add").small().ghost().on_click(cx.listener(|this, _, window, cx| this.add_form_data(window, cx))))
                            })
                            .when(self.request.content_type == "application/x-www-form-urlencoded", |this| {
                                this.child(Button::new("add-urlencoded").icon(IconName::Plus).label("Add").small().ghost().on_click(cx.listener(|this, _, window, cx| this.add_urlencoded(window, cx))))
                            })
                    )
                    .child(
                        match self.request.content_type.as_str() {
                            "application/none" => v_flex().flex_1().items_center().justify_center().child(Label::new("No body").text_color(colors.subtext)).into_any_element(),
                            "multipart/form-data" => v_flex().flex_1().overflow_y_scrollbar().children(self.form_data.iter().enumerate().map(|(i, row)| {
                                h_flex().gap_2().child(Input::new(&row.key).flex_1()).child(Input::new(&row.value).flex_1())
                                .child(Button::new(format!("rem-fd-{}", i)).icon(IconName::Close).ghost().small().on_click(cx.listener(move |this, _, _, cx| { this.remove_form_data(i, cx); cx.notify(); })))
                            })).into_any_element(),
                            "application/x-www-form-urlencoded" => v_flex().flex_1().overflow_y_scrollbar().children(self.urlencoded.iter().enumerate().map(|(i, row)| {
                                h_flex().gap_2().child(Input::new(&row.key).flex_1()).child(Input::new(&row.value).flex_1())
                                .child(Button::new(format!("rem-ue-{}", i)).icon(IconName::Close).ghost().small().on_click(cx.listener(move |this, _, _, cx| { this.remove_urlencoded(i, cx); cx.notify(); })))
                            })).into_any_element(),
                            _ => Input::new(&self.body_input).size_full().into_any_element(),
                        }
                    )
                    .into_any_element()
            }
            RequestTab::Auth => {
                let view = cx.weak_entity();
                v_flex().gap_4()
                    .child(h_flex().gap_4().items_center().child(Label::new("Auth Type:").text_color(colors.text)).child(Button::new("auth-dropdown")
                        .label(format!("{:?}", self.request.auth.auth_type))
                        .ghost()
                        .small()
                        .dropdown_caret(true)
                        .dropdown_menu({
                            let view = view.clone();
                            move |menu, _, _| {
                                let mut menu = menu;
                                for t in [model::AuthType::None, model::AuthType::Bearer, model::AuthType::Basic] {
                                    let t_clone = t.clone(); let view = view.clone();
                                    menu = menu.item(PopupMenuItem::new(format!("{:?}", t)).on_click(move |_, _, cx| {
                                        view.update(cx, |this, cx| { this.request.auth.auth_type = t_clone.clone(); cx.notify(); }).ok();
                                    }));
                                }
                                menu
                            }
                        })
                    ))
                    .child(v_flex().gap_2()
                        .when(self.request.auth.auth_type == model::AuthType::Bearer, |this| this.child(Label::new("Token").text_color(colors.text)).child(Input::new(&self.auth_token_input)))
                        .when(self.request.auth.auth_type == model::AuthType::Basic, |this| this.child(Label::new("User").text_color(colors.text)).child(Input::new(&self.auth_username_input)).child(Label::new("Pass").text_color(colors.text)).child(Input::new(&self.auth_password_input)))
                        .when(self.request.auth.auth_type == model::AuthType::None, |this| this.child(Label::new("No Auth").text_color(colors.subtext)))
                    ).into_any_element()
            }
        };

        v_flex().size_full().child(
            TabBar::new("req-tabs").w_full()
                .child(Tab::new().label("Params").selected(self.active_tab == RequestTab::Params).on_click(cx.listener(|this, _, _, cx| this.select_tab(RequestTab::Params, cx))))
                .child(Tab::new().label("Headers").selected(self.active_tab == RequestTab::Headers).on_click(cx.listener(|this, _, _, cx| this.select_tab(RequestTab::Headers, cx))))
                .child(Tab::new().label("Body").selected(self.active_tab == RequestTab::Body).on_click(cx.listener(|this, _, _, cx| this.select_tab(RequestTab::Body, cx))))
                .child(Tab::new().label("Auth").selected(self.active_tab == RequestTab::Auth).on_click(cx.listener(|this, _, _, cx| this.select_tab(RequestTab::Auth, cx))))
        ).child(v_flex().flex_1().p_4().child(request_content))
    }

    fn render_response_panel(&self, colors: &ThemeColors, _view: WeakEntity<Self>, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().size_full().border_t_1().border_color(colors.border)
            .child(
                TabBar::new("res-tabs").w_full()
                    .child(Tab::new().label("Body").selected(self.active_response_tab == ResponseTab::Body).on_click(cx.listener(|this, _, _, cx| { this.active_response_tab = ResponseTab::Body; cx.notify(); })))
                    .child(Tab::new().label("Headers").selected(self.active_response_tab == ResponseTab::Headers).on_click(cx.listener(|this, _, _, cx| { this.active_response_tab = ResponseTab::Headers; cx.notify(); })))
                    .suffix(if let Some(resp) = &self.response {
                        let is_success = resp.status_code < 400 && resp.status_code > 0;
                        let mut status_bg = if is_success { colors.green } else { colors.red };
                        status_bg.a = 0.15;

                        let mut meta_bg = colors.text;
                        meta_bg.a = 0.08;

                        h_flex().gap_2().items_center().px_4()
                            // Status Code Badge
                            .child(
                                h_flex().items_center().gap_1().px_2().py_0p5().rounded_md().bg(status_bg)
                                    .child(Icon::new(if is_success { IconName::CircleCheck } else { IconName::CircleX })
                                        .small()
                                        .text_color(if is_success { colors.green } else { colors.red }))
                                    .child(Label::new(format!("{} {}", resp.status_code, resp.status_text))
                                        .text_color(if is_success { colors.green } else { colors.red })
                                        .text_size(rems(0.75))
                                        .font_weight(gpui::FontWeight::BOLD))
                            )
                            // Time Badge
                            .child(
                                h_flex().items_center().gap_1().px_2().py_0p5().rounded_md().bg(meta_bg)
                                    .child(Icon::new(IconName::Info).small().text_color(colors.subtext))
                                    .child(Label::new(format!("{} ms", resp.elapsed_ms))
                                        .text_color(colors.subtext)
                                        .text_size(rems(0.75))
                                        .font_weight(gpui::FontWeight::MEDIUM))
                            )
                            // Size Badge
                            .child(
                                h_flex().items_center().gap_1().px_2().py_0p5().rounded_md().bg(meta_bg)
                                    .child(Icon::new(IconName::Inbox).small().text_color(colors.subtext))
                                    .child(Label::new(format!("{} bytes", resp.size))
                                        .text_color(colors.subtext)
                                        .text_size(rems(0.75))
                                        .font_weight(gpui::FontWeight::MEDIUM))
                            )
                    } else { div() })
            )
            .child(v_flex().flex_1().child(
                if self.loading {
                    v_flex().size_full().items_center().justify_center().gap_4()
                        .child(gpui_component::spinner::Spinner::new().with_size(gpui_component::Size::Large).color(colors.blue))
                        .child(Label::new("Sending request...").text_color(colors.subtext))
                        .into_any_element()
                } else if let Some(resp) = &self.response {
                    match self.active_response_tab {
                        ResponseTab::Body => v_flex().size_full()
                            .child(
                                h_flex().px_4().py_1p5().child(
                                    h_flex().bg(colors.sidebar).rounded_md().border_1().border_color(colors.border).p_0p5().gap_0p5()
                                        .child(
                                            Button::new("pretty")
                                                .label("Pretty")
                                                .small()
                                                .ghost()
                                                .when(self.active_response_view == ResponseView::Pretty, |s| s.bg(colors.bg).text_color(colors.blue))
                                                .when(self.active_response_view != ResponseView::Pretty, |s| s.text_color(colors.subtext))
                                                .on_click(cx.listener(|this, _, _, cx| { this.set_response_view(ResponseView::Pretty, cx); }))
                                        )
                                        .child(
                                            Button::new("raw")
                                                .label("Raw")
                                                .small()
                                                .ghost()
                                                .when(self.active_response_view == ResponseView::Raw, |s| s.bg(colors.bg).text_color(colors.blue))
                                                .when(self.active_response_view != ResponseView::Raw, |s| s.text_color(colors.subtext))
                                                .on_click(cx.listener(|this, _, _, cx| { this.set_response_view(ResponseView::Raw, cx); }))
                                        )
                                )
                            )
                            .child(
                                v_flex().flex_1().p_4().child(
                                    Input::new(&self.response_body_input).size_full()
                                )
                            )
                            .into_any_element(),
                        ResponseTab::Headers => v_flex().flex_1().p_4().overflow_y_scrollbar().children(resp.headers.iter().map(|h| h_flex().gap_2().py_1().border_b_1().border_color(colors.border).child(div().w_48().child(Label::new(h.key.as_str()).text_color(colors.subtext))).child(div().flex_1().child(Label::new(h.value.as_str()).text_color(colors.text))))).into_any_element()
                    }
                } else {
                    v_flex().size_full().items_center().justify_center().gap_4().child(Icon::new(IconName::Globe).size_12().text_color(colors.subtext)).child(Label::new("No response yet").text_color(colors.subtext)).into_any_element()
                }
            ))
    }
}

impl Render for ApiTab {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = self.theme.colors();
        let view = cx.weak_entity();

        if self.params_dirty {
            self.sync_params_from_url(window, cx);
        }
        if self.url_dirty {
            self.sync_url_from_params(window, cx);
        }
        if self.dirty_response {
            self.update_response_display(window, cx);
        }
        
        v_flex().flex_1().size_full().bg(colors.bg)
            .child(self.render_url_bar(&colors, view.clone(), window, cx))
            .child(div().flex_1().size_full().child(
                v_resizable("main-split")
                    .child(resizable_panel().size(px(450.)).min_size(px(100.)).child(self.render_request_panel(&colors, view.clone(), window, cx)))
                    .child(resizable_panel().flex_1().min_size(px(150.)).child(self.render_response_panel(&colors, view.clone(), window, cx)))
            ))
    }
}

pub fn url_decode(s: &str) -> String {
    let mut decoded = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '%' {
            let mut hex = String::new();
            if let Some(h1) = chars.next() { hex.push(h1); }
            if let Some(h2) = chars.next() { hex.push(h2); }
            if let Ok(b) = u8::from_str_radix(&hex, 16) {
                decoded.push(b as char);
            } else {
                decoded.push('%');
                decoded.push_str(&hex);
            }
        } else if c == '+' {
            decoded.push(' ');
        } else {
            decoded.push(c);
        }
    }
    decoded
}

pub fn url_encode(s: &str) -> String {
    let mut encoded = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(b as char);
            }
            b' ' => {
                encoded.push('+');
            }
            _ => {
                encoded.push_str(&format!("%{:02X}", b));
            }
        }
    }
    encoded
}

pub fn parse_url_params(url: &str) -> (String, Vec<(String, String)>) {
    if let Some((base, query)) = url.split_once('?') {
        let mut params = Vec::new();
        for pair in query.split('&') {
            if pair.is_empty() {
                continue;
            }
            let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
            params.push((k.to_string(), v.to_string()));
        }
        (base.to_string(), params)
    } else {
        (url.to_string(), Vec::new())
    }
}
