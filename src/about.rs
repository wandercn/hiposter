use gpui::prelude::*;
use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants},
    label::Label,
    v_flex,
    Sizable,
};

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
                gpui::img("icons/logo.png")
                    .w_20()
                    .h_20()
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
                        Label::new("Version 0.1.2")
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
                            .icon(gpui_component::IconName::ExternalLink)
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

