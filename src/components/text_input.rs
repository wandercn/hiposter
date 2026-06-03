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
        match event.keystroke.key.as_str() {
            "backspace" => {
                self.content.pop();
            }
            "enter" => {
                // Handle enter if needed
            }
            key if key.len() == 1 => {
                self.content.push_str(key);
            }
            _ => {
                // Check if it's a character input through key_char
                if let Some(key_char) = &event.keystroke.key_char {
                    if key_char.len() == 1 {
                         self.content.push_str(key_char);
                    }
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
            .flex_1()
            .px_2()
            .py_1()
            .bg(rgb(0x3b4252))
            .rounded_md()
            .border_1()
            .border_color(if focused { rgb(0x81a1c1) } else { rgb(0x3b4252) })
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::on_key_down))
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, window, cx| {
                window.focus(&this.focus_handle, cx);
            }))
            .child(
                if self.content.is_empty() {
                    div().text_color(rgb(0x4c566a)).child(self.placeholder.clone())
                } else {
                    div().child(self.content.clone())
                }
            )
    }
}
