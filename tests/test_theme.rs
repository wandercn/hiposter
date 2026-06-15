use hiposter_gpui::theme::*;
use gpui::Hsla;

#[test]
fn test_theme_names() {
    assert_eq!(AppTheme::GitHubLight.name(), "GitHub Light");
    assert_eq!(AppTheme::Monokai.name(), "Monokai");
}

#[test]
fn test_dark_mode_detection() {
    assert!(AppTheme::Monokai.is_dark());
    assert!(AppTheme::OceanicNext.is_dark());
    assert!(!AppTheme::GitHubLight.is_dark());
    assert!(!AppTheme::OneLight.is_dark());
}

#[test]
fn test_hsla_to_hex() {
    // White
    let white = Hsla { h: 0.0, s: 0.0, l: 1.0, a: 1.0 };
    assert_eq!(hsla_to_hex(white).to_lowercase(), "#ffffff");

    // Black
    let black = Hsla { h: 0.0, s: 0.0, l: 0.0, a: 1.0 };
    assert_eq!(hsla_to_hex(black).to_lowercase(), "#000000");
}
