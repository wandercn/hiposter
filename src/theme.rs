use gpui::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AppTheme {
    GitHubLight,
    SolarizedLight,
    OneLight,
    VitesseLight,
    CatppuccinLatte,
    NordLight,
    GruvboxLight,
    AyuLight,
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
    pub fn colors(&self) -> ThemeColors {
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
            AppTheme::NordLight => ThemeColors {
                bg: Hsla::from(rgb(0xe5e9f0)),
                sidebar: Hsla::from(rgb(0xd8dee9)),
                surface: Hsla::from(rgb(0xeceff4)),
                border: Hsla::from(rgb(0xc0c8d4)),
                text: Hsla::from(rgb(0x2e3440)),
                subtext: Hsla::from(rgb(0x4c566a)),
                blue: Hsla::from(rgb(0x5e81ac)),
                green: Hsla::from(rgb(0xa3be8c)),
                yellow: Hsla::from(rgb(0xebcb8b)),
                red: Hsla::from(rgb(0xbf616a)),
            },
            AppTheme::GruvboxLight => ThemeColors {
                bg: Hsla::from(rgb(0xfbf1c7)),
                sidebar: Hsla::from(rgb(0xebdbb2)),
                surface: Hsla::from(rgb(0xf2e5bc)),
                border: Hsla::from(rgb(0xd5c4a1)),
                text: Hsla::from(rgb(0x3c3836)),
                subtext: Hsla::from(rgb(0x7c6f64)),
                blue: Hsla::from(rgb(0x458588)),
                green: Hsla::from(rgb(0x98971a)),
                yellow: Hsla::from(rgb(0xd79921)),
                red: Hsla::from(rgb(0xcc241d)),
            },
            AppTheme::AyuLight => ThemeColors {
                bg: Hsla::from(rgb(0xfafafa)),
                sidebar: Hsla::from(rgb(0xf3f4f5)),
                surface: Hsla::from(rgb(0xf8f9fa)),
                border: Hsla::from(rgb(0xe6e6e6)),
                text: Hsla::from(rgb(0x5c6166)),
                subtext: Hsla::from(rgb(0x8a9199)),
                blue: Hsla::from(rgb(0x399ee6)),
                green: Hsla::from(rgb(0x86b300)),
                yellow: Hsla::from(rgb(0xfa8d3e)),
                red: Hsla::from(rgb(0xf07178)),
            },
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            AppTheme::GitHubLight => "GitHub Light",
            AppTheme::SolarizedLight => "Solarized Light",
            AppTheme::OneLight => "One Light",
            AppTheme::VitesseLight => "Vitesse Light",
            AppTheme::CatppuccinLatte => "Catppuccin Latte",
            AppTheme::NordLight => "Nord Light",
            AppTheme::GruvboxLight => "Gruvbox Light",
            AppTheme::AyuLight => "Ayu Light",
        }
    }
}

impl Default for AppTheme {
    fn default() -> Self {
        AppTheme::VitesseLight
    }
}

pub fn hsla_to_hex(color: Hsla) -> String {
    let rgb = color.to_rgb();
    format!("#{:02x}{:02x}{:02x}", 
        (rgb.r * 255.0) as u8, 
        (rgb.g * 255.0) as u8, 
        (rgb.b * 255.0) as u8
    )
}
