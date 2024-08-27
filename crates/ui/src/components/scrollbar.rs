use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// _This is a placeholder for a general scrollbar implementation._
///
/// # Scrollbar
///
/// A scrollbar is a graphical control element that allows users to navigate through content quickly by dragging a thumb.
///
/// ## Anatomy
///
/// `Thumb`: The part of the scrollbar that the user can drag. It represents the current view position, but may not be exactly the same size as the visible content for very large content.
///
/// `Track`: The background of the scrollbar. The thumb moves along the track to represent the current view position.
///
/// `Arrow Buttons`: The buttons at the ends of the scrollbar that the user can click to scroll by a small amount ("a page").
///
/// Scrollbars in Zed do not currently have arrow buttons.
#[allow(dead_code)]
struct Scrollbar;

/// When to show a scrollbar.
///
/// Default: always
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScrollbarVisibility {
    #[default]
    /// Always show the scrollbar.
    Always,
    /// Show when the mouse is over the project panel.
    OnHover,
    /// Never show the scrollbar.
    Never,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScrollbarStyle {
    // The scrollbar takes up space, and offsets the content.
    //
    // Usually paired with
    #[default]
    Block,
    // The scrollbar overlays the content. Usually with no track background to prevent obscuring content.
    Overlay,
}

impl ScrollbarStyle {
    pub fn from_visibility(visibility: ScrollbarVisibility) -> Self {
        match visibility {
            ScrollbarVisibility::OnHover => ScrollbarStyle::Overlay,
            _ => ScrollbarStyle::Block,
        }
    }
}
