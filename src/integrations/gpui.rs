#[cfg(feature = "gpui")]
use gpui::{AssetSource, SharedString, Svg};
#[cfg(feature = "gpui")]
use std::borrow::Cow;
#[cfg(feature = "gpui")]
use std::sync::atomic::{AtomicU64, Ordering};

#[cfg(feature = "gpui")]
use crate::animation::MorphController;
#[cfg(feature = "gpui")]
use crate::icons::Icon;

#[cfg(feature = "gpui")]
static FRAME_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Asset source for GPUI that loads dynamic MorpheusIcons SVG files from disk/memory.
#[cfg(feature = "gpui")]
pub struct MorpheusAssetSource;

#[cfg(feature = "gpui")]
impl AssetSource for MorpheusAssetSource {
    fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
        if let Ok(content) = std::fs::read(path) {
            Ok(Some(Cow::Owned(content)))
        } else {
            Ok(None)
        }
    }

    fn list(&self, _path: &str) -> anyhow::Result<Vec<SharedString>> {
        Ok(Vec::new())
    }
}

/// GPUI Integration for MorpheusIcons.
#[cfg(feature = "gpui")]
pub struct MorpheusGpui;

#[cfg(feature = "gpui")]
impl MorpheusGpui {
    /// Render an SVG path string `d="..."` as a GPUI `Svg` element.
    pub fn icon_svg(d_path: impl Into<String>) -> Svg {
        let full_svg = crate::integrations::svg::SvgRenderer::render_svg_document(
            &d_path.into(),
            24.0,
            24.0,
            "#3b82f6",
            2.0,
        );

        let frame = FRAME_COUNTER.fetch_add(1, Ordering::Relaxed);
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join(format!("morpheus_icon_{frame}.svg"));
        let _ = std::fs::write(&file_path, full_svg);

        let path_str = file_path.to_string_lossy().to_string();
        gpui::svg().path(SharedString::from(path_str))
    }

    /// Renders the current state of a `MorphController` as a GPUI `Svg` element.
    pub fn morph_svg(controller: &MorphController) -> Svg {
        Self::icon_svg(controller.current_svg_path())
    }

    /// Convenience method to create a static GPUI icon from the catalog.
    pub fn catalog_svg(icon: Icon) -> Svg {
        Self::icon_svg(icon.path_data())
    }
}
