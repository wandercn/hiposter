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
    scroll::ScrollableElement,
    resizable::*,
    highlighter::Language,
    *,
};
use gpui_component_assets::Assets as GpuiAssets;
use serde::Serialize;
use serde_json::Value;
use std::fs;

const TRASH_2_SVG: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-trash2-icon lucide-trash-2"><path d="M10 11v6"/><path d="M14 11v6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"/><path d="M3 6h18"/><path d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>"#;

pub enum CustomIconName {
    Trash,
}

impl IconNamed for CustomIconName {
    fn path(self) -> SharedString {
        match self {
            CustomIconName::Trash => "icons/trash-2.svg".into(),
        }
    }
}

impl RenderOnce for CustomIconName {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        Icon::new(self)
    }
}

struct AppAssets;

impl AssetSource for AppAssets {
    fn load(&self, path: &str) -> gpui::Result<Option<std::borrow::Cow<'static, [u8]>>> {
        if path == "icons/trash-2.svg" {
            return Ok(Some(std::borrow::Cow::Borrowed(TRASH_2_SVG)));
        }
        GpuiAssets.load(path)
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        let mut list = GpuiAssets.list(path)?;
        if "icons/".starts_with(path) {
            list.push("icons/trash-2.svg".into());
        }
        Ok(list)
    }
}

