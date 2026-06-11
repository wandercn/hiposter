use gpui::prelude::*;
use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants},
    label::Label,
    v_flex,
    Icon,
    Sizable,
};
use crate::assets::CustomIconName;

pub struct AboutWindow;

impl AboutWindow {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for AboutWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<gpui_component::theme::Theme>();
        let bg_color = theme.colors.background;
        let text_color = theme.colors.foreground;
        let subtext_color = theme.colors.muted_foreground;

        v_flex()
            .size_full()
            .bg(bg_color)
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                div()
                    .w_32()
                    .h_32()
                    .bg(theme.colors.primary)
                    .rounded_xl()
                    .shadow_lg()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        Icon::new(CustomIconName::Trash)
                            .size(rems(4.0))
                            .text_color(theme.colors.primary_foreground)
                    )
            )
            .child(
                v_flex()
                    .items_center()
                    .gap_1()
                    .child(
                        Label::new("HiPoster")
                            .text_size(rems(1.5))
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(text_color)
                    )
                    .child(
                        Label::new("Version 0.1.0")
                            .text_color(subtext_color)
                    )
            )
            .child(
                v_flex()
                    .items_center()
                    .gap_1()
                    .mt_4()
                    .child(
                        Label::new("Developed by wander")
                            .text_color(subtext_color)
                            .text_size(rems(0.875))
                    )
                    .child(
                        Button::new("github-link")
                            .label("GitHub Repository")
                            .ghost()
                            .small()
                            .on_click(|_, _, cx| {
                                cx.open_url("https://github.com/wandercn/hiposter");
                            })
                    )
            )
    }
}

