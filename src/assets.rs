use gpui::*;
use gpui_component::*;
use gpui_component_assets::Assets as GpuiAssets;

pub const TRASH_2_SVG: &[u8] = br#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-trash2-icon lucide-trash-2"><path d="M10 11v6"/><path d="M14 11v6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"/><path d="M3 6h18"/><path d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>"#;

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

pub struct AppAssets;

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