fn format_json(value: &Value) -> String {
    let mut buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
    if value.serialize(&mut ser).is_ok() {
        String::from_utf8(buf).unwrap_or_default()
    } else {
        serde_json::to_string_pretty(value).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AppTheme {
    GitHubLight,
    SolarizedLight,
    OneLight,
    VitesseLight,
    CatppuccinLatte,
}

#[derive(Clone, Copy)]
pub struct ThemeColors {
    pub bg: Hsla,
    pub sidebar: Hsla,
    pub surface: Hsla,
    pub border: Hsla,
    pub text: Hsla,
    pub subtext: Hsla,
    pub blue: Hsla,
    pub green: Hsla,
    pub yellow: Hsla,
    pub red: Hsla,
}

impl AppTheme {
    fn colors(&self) -> ThemeColors {
        match self {
            AppTheme::GitHubLight => ThemeColors {
                bg: Hsla::from(rgb(0xffffff)),
                sidebar: Hsla::from(rgb(0xf6f8fa)),
                surface: Hsla::from(rgb(0xf3f4f6)),
                border: Hsla::from(rgb(0xd0d7de)),
                text: Hsla::from(rgb(0x24292f)),
                subtext: Hsla::from(rgb(0x57606a)),
                blue: Hsla::from(rgb(0x0969da)),
                green: Hsla::from(rgb(0x1a7f37)),
                yellow: Hsla::from(rgb(0xbf8700)),
                red: Hsla::from(rgb(0xd1242f)),
            },
            AppTheme::SolarizedLight => ThemeColors {
                bg: Hsla::from(rgb(0xfdf6e3)),
                sidebar: Hsla::from(rgb(0xeee8d5)),
                surface: Hsla::from(rgb(0xe8e2c8)),
                border: Hsla::from(rgb(0xd3caba)),
                text: Hsla::from(rgb(0x657b83)),
                subtext: Hsla::from(rgb(0x93a1a1)),
                blue: Hsla::from(rgb(0x268bd2)),
                green: Hsla::from(rgb(0x859900)),
                yellow: Hsla::from(rgb(0xb58900)),
                red: Hsla::from(rgb(0xdc322f)),
            },
            AppTheme::OneLight => ThemeColors {
                bg: Hsla::from(rgb(0xfafafa)),
                sidebar: Hsla::from(rgb(0xf0f0f0)),
                surface: Hsla::from(rgb(0xe5e5e6)),
                border: Hsla::from(rgb(0xd7d7d7)),
                text: Hsla::from(rgb(0x383a42)),
                subtext: Hsla::from(rgb(0xa0a1a7)),
                blue: Hsla::from(rgb(0x4078f2)),
                green: Hsla::from(rgb(0x50a14f)),
                yellow: Hsla::from(rgb(0xc18401)),
                red: Hsla::from(rgb(0xe45649)),
            },
            AppTheme::VitesseLight => ThemeColors {
                bg: Hsla::from(rgb(0xffffff)),
                sidebar: Hsla::from(rgb(0xf8f8f8)),
                surface: Hsla::from(rgb(0xf0f0f0)),
                border: Hsla::from(rgb(0xeeeeee)),
                text: Hsla::from(rgb(0x393a34)),
                subtext: Hsla::from(rgb(0xa0ada0)),
                blue: Hsla::from(rgb(0x0550ae)),
                green: Hsla::from(rgb(0x29834d)),
                yellow: Hsla::from(rgb(0xa65e2b)),
                red: Hsla::from(rgb(0xd44c47)),
            },
            AppTheme::CatppuccinLatte => ThemeColors {
                bg: Hsla::from(rgb(0xeff1f5)),
                sidebar: Hsla::from(rgb(0xe6e9ef)),
                surface: Hsla::from(rgb(0xdce0e8)),
                border: Hsla::from(rgb(0xccd0da)),
                text: Hsla::from(rgb(0x4c4f69)),
                subtext: Hsla::from(rgb(0x8c8fa1)),
                blue: Hsla::from(rgb(0x1e66f5)),
                green: Hsla::from(rgb(0x40a02b)),
                yellow: Hsla::from(rgb(0xdf8e1d)),
                red: Hsla::from(rgb(0xd20f39)),
            },
        }
    }
    
    fn name(&self) -> &'static str {
        match self {
            AppTheme::GitHubLight => "GitHub Light",
            AppTheme::SolarizedLight => "Solarized Light",
            AppTheme::OneLight => "One Light",
            AppTheme::VitesseLight => "Vitesse Light",
            AppTheme::CatppuccinLatte => "Catppuccin Latte",
        }
    }
}

impl Default for AppTheme {
    fn default() -> Self {
        AppTheme::VitesseLight
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum RequestTab {
    Params,
    Headers,
    Body,
    Auth,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ResponseTab {
    Body,
    Headers,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct HistoryItem {
    request: model::HttpRequest,
    timestamp: u64,
}

struct ApiTab {
    title: String,
    url_input: Entity<InputState>,
    body_input: Entity<InputState>,
    auth_token_input: Entity<InputState>,
    auth_username_input: Entity<InputState>,
    auth_password_input: Entity<InputState>,
    response_body_input: Entity<InputState>,
    params: Vec<KeyValueRow>,
    headers: Vec<KeyValueRow>,
    form_data: Vec<KeyValueRow>,
    urlencoded: Vec<KeyValueRow>,
    request: model::HttpRequest,
    response: Option<model::HttpResponse>,
    loading: bool,
    active_tab: RequestTab,
    active_response_tab: ResponseTab,
    active_response_view: ResponseView,
    theme: AppTheme,
}

impl EventEmitter<model::HttpRequest> for ApiTab {}

impl ApiTab {
    fn with_request(request: model::HttpRequest, theme: AppTheme, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let url = request.url.clone();
        let auth = request.auth.clone();

        let url_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Enter URL...")
                .default_value(url)
        });

        cx.observe(&url_input, |_, _, cx| {
            cx.notify();
        })
        .detach();

        let mut headers = Vec::new();
        for h in &request.headers {
            let key = cx.new(|cx| InputState::new(window, cx).default_value(h.key.clone()));
            let value = cx.new(|cx| InputState::new(window, cx).default_value(h.value.clone()));
            headers.push(KeyValueRow { key, value });
        }

        let mut form_data = Vec::new();
        let mut urlencoded = Vec::new();
        let mut body_str = String::new();

        match &request.body {
            model::HttpBody::Raw(raw) => body_str = raw.clone(),
            model::HttpBody::FormData(form) => {
                for item in form {
                    let key = cx.new(|cx| InputState::new(window, cx).default_value(item.key.clone()));
                    let value = cx.new(|cx| InputState::new(window, cx).default_value(item.value.clone()));
                    form_data.push(KeyValueRow { key, value });
                }
            }
            model::HttpBody::UrlEncoded(form) => {
                for item in form {
                    let key = cx.new(|cx| InputState::new(window, cx).default_value(item.key.clone()));
                    let value = cx.new(|cx| InputState::new(window, cx).default_value(item.value.clone()));
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
            title: "New Request".to_string(),
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
            loading: false,
            active_tab: RequestTab::Params,
            active_response_tab: ResponseTab::Body,
            active_response_view: ResponseView::Pretty,
            theme,
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
        
        // Handle Body based on content type
        match self.request.content_type.as_str() {
            "application/none" => self.request.body = model::HttpBody::None,
            "multipart/form-data" => {
                let mut items = Vec::new();
                for row in &self.form_data {
                    let key = row.key.read(cx).value().to_string();
                    let value = row.value.read(cx).value().to_string();
                    if !key.is_empty() {
                        items.push(model::Header { key, value });
                    }
                }
                self.request.body = model::HttpBody::FormData(items);
            }
            "application/x-www-form-urlencoded" => {
                let mut items = Vec::new();
                for row in &self.urlencoded {
                    let key = row.key.read(cx).value().to_string();
                    let value = row.value.read(cx).value().to_string();
                    if !key.is_empty() {
                        items.push(model::Header { key, value });
                    }
                }
                self.request.body = model::HttpBody::UrlEncoded(items);
            }
            _ => {
                // Default to Raw for JSON, Text, etc.
                self.request.body = model::HttpBody::Raw(self.body_input.read(cx).value().to_string());
            }
        }
        
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

    fn set_content_type(&mut self, content_type: &str, cx: &mut Context<Self>) {
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
            let pretty = format_json(&v);
            self.body_input.update(cx, |this, cx| {
                this.set_value(pretty, window, cx);
            });
        }
    }
}

impl Render for ApiTab {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = self.theme.colors();
        let view = cx.weak_entity();
        
        if let Some(resp) = &self.response {
            let content = resp.body.clone();
            let display_content = if self.active_response_view == ResponseView::Pretty {
                if let Ok(v) = serde_json::from_str::<Value>(&content) {
                    format_json(&v)
                } else {
                    content
                }
            } else {
                content
            };
            
            if self.response_body_input.read(cx).value().to_string() != display_content {
                let current_view = self.active_response_view;
                self.response_body_input.update(cx, |this, cx| {
                    this.set_value(display_content, window, cx);
                    // Force refresh highlighter by toggling
                    this.set_highlighter(Language::Plain, cx);
                    if current_view == ResponseView::Pretty {
                        this.set_highlighter(Language::Json, cx);
                    }
                });
            }
        }

        let request_content = match self.active_tab {
            RequestTab::Params => {
                v_flex().gap_3().child(
                    h_flex().justify_between().items_center()
                        .child(Label::new("Query Parameters").text_color(colors.text))
                        .child(
                            Button::new("add-param")
                                .icon(IconName::Plus)
                                .label("Add")
                                .small()
                                .ghost()
                                .on_click(cx.listener(|this, _, window, cx| this.add_param(window, cx)))
                        )

                )
                .child(v_flex().gap_2().children(self.params.iter().enumerate().map(|(i, row)| {
                    h_flex().gap_2().child(Input::new(&row.key).flex_1()).child(Input::new(&row.value).flex_1())
                    .child(Button::new(format!("rem-p-{}", i)).label("X").on_click(cx.listener(move |this, _, _, cx| { this.remove_param(i, cx); cx.notify(); })))
                })))
                .into_any_element()
            }
            RequestTab::Headers => {
                v_flex().gap_3().child(
                    h_flex().justify_between().items_center()
                        .child(Label::new("Request Headers").text_color(colors.text))
                        .child(
                            Button::new("add-header")
                                .icon(IconName::Plus)
                                .label("Add")
                                .small()
                                .ghost()
                                .on_click(cx.listener(|this, _, window, cx| this.add_header(window, cx)))
                        )

                )
                .child(v_flex().gap_2().children(self.headers.iter().enumerate().map(|(i, row)| {
                    h_flex().gap_2().child(Input::new(&row.key).flex_1()).child(Input::new(&row.value).flex_1())
                    .child(Button::new(format!("rem-h-{}", i)).label("X").on_click(cx.listener(move |this, _, _, cx| { this.remove_header(i, cx); cx.notify(); })))
                })))
                .into_any_element()
            }
            RequestTab::Body => {
                v_flex().size_full().gap_2()
                    .child(
                        h_flex().gap_4().items_center()
                            .child(Label::new("Body Type:").text_color(colors.text))
                            .child(Button::new("body-type-dropdown").label(self.request.content_type.clone()).dropdown_menu({
                                let view = view.clone();
                                move |menu, _, _| {
                                    let types = [
                                        ("None", "application/none"), 
                                        ("JSON", "application/json"), 
                                        ("Text", "text/plain"), 
                                        ("Form-data", "multipart/form-data"),
                                        ("x-www-form-urlencoded", "application/x-www-form-urlencoded"),
                                        ("XML", "application/xml"), 
                                        ("HTML", "text/html")
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
                            }))
                            .when(self.request.content_type == "application/json", |this| {
                                this.child(Button::new("format-json").label("Format JSON").on_click(cx.listener(|this, _, window, cx| this.format_request_json(window, cx))))
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
                            "application/none" => {
                                v_flex().flex_1().items_center().justify_center()
                                    .child(Label::new("This request does not have a body").text_color(colors.subtext))
                                    .into_any_element()
                            }
                            "multipart/form-data" => {
                                v_flex().flex_1().overflow_y_scrollbar()
                                    .children(self.form_data.iter().enumerate().map(|(i, row)| {
                                        h_flex().gap_2().child(Input::new(&row.key).flex_1()).child(Input::new(&row.value).flex_1())
                                        .child(Button::new(format!("rem-fd-{}", i)).label("X").on_click(cx.listener(move |this, _, _, cx| { this.remove_form_data(i, cx); cx.notify(); })))
                                    }))
                                    .into_any_element()
                            }
                            "application/x-www-form-urlencoded" => {
                                v_flex().flex_1().overflow_y_scrollbar()
                                    .children(self.urlencoded.iter().enumerate().map(|(i, row)| {
                                        h_flex().gap_2().child(Input::new(&row.key).flex_1()).child(Input::new(&row.value).flex_1())
                                        .child(Button::new(format!("rem-ue-{}", i)).label("X").on_click(cx.listener(move |this, _, _, cx| { this.remove_urlencoded(i, cx); cx.notify(); })))
                                    }))
                                    .into_any_element()
                            }
                            _ => {
                                div()
                                    .flex_1()
                                    .bg(colors.bg) // 纯白背景以确保 JSON 高亮对比度
                                    .border_1()
                                    .border_color(colors.border)
                                    .p_1()
                                    .rounded_md()
                                    .child(Input::new(&self.body_input).size_full())
                                    .into_any_element()
                            }
                        }
                    )
                    .into_any_element()
            }
            RequestTab::Auth => {
                v_flex().gap_4()
                    .child(
                        h_flex().gap_4().items_center()
                            .child(Label::new("Auth Type:").text_color(colors.text))
                            .child(Button::new("auth-type-dropdown").label(format!("{:?}", self.request.auth.auth_type)).dropdown_menu({
                                let view = view.clone();
                                move |menu, _, _| {
                                    let types = [model::AuthType::None, model::AuthType::Bearer, model::AuthType::Basic];
                                    let mut menu = menu;
                                    for t in types {
                                        let t_clone = t.clone();
                                        let view = view.clone();
                                        menu = menu.item(PopupMenuItem::new(format!("{:?}", t)).on_click(move |_, _, cx| {
                                            view.update(cx, |this, cx| { this.request.auth.auth_type = t_clone.clone(); cx.notify(); }).ok();
                                        }));
                                    }
                                    menu
                                }
                            }))
                    )
                    .child(
                        v_flex().gap_2()
                            .when(self.request.auth.auth_type == model::AuthType::Bearer, |this| {
                                this.child(Label::new("Token").text_color(colors.text)).child(Input::new(&self.auth_token_input))
                            })
                            .when(self.request.auth.auth_type == model::AuthType::Basic, |this| {
                                this.child(Label::new("Username").text_color(colors.text)).child(Input::new(&self.auth_username_input))
                                    .child(Label::new("Password").text_color(colors.text)).child(Input::new(&self.auth_password_input))
                            })
                            .when(self.request.auth.auth_type == model::AuthType::None, |this| {
                                this.child(Label::new("No authentication required").text_color(colors.subtext))
                            })
                    )
                    .into_any_element()
            }
        };

        let response_content = if let Some(resp) = &self.response {
            match self.active_response_tab {
                ResponseTab::Body => {
                    v_flex().size_full()
                        .child(
                            h_flex().items_center().px_4().py_1().gap_2()
                                .child(
                                    Button::new("view-pretty")
                                        .label("Pretty")
                                        .small()
                                        .ghost()
                                        .when(self.active_response_view == ResponseView::Pretty, |this| this.bg(colors.surface).text_color(colors.blue))
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.active_response_view = ResponseView::Pretty;
                                            this.response_body_input.update(cx, |input, cx| {
                                                input.set_highlighter(Language::Plain, cx);
                                                input.set_highlighter(Language::Json, cx);
                                            });
                                            cx.notify();
                                        }))
                                )
                                .child(
                                    Button::new("view-raw")
                                        .label("Raw")
                                        .small()
                                        .ghost()
                                        .when(self.active_response_view == ResponseView::Raw, |this| this.bg(colors.surface).text_color(colors.blue))
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.active_response_view = ResponseView::Raw;
                                            this.response_body_input.update(cx, |input, cx| input.set_highlighter(Language::Plain, cx));
                                            cx.notify();
                                        }))
                                )
                        )
                        .child(
                            v_flex()
                                .flex_1()
                                .p_4()
                                .child(
                                    div()
                                        .flex_1()
                                        .bg(colors.bg) // 与请求框统一背景以确保高亮一致
                                        .border_1()
                                        .border_color(colors.border)
                                        .p_1()
                                        .rounded_md()
                                        .child(Input::new(&self.response_body_input).size_full())
                                )
                        )
                        .into_any_element()
                }
                ResponseTab::Headers => {
                    v_flex().flex_1().p_4().overflow_y_scrollbar()
                        .children(resp.headers.iter().map(|h| {
                            h_flex().gap_2().py_1().border_b_1().border_color(colors.border)
                                .child(div().w_48().child(Label::new(h.key.clone()).text_color(colors.subtext)))
                                .child(div().flex_1().child(Label::new(h.value.clone()).text_color(colors.text)))
                        }))
                        .into_any_element()
                }
            }
        } else {
            v_flex()
                .size_full()
                .items_center()
                .justify_center()
                .gap_4()
                .child(
                    Icon::new(IconName::Globe)
                        .size_12()
                        .text_color(colors.subtext)
                )
                .child(
                    Label::new("No response yet. Enter URL and click Send.")
                        .text_color(colors.subtext)
                )
                .into_any_element()
        };

        v_flex()
            .flex_1()
            .size_full()
            .bg(colors.bg)
            .child(
                // Header / URL Bar
                h_flex()
                    .px_4()
                    .py_2()
                    .border_b_1()
                    .border_color(colors.border)
                    .gap_3()
                    .child(
                        Button::new("method-dropdown")
                            .label(format!("{:?}", self.request.method))
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
                    .child(Input::new(&self.url_input).flex_1())
                    .child(
                        Button::new("send")
                            .primary()
                            .icon(IconName::ArrowRight)
                            .label(if self.loading { "Sending..." } else { "Send" })
                            .disabled(self.loading)
                            .on_click({
                                let view = view.clone();
                                move |_, _, cx| {
                                    view.update(cx, |this, cx| {
                                        this.update_request_state(cx);
                                        let request = this.request.clone();
                                        this.loading = true;
                                        this.response = None;
                                        cx.notify();
                                        cx.emit(request); 
                                    }).ok();
                                }
                            })
                    )
            )
            .child(
                div()
                    .flex_1()
                    .size_full()
                    .child(
                        v_resizable("main-split")
                            .child(
                                resizable_panel()
                                    .size(px(450.))
                                    .min_size(px(100.))
                                    .child(
                                        v_flex()
                                            .size_full()
                                            .child(
                                                TabBar::new("request-tabs").w_full()
                                                    .child(Tab::new().label("Params").underline().selected(self.active_tab == RequestTab::Params).when(self.active_tab == RequestTab::Params, |this| this.bg(colors.surface).shadow_sm().rounded_md()).on_click(cx.listener(|this, _, _, cx| this.select_tab(RequestTab::Params, cx))))
                                                    .child(Tab::new().label("Headers").underline().selected(self.active_tab == RequestTab::Headers).when(self.active_tab == RequestTab::Headers, |this| this.bg(colors.surface).shadow_sm().rounded_md()).on_click(cx.listener(|this, _, _, cx| this.select_tab(RequestTab::Headers, cx))))
                                                    .child(Tab::new().label("Body").underline().selected(self.active_tab == RequestTab::Body).when(self.active_tab == RequestTab::Body, |this| this.bg(colors.surface).shadow_sm().rounded_md()).on_click(cx.listener(|this, _, _, cx| this.select_tab(RequestTab::Body, cx))))
                                                    .child(Tab::new().label("Auth").underline().selected(self.active_tab == RequestTab::Auth).when(self.active_tab == RequestTab::Auth, |this| this.bg(colors.surface).shadow_sm().rounded_md()).on_click(cx.listener(|this, _, _, cx| this.select_tab(RequestTab::Auth, cx))))
                                            )
                                            .child(
                                                v_flex()
                                                    .flex_1()
                                                    .p_4()
                                                    .child(request_content)
                                            )
                                    )
                            )
                            .child(
                                resizable_panel()
                                    .flex_1()
                                    .min_size(px(150.))
                                    .child(
                                        v_flex()
                                            .size_full()
                                            .border_t_1()
                                            .border_color(colors.border)
                                            .child(
                                                TabBar::new("response-tabs").w_full()
                                                    .child(Tab::new().label("Body").underline().selected(self.active_response_tab == ResponseTab::Body).when(self.active_response_tab == ResponseTab::Body, |this| this.bg(colors.surface).shadow_sm().rounded_md()).on_click(cx.listener(|this, _, _, cx| { this.active_response_tab = ResponseTab::Body; cx.notify(); })))
                                                    .child(Tab::new().label("Headers").underline().selected(self.active_response_tab == ResponseTab::Headers).when(self.active_response_tab == ResponseTab::Headers, |this| this.bg(colors.surface).shadow_sm().rounded_md()).on_click(cx.listener(|this, _, _, cx| { this.active_response_tab = ResponseTab::Headers; cx.notify(); })))
                                                    .suffix(
                                                        if let Some(resp) = &self.response {
                                                            h_flex().gap_4().items_center().px_4()
                                                                .child(
                                                                    h_flex().gap_1().items_center()
                                                                        .child(Icon::new(if resp.status_code < 400 { IconName::CircleCheck } else { IconName::CircleX }).small().text_color(if resp.status_code < 400 { colors.green } else { colors.red }))
                                                                        .child(Label::new(format!("{} {}", resp.status_code, resp.status_text)).text_color(if resp.status_code < 400 { colors.green } else { colors.red }))
                                                                )
                                                                .child(
                                                                    h_flex().gap_1().items_center()
                                                                        .child(Icon::new(IconName::Info).small().text_color(colors.subtext))
                                                                        .child(Label::new(format!("{} ms", resp.elapsed_ms)).text_color(colors.subtext))
                                                                )
                                                                .child(
                                                                    h_flex().gap_1().items_center()
                                                                        .child(Icon::new(IconName::Inbox).small().text_color(colors.subtext))
                                                                        .child(Label::new(format!("{} bytes", resp.size)).text_color(colors.subtext))
                                                                )
                                                        } else {
                                                            div()
                                                        }
                                                    )
                                            )
                                            .child(
                                                v_flex()
                                                    .flex_1()
                                                    .child(response_content)
                                            )
                                    )
                            )
                    )
            )
    }
}

struct Hiposter {
    tabs: Vec<Entity<ApiTab>>,
    active_tab_index: usize,
    history: Vec<HistoryItem>,
    _subscriptions: Vec<Subscription>,
    theme: AppTheme,
}

impl Hiposter {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut this = Self {
            tabs: Vec::new(),
            active_tab_index: 0,
            history: Vec::new(),
            _subscriptions: Vec::new(),
            theme: AppTheme::default(),
        };
        
        this.load_history();
        this.load_theme();
        
        // Initial setup for theme and highlighter
        let theme = this.theme;
        this.set_theme(theme, _window, cx);

        this
    }

    fn add_tab(&mut self, request: model::HttpRequest, window: &mut Window, cx: &mut Context<Self>) {
        let tab = cx.new(|cx| ApiTab::with_request(request, self.theme, window, cx));
        let sub = cx.subscribe(&tab, |this, tab, request: &model::HttpRequest, cx| {
            this.execute_request(tab, request.clone(), cx);
        });
        self._subscriptions.push(sub);

        let obs = cx.observe(&tab, |_, _, cx| {
            cx.notify();
        });
        self._subscriptions.push(obs);

        self.tabs.push(tab);
        self.active_tab_index = self.tabs.len() - 1;
        cx.notify();
    }

    fn close_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        self.tabs.remove(index);
        if self.active_tab_index >= self.tabs.len() && !self.tabs.is_empty() {
            self.active_tab_index = self.tabs.len() - 1;
        }
        cx.notify();
    }

    fn execute_request(&mut self, tab: Entity<ApiTab>, request: model::HttpRequest, cx: &mut Context<Self>) {
        self.add_history(request.clone());
        
        cx.spawn(move |_this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let mut cx = cx.clone();
            let request = request.clone();
            async move {
                let result = http::execute_request(&request).await;
                let _ = tab.update(&mut cx, |tab, cx| {
                    tab.loading = false;
                    match result {
                        Ok(resp) => {
                            tab.response = Some(resp);
                        }
                        Err(e) => {
                            tab.response = Some(model::HttpResponse {
                                status_code: 0,
                                status_text: format!("Error: {}", e),
                                ..Default::default()
                            });
                        }
                    }
                    cx.notify();
                });
            }
        }).detach();
    }

    
    fn load_theme(&mut self) {
        if let Ok(data) = fs::read_to_string("theme_config.json") {
            if let Ok(theme) = serde_json::from_str(&data) {
                self.theme = theme;
            }
        }
    }

    fn save_theme(&self) {
        if let Ok(data) = serde_json::to_string(&self.theme) {
            let _ = fs::write("theme_config.json", data);
        }
    }

    fn set_theme(&mut self, theme: AppTheme, window: &mut Window, cx: &mut Context<Self>) {
        self.theme = theme;
        let colors = theme.colors();

        cx.update_global::<gpui_component::theme::Theme, _>(|global_theme, cx| {
            // Generate a dynamic highlight theme configuration based on the current palette
            let config = serde_json::json!({
                "name" : theme.name(),
                "mode" : "light",
                "colors" : {
                    "background" : Self::hsla_to_hex(colors.bg),
                    "foreground" : Self::hsla_to_hex(colors.text),
                    "primary" : Self::hsla_to_hex(colors.blue),
                    "primary.foreground" : "#ffffff",
                    "border" : Self::hsla_to_hex(colors.border),
                    "accent.background" : format!("{}1a", Self::hsla_to_hex(colors.blue))
                },
                "highlight" : {
                    "editor.background" : Self::hsla_to_hex(colors.bg),
                    "editor.foreground" : Self::hsla_to_hex(colors.text),
                    "syntax" : {
                        "property" : { "color" : "#8250DF" },
                        "label" : { "color" : "#8250DF" },
                        "tag" : { "color" : "#8250DF" },
                        "variable" : { "color" : "#8250DF" },
                        "attribute" : { "color" : "#8250DF" },
                        "variable.other.member" : { "color" : "#8250DF" },
                        "string" : { "color" : "#0A7D28" },
                        "number" : { "color" : "#0550AE" },
                        "boolean" : { "color" : "#CF222E" },
                        "constant" : { "color" : "#6A737D", "font_style" : "italic" },
                        "null" : { "color" : "#6A737D", "font_style" : "italic" },
                        "keyword" : { "color" : "#CF222E" },
                        "operator" : { "color" : "#999999" },
                        "punctuation" : { "color" : "#999999" },
                        "punctuation.bracket" : { "color" : "#999999" },
                        "punctuation.delimiter" : { "color" : "#999999" }
                    }
                }
            });

            if let Ok(config) = serde_json::from_value::<gpui_component::theme::ThemeConfig>(config) {
                global_theme.apply_config(&std::rc::Rc::new(config));
                // Crucial: Trigger a theme change to re-initialize active colors and highlighters from the new config
                gpui_component::theme::Theme::change(gpui_component::theme::ThemeMode::Light, None, cx);
            }
        });

        for tab in &self.tabs {
            tab.update(cx, |t, cx| {
                t.theme = theme;
                
                // FORCE RE-PARSE: By touching the values with the window reference, 
                // we force the editor to re-render using the new theme colors immediately.
                let body_val = t.body_input.read(cx).value().to_string();
                t.body_input.update(cx, |input, cx| {
                    let lang = if t.request.content_type == "application/json" { Language::Json } else { Language::Plain };
                    input.set_value(body_val, window, cx);
                    input.set_highlighter(Language::Plain, cx);
                    input.set_highlighter(lang, cx);
                });
                
                let res_val = t.response_body_input.read(cx).value().to_string();
                let res_view = t.active_response_view;
                t.response_body_input.update(cx, |input, cx| {
                    let lang = if res_view == ResponseView::Pretty { Language::Json } else { Language::Plain };
                    input.set_value(res_val, window, cx);
                    input.set_highlighter(Language::Plain, cx);
                    input.set_highlighter(lang, cx);
                });

                cx.notify();
            });
        }
        self.save_theme();
        cx.notify();
    }

    fn hsla_to_hex(color: Hsla) -> String {
        // Convert HSLA to SRGBA then to hex
        let rgb = color.to_rgb();
        format!("#{:02x}{:02x}{:02x}", 
            (rgb.r * 255.0) as u8, 
            (rgb.g * 255.0) as u8, 
            (rgb.b * 255.0) as u8
        )
    }

    fn load_history(&mut self) {
        if let Ok(data) = fs::read_to_string("history.json") {
            if let Ok(history) = serde_json::from_str(&data) {
                self.history = history;
            }
        }
    }

    fn save_history(&self) {
        if let Ok(data) = serde_json::to_string(&self.history) {
            let _ = fs::write("history.json", data);
        }
    }

    fn add_history(&mut self, request: model::HttpRequest) {
        self.history.insert(0, HistoryItem {
            request,
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        });
        self.history.truncate(50);
        self.save_history();
    }

    fn clear_all_history(&mut self, cx: &mut Context<Self>) {
        self.history.clear();
        self.save_history();
        cx.notify();
    }
}

