use gpui::*;

pub struct TextInput {
    focus_handle: FocusHandle,
    content: String,
    placeholder: SharedString,
}

impl TextInput {
    pub fn new(cx: &mut Context<Self>, placeholder: impl Into<SharedString>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            content: String::new(),
            placeholder: placeholder.into(),
        }
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn set_content(&mut self, content: String, cx: &mut Context<Self>) {
        self.content = content;
        cx.notify();
    }

    fn on_key_down(&mut self, event: &KeyDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let key = &event.keystroke.key;
        match key.as_str() {
            "backspace" => {
                self.content.pop();
            }
            "space" => {
                self.content.push(' ');
            }
            "enter" => {
                // Handle enter if needed
            }
            _ => {
                if let Some(key_char) = &event.keystroke.key_char {
                    self.content.push_str(key_char);
                } else if key.len() == 1 {
                    self.content.push_str(key);
                }
            }
        }
        cx.notify();
    }
}

impl Render for TextInput {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focused = self.focus_handle.is_focused(window);
        
        div()
            .id(cx.entity_id().to_string())
            .flex_1()
            .px_2()
            .py_1()
            .bg(rgb(0x3b4252))
            .rounded_md()
            .border_1()
            .border_color(if focused { rgb(0x81a1c1) } else { rgb(0x4c566a) })
            .track_focus(&self.focus_handle)
            .focusable()
            .on_key_down(cx.listener(Self::on_key_down))
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, window, cx| {
                window.focus(&this.focus_handle, cx);
            }))
            .child(
                div()
                    .flex()
                    .items_center()
                    .child(
                        if self.content.is_empty() {
                            div()
                                .text_color(rgb(0x4c566a))
                                .child(self.placeholder.clone())
                        } else {
                            div()
                                .text_color(rgb(0xd8dee9))
                                .child(self.content.clone())
                        }
                    )
                    .child(
                        if focused {
                            div()
                                .ml_0p5()
                                .w_px()
                                .h_4()
                                .bg(rgb(0x81a1c1))
                        } else {
                            div()
                        }
                    )
            )
    }
}

impl Focusable for TextInput {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
