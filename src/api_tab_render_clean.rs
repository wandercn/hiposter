    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.weak_entity();
        
        if let Some(resp) = &self.response {
            let content = resp.body.clone();
            let display_content = if self.active_response_view == ResponseView::Pretty {
                if let Ok(v) = serde_json::from_str::<Value>(&content) {
                    serde_json::to_string_pretty(&v).unwrap_or(content)
                } else {
                    content
                }
            } else {
                content
            };
            
            if self.response_body_input.read(cx).value().to_string() != display_content {
                self.response_body_input.update(cx, |this, cx| {
                    this.set_value(display_content, window, cx);
                    this.set_highlighter(Language::Json, cx);
                });
            }
        }

        let request_content = match self.active_tab {
            RequestTab::Params => {
                v_flex().gap_3().child(
                    h_flex().justify_between().child(Label::new("Query Parameters").text_color(cx.theme().foreground))
                    .child(Button::new("add-param").label("+ Add Param").on_click(cx.listener(|this, _, window, cx| this.add_param(window, cx))))
                )
                .child(v_flex().gap_2().children(self.params.iter().enumerate().map(|(i, row)| {
                    h_flex().gap_2().child(Input::new(&row.key).flex_1()).child(Input::new(&row.value).flex_1())
                    .child(Button::new(format!("rem-p-{}", i)).label("X").on_click(cx.listener(move |this, _, _, cx| { this.remove_param(i, cx); cx.notify(); })))
                })))
                .into_any_element()
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
                .into_any_element()
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
                    )
                    .child(Input::new(&self.body_input).flex_1())
                    .into_any_element()
            }
            RequestTab::Auth => {
                v_flex().gap_4()
                    .child(
                        h_flex().gap_4().items_center()
                            .child(Label::new("Auth Type:").text_color(cx.theme().foreground))
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
                                this.child(Label::new("Token").text_color(cx.theme().foreground)).child(Input::new(&self.auth_token_input))
                            })
                            .when(self.request.auth.auth_type == model::AuthType::Basic, |this| {
                                this.child(Label::new("Username").text_color(cx.theme().foreground)).child(Input::new(&self.auth_username_input))
                                    .child(Label::new("Password").text_color(cx.theme().foreground)).child(Input::new(&self.auth_password_input))
                            })
                            .when(self.request.auth.auth_type == model::AuthType::None, |this| {
                                this.child(Label::new("No authentication required").text_color(cx.theme().muted_foreground))
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
                            h_flex().gap_4().items_center().mb_2()
                                .child(TabBar::new("res-view-tabs")
                                    .child(Tab::new().label("Pretty").selected(self.active_response_view == ResponseView::Pretty).on_click(cx.listener(|this, _, _, cx| { this.active_response_view = ResponseView::Pretty; cx.notify(); })))
                                    .child(Tab::new().label("Raw").selected(self.active_response_view == ResponseView::Raw).on_click(cx.listener(|this, _, _, cx| { this.active_response_view = ResponseView::Raw; cx.notify(); })))
                                )
                        )
                        .child(Input::new(&self.response_body_input).flex_1().disabled(true))
                        .into_any_element()
                }
                ResponseTab::Headers => {
                    v_flex().flex_1().overflow_y_scrollbar()
                        .children(resp.headers.iter().map(|h| {
                            h_flex().gap_2().py_1().border_b_1().border_color(cx.theme().border)
                                .child(div().w_48().child(Label::new(h.key.clone()).text_color(cx.theme().muted_foreground)))
                                .child(div().flex_1().child(Label::new(h.value.clone()).text_color(cx.theme().foreground)))
                        }))
                        .into_any_element()
                }
            }
        } else {
            v_flex()
                .child(
                    Label::new("No response yet. Enter URL and click Send.")
                        .text_color(cx.theme().muted_foreground)
                )
                .into_any_element()
        };

        v_flex()
            .flex_1()
            .size_full()
            .child(
                // Header / URL Bar
                h_flex()
                    .px_4()
                    .py_2()
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
                            .label(if self.loading { "Sending..." } else { "Send" })
                            .disabled(self.loading)
                            .on_click({
                                let view = view.clone();
                                move |_, _window, cx| {
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
                            .size_full()
                            .child(
                                resizable_panel()
                                    .size(px(320.))
                                    .min_size(px(100.))
                                    .child(
                                        v_flex()
                                            .size_full()
                                            .child(
                                                TabBar::new("request-tabs")
                                                    .child(Tab::new().label("Params").selected(self.active_tab == RequestTab::Params).on_click(cx.listener(|this, _, _, cx| this.select_tab(RequestTab::Params, cx))))
                                                    .child(Tab::new().label("Headers").selected(self.active_tab == RequestTab::Headers).on_click(cx.listener(|this, _, _, cx| this.select_tab(RequestTab::Headers, cx))))
                                                    .child(Tab::new().label("Body").selected(self.active_tab == RequestTab::Body).on_click(cx.listener(|this, _, _, cx| this.select_tab(RequestTab::Body, cx))))
                                                    .child(Tab::new().label("Auth").selected(self.active_tab == RequestTab::Auth).on_click(cx.listener(|this, _, _, cx| this.select_tab(RequestTab::Auth, cx))))
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
                                            .p_4()
                                            .border_t_1()
                                            .border_color(cx.theme().border)
                                            .child(
                                                h_flex().justify_between().items_center()
                                                    .child(
                                                        h_flex().gap_4().items_center()
                                                            .child(Label::new("Response").text_color(cx.theme().foreground))
                                                            .child(if let Some(resp) = &self.response {
                                                                h_flex().gap_4()
                                                                    .child(Label::new(format!("{} {}", resp.status_code, resp.status_text)).text_color(if resp.status_code < 400 { Hsla::from(rgb(0xa3be8c)) } else { Hsla::from(rgb(0xbf616a)) }))
                                                                    .child(Label::new(format!("{} ms", resp.elapsed_ms)).text_color(cx.theme().muted_foreground))
                                                                    .child(Label::new(format!("{} bytes", resp.size)).text_color(cx.theme().muted_foreground))
                                                            } else {
                                                                div()
                                                            })
                                                    )
                                                    .child(
                                                        TabBar::new("response-tabs")
                                                            .child(Tab::new().label("Body").selected(self.active_response_tab == ResponseTab::Body).on_click(cx.listener(|this, _, _, cx| { this.active_response_tab = ResponseTab::Body; cx.notify(); })))
                                                            .child(Tab::new().label("Headers").selected(self.active_response_tab == ResponseTab::Headers).on_click(cx.listener(|this, _, _, cx| { this.active_response_tab = ResponseTab::Headers; cx.notify(); })))
                                                    )
                                            )
                                            .child(
                                                v_flex()
                                                    .flex_1()
                                                    .mt_4()
                                                    .child(response_content)
                                            )
                                    )
                            )
                    )
            )
    }