impl Render for Hiposter {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = self.theme.colors();
        let view = cx.weak_entity();

        if self.tabs.is_empty() {
            self.add_tab(model::HttpRequest::default(), window, cx);
        }

        v_flex()
            .size_full()
            .bg(colors.bg)
            .child(
                TitleBar::new()
                    .child(div().flex_1()) // Spacer to push theme to the right
                    .child(
                        h_flex()
                            .px_3()
                            .child(
                                Button::new("theme-dropdown")
                                    .label(format!("Theme: {}", self.theme.name()))
                                    .ghost()
                                    .small()
                                    .dropdown_caret(true)
                                    .dropdown_menu({
                                        let view = view.clone();
                                        move |menu, _, _| {
                                            let themes = [
                                                AppTheme::GitHubLight,
                                                AppTheme::SolarizedLight,
                                                AppTheme::OneLight,
                                                AppTheme::VitesseLight,
                                                AppTheme::CatppuccinLatte,
                                            ];
                                            let mut menu = menu;
                                            for t in themes {
                                                let view = view.clone();
                                                menu = menu.item(PopupMenuItem::new(t.name()).on_click(move |_, window, cx| {
                                                    view.update(cx, |this, cx| {
                                                        this.set_theme(t, window, cx);
                                                    }).ok();
                                                }));
                                            }
                                            menu
                                        }
                                    })
                            )
                    )
            )
            .child(
                h_flex()
                    .flex_1()
                    .size_full()
                    .child(
                        h_resizable("global-split")
                            .child(
                                resizable_panel()
                                    .size(px(280.))
                                    .min_size(px(150.))
                                    .child(
                                        // Left Sidebar: History
                                        v_flex()
                                            .size_full()
                                            .bg(colors.sidebar)
                                            .border_r_1()
                                            .border_color(colors.border)
                                            .child(
                                                v_flex()
                                                    .border_b_1()
                                                    .border_color(colors.border)
                                                    .child(
                                                        h_flex()
                                                            .justify_between()
                                                            .items_center()
                                                            .p_3()
                                                            .child(
                                                                h_flex()
                                                                    .gap_2()
                                                                    .items_center()
                                                                    .child(Icon::new(IconName::Inbox).small().text_color(colors.text))
                                                                    .child(Label::new("History").text_color(colors.text))
                                                            )
                                                            .child(
                                                                Button::new("clear-history")
                                                                    .icon(CustomIconName::Trash)
                                                                    .ghost()
                                                                    .small()
                                                                    .on_click(cx.listener(|this, _, _, cx| {
                                                                        this.clear_all_history(cx);
                                                                    }))
                                                            )
                                                    )
                                            )
                                            .child(
                                                v_flex()
                                                    .flex_1()
                                                    .overflow_y_scrollbar()
                                                    .children(self.history.iter().enumerate().map(|(i, item)| {
                                                        let url = item.request.url.clone();
                                                        let method = item.request.method.clone();
                                                        let request = item.request.clone();
                                                        h_flex()
                                                            .id(("history-item", i))
                                                            .group("history-item")
                                                            .p_2()
                                                            .cursor_pointer()
                                                            .hover(|s| s.bg(colors.surface))
                                                            .on_click(cx.listener(move |this, _, window, cx| {
                                                                this.add_tab(request.clone(), window, cx);
                                                            }))
                                                            .child(
                                                                h_flex()
                                                                    .gap_2()
                                                                    .items_center()
                                                                    .child(
                                                                        Icon::new(match method {
                                                                            model::HttpMethod::GET => IconName::ArrowDown,
                                                                            model::HttpMethod::POST => IconName::ArrowUp,
                                                                            _ => IconName::Info,
                                                                        })
                                                                        .small()
                                                                        .text_color(match method {
                                                                            model::HttpMethod::GET => colors.green,
                                                                            model::HttpMethod::POST => colors.yellow,
                                                                            _ => colors.text,
                                                                        })
                                                                    )
                                                                    .child(
                                                                        Label::new(format!("{:?}", method))
                                                                            .text_color(match method {
                                                                                model::HttpMethod::GET => colors.green,
                                                                                model::HttpMethod::POST => colors.yellow,
                                                                                _ => colors.text,
                                                                            })
                                                                            .w_12()
                                                                    )
                                                            )
                                                            .child(Label::new(url).text_color(colors.text).ml_2().flex_1())
                                                            .child(
                                                                div()
                                                                    .invisible()
                                                                    .group_hover("history-item", |s| s.visible())
                                                                    .child(
                                                                        Button::new(format!("del-hist-{}", i))
                                                                            .icon(IconName::Close)
                                                                            .ghost()
                                                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                                                cx.stop_propagation();
                                                                                this.history.remove(i);
                                                                                this.save_history();
                                                                                cx.notify();
                                                                            }))
                                                                    )
                                                            )
                                                    }))
                                            )
                                    )
                            )
                            .child(
                                resizable_panel()
                                    .flex_1()
                                    .child(
                                        v_flex()
                                            .size_full()
                                            .child(
                                                // Tab Bar
                                                h_flex()
                                                    .h(px(34.))
                                                    .bg(colors.sidebar)
                                                    .border_b_1()
                                                    .border_color(colors.border)
                                                    .child(
                                                        h_flex()
                                                            .flex_1()
                                                            .h_full()
                                                            .overflow_hidden()
                                                            .children(self.tabs.iter().enumerate().map(|(i, tab)| {
                                                                let is_active = i == self.active_tab_index;
                                                                let tab_title = tab.read(cx).url_input.read(cx).value().to_string();
                                                                let tab_title = if tab_title.is_empty() { "New Request".to_string() } else { tab_title };
                                                                
                                                                h_flex()
                                                                    .id(("tab", i))
                                                                    .flex_1()
                                                                    .min_w(px(60.))
                                                                    .max_w(px(180.))
                                                                    .h_full()
                                                                    .px_3()
                                                                    .border_r_1()
                                                                    .border_color(colors.border)
                                                                    .cursor_pointer()
                                                                    .bg(if is_active { colors.surface } else { colors.sidebar })
                                                                    .on_click(cx.listener(move |this, _, _, cx| {
                                                                        this.active_tab_index = i;
                                                                        cx.notify();
                                                                    }))
                                                                    .child(
                                                                        h_flex()
                                                                            .flex_1()
                                                                            .gap_2()
                                                                            .items_center()
                                                                            .overflow_hidden()
                                                                            .child(Icon::new(IconName::Globe).small().text_color(if is_active { colors.blue } else { colors.subtext }))
                                                                            .child(
                                                                                Label::new(tab_title)
                                                                                    .text_color(if is_active { colors.blue } else { colors.subtext })
                                                                            )
                                                                    )
                                                                    .child(
                                                                        div()
                                                                            .ml_1()
                                                                            .child(
                                                                                Button::new(format!("close-tab-{}", i))
                                                                                    .icon(IconName::Close)
                                                                                    .ghost()
                                                                                    .small()
                                                                                    .on_click(cx.listener(move |this, _, _, cx| {
                                                                                        this.close_tab(i, cx);
                                                                                    }))
                                                                            )
                                                                    )
                                                                    .when(is_active, |this| {
                                                                        this.border_b_2().border_color(colors.blue)
                                                                    })
                                                            }))
                                                    )
                                                    .child(
                                                        h_flex()
                                                            .flex_none()
                                                            .h_full()
                                                            .px_1()
                                                            .border_l_1()
                                                            .border_color(colors.border)
                                                            .items_center()
                                                            .bg(colors.sidebar)
                                                            .child(
                                                                Button::new("add-tab")
                                                                    .icon(IconName::Plus)
                                                                    .ghost()
                                                                    .on_click(cx.listener(|this, _, window, cx| {
                                                                        this.add_tab(model::HttpRequest::default(), window, cx);
                                                                    }))
                                                            )
                                                    )
                                            )
                                            .child(
                                                // Active Tab Content
                                                if let Some(tab) = self.tabs.get(self.active_tab_index) {
                                                    div().flex_1().bg(colors.bg).child(tab.clone())
                                                } else {
                                                    div().flex_1().bg(colors.bg)
                                                }
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

    let app = gpui_platform::application().with_assets(AppAssets);

    app.run(move |cx| {
        gpui_component::init(cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(1400.), px(900.)), cx)),
            titlebar: Some(TitleBar::title_bar_options()),
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
