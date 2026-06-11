use gpui::prelude::*;
use gpui::*;
use gpui_component::{
    button::*,
    label::Label,
    menu::{DropdownMenu, PopupMenuItem},
    resizable::*,
    scroll::ScrollableElement,
    highlighter::Language,
    *,
};
use crate::model;
use crate::theme::{AppTheme, ThemeColors, hsla_to_hex};
use crate::assets::{CustomIconName};
use crate::api_tab::ApiTab;
use crate::http;
use std::fs;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HistoryItem {
    pub request: model::HttpRequest,
    pub timestamp: u64,
}

pub struct Hiposter {
    pub tabs: Vec<Entity<ApiTab>>,
    pub active_tab_index: usize,
    pub history: Vec<HistoryItem>,
    pub _subscriptions: Vec<Subscription>,
    pub theme: AppTheme,
}

impl Hiposter {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut this = Self {
            tabs: Vec::new(),
            active_tab_index: 0,
            history: Vec::new(),
            _subscriptions: Vec::new(),
            theme: AppTheme::default(),
        };
        
        this.load_history();
        this.load_theme();
        
        let theme = this.theme;
        this.set_theme(theme, _window, cx);

        this
    }

    pub fn add_tab(&mut self, request: model::HttpRequest, window: &mut Window, cx: &mut Context<Self>) {
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

    pub fn close_tab(&mut self, index: usize, cx: &mut Context<Self>) {
        self.tabs.remove(index);
        if self.active_tab_index >= self.tabs.len() && !self.tabs.is_empty() {
            self.active_tab_index = self.tabs.len() - 1;
        }
        cx.notify();
    }

    pub fn execute_request(&mut self, tab: Entity<ApiTab>, request: model::HttpRequest, cx: &mut Context<Self>) {
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

    pub fn load_theme(&mut self) {
        if let Ok(data) = fs::read_to_string("theme_config.json") {
            if let Ok(theme) = serde_json::from_str(&data) {
                self.theme = theme;
            }
        }
    }

    pub fn save_theme(&self) {
        if let Ok(data) = serde_json::to_string(&self.theme) {
            let _ = fs::write("theme_config.json", data);
        }
    }

    pub fn set_theme(&mut self, theme: AppTheme, window: &mut Window, cx: &mut Context<Self>) {
        self.theme = theme;
        let colors = theme.colors();

        cx.update_global::<gpui_component::theme::Theme, _>(|global_theme, cx| {
            let config = serde_json::json!({
                "name" : theme.name(),
                "mode" : "light",
                "colors" : {
                    "background" : hsla_to_hex(colors.bg),
                    "foreground" : hsla_to_hex(colors.text),
                    "primary" : hsla_to_hex(colors.blue),
                    "primary.foreground" : "#ffffff",
                    "border" : hsla_to_hex(colors.border),
                    "accent.background" : format!("{}1a", hsla_to_hex(colors.blue))
                },
                "highlight" : {
                    "editor.background" : hsla_to_hex(colors.bg),
                    "editor.foreground" : hsla_to_hex(colors.text),
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
                gpui_component::theme::Theme::change(gpui_component::theme::ThemeMode::Light, None, cx);
            }
        });

        for tab in &self.tabs {
            tab.update(cx, |t, cx| {
                t.theme = theme;
                
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
                    let lang = if res_view == crate::api_tab::ResponseView::Pretty { Language::Json } else { Language::Plain };
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

    pub fn load_history(&mut self) {
        if let Ok(data) = fs::read_to_string("history.json") {
            if let Ok(history) = serde_json::from_str(&data) {
                self.history = history;
            }
        }
    }

    pub fn save_history(&self) {
        if let Ok(data) = serde_json::to_string(&self.history) {
            let _ = fs::write("history.json", data);
        }
    }

    pub fn add_history(&mut self, request: model::HttpRequest) {
        self.history.insert(0, HistoryItem {
            request,
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        });
        self.history.truncate(50);
        self.save_history();
    }

    pub fn clear_all_history(&mut self, _cx: &mut Context<Self>) {
        self.history.clear();
        self.save_history();
    }

    fn render_sidebar(&self, colors: &ThemeColors, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex().size_full().bg(colors.sidebar).border_r_1().border_color(colors.border)
            .child(v_flex().border_b_1().border_color(colors.border).child(
                h_flex().justify_between().items_center().p_3()
                    .child(h_flex().gap_2().items_center().child(Icon::new(IconName::Inbox).small().text_color(colors.text)).child(Label::new("History").text_color(colors.text)))
                    .child(Button::new("clear-history").icon(CustomIconName::Trash).ghost().small().on_click(cx.listener(|this, _, _, cx| this.clear_all_history(cx))))
            ))
            .child(v_flex().flex_1().overflow_y_scrollbar().children(self.history.iter().enumerate().map(|(i, item)| {
                let method = item.request.method.clone();
                let request = item.request.clone();
                h_flex().id(("history-item", i)).group("history-item").p_2().cursor_pointer().hover(|s| s.bg(colors.surface)).on_click(cx.listener(move |this, _, window, cx| {
                    this.add_tab(request.clone(), window, cx);
                }))
                .child(
                    h_flex().gap_2().items_center()
                        .child(Icon::new(match method {
                            model::HttpMethod::GET => IconName::ArrowDown,
                            model::HttpMethod::POST => IconName::ArrowUp,
                            _ => IconName::Info,
                        }).small().text_color(match method {
                            model::HttpMethod::GET => colors.green,
                            model::HttpMethod::POST => colors.yellow,
                            _ => colors.text,
                        }))
                        .child(Label::new(format!("{:?}", method)).text_color(match method {
                            model::HttpMethod::GET => colors.green,
                            model::HttpMethod::POST => colors.yellow,
                            _ => colors.text,
                        }).w_12())
                )
                .child(Label::new(item.request.url.clone()).text_color(colors.text).ml_2().flex_1())
                .child(
                    div().invisible().group_hover("history-item", |s| s.visible())
                        .child(Button::new(format!("del-hist-{}", i)).icon(IconName::Close).ghost().on_click(cx.listener(move |this, _, _, cx| {
                            cx.stop_propagation();
                            this.history.remove(i);
                            this.save_history();
                            cx.notify();
                        })))
                )
            })))
    }

    fn render_tab_bar(&self, colors: &ThemeColors, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex().h(px(34.)).bg(colors.sidebar).border_b_1().border_color(colors.border)
            .child(
                h_flex().flex_1().h_full().overflow_hidden().children(self.tabs.iter().enumerate().map(|(i, tab)| {
                    let is_active = i == self.active_tab_index;
                    let tab_title = tab.read(cx).url_input.read(cx).value().to_string();
                    let tab_title = if tab_title.is_empty() { "New Request".to_string() } else { 
                        if tab_title.len() > 30 { format!("...{}", &tab_title[tab_title.len()-27..]) } else { tab_title }
                    };
                    
                    h_flex().id(("tab", i)).flex_1().min_w(px(60.)).max_w(px(180.)).h_full().px_3().border_r_1().border_color(colors.border).cursor_pointer().bg(if is_active { colors.surface } else { colors.sidebar }).on_click(cx.listener(move |this, _, _, cx| {
                        this.active_tab_index = i;
                        cx.notify();
                    }))
                    .child(
                        h_flex().flex_1().gap_2().items_center().overflow_hidden()
                            .child(Icon::new(IconName::Globe).small().text_color(if is_active { colors.blue } else { colors.subtext }))
                            .child(Label::new(tab_title).text_color(if is_active { colors.blue } else { colors.subtext }))
                    )
                    .child(
                        div().ml_1()
                            .child(Button::new(format!("close-tab-{}", i)).icon(IconName::Close).ghost().small().on_click(cx.listener(move |this, _, _, cx| {
                                this.close_tab(i, cx);
                            })))
                    )
                    .when(is_active, |this| this.border_b_2().border_color(colors.blue))
                }))
            )
            .child(
                h_flex().flex_none().h_full().px_1().border_l_1().border_color(colors.border).items_center().bg(colors.sidebar)
                    .child(Button::new("add-tab").icon(IconName::Plus).ghost().on_click(cx.listener(|this, _, window, cx| {
                        this.add_tab(model::HttpRequest::default(), window, cx);
                    })))
            )
    }
}

impl Render for Hiposter {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = self.theme.colors();
        let view = cx.weak_entity();

        if self.tabs.is_empty() {
            self.add_tab(model::HttpRequest::default(), window, cx);
        }

        v_flex().size_full().bg(colors.bg)
            .child(
                TitleBar::new()
                    .child(div().flex_1())
                    .child(
                        h_flex().px_3().child(
                            Button::new("theme-dropdown").label(format!("Theme: {}", self.theme.name())).ghost().small().dropdown_caret(true)
                                .dropdown_menu({
                                    let view = view.clone();
                                    move |menu, _, _| {
                                        let themes = [AppTheme::GitHubLight, AppTheme::SolarizedLight, AppTheme::OneLight, AppTheme::VitesseLight, AppTheme::CatppuccinLatte];
                                        let mut menu = menu;
                                        for t in themes {
                                            let view = view.clone();
                                            menu = menu.item(PopupMenuItem::new(t.name()).on_click(move |_, window, cx| {
                                                view.update(cx, |this, cx| { this.set_theme(t, window, cx); }).ok();
                                            }));
                                        }
                                        menu
                                    }
                                })
                        )
                    )
            )
            .child(
                h_flex().flex_1().size_full()
                    .child(
                        h_resizable("global-split")
                            .child(resizable_panel().size(px(280.)).min_size(px(150.)).child(self.render_sidebar(&colors, cx)))
                            .child(
                                resizable_panel().flex_1().child(
                                    v_flex().size_full()
                                        .child(self.render_tab_bar(&colors, cx))
                                        .child(
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
