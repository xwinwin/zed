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
struct Scrollbar {
    style: ScrollbarStyle,
}

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
pub enum ScrollbarDisplayStyle {
    /// The scrollbar takes up space, and offsets the content.
    ///
    /// Usually paired with [ScrollbarVisibility::Always]
    #[default]
    Block,
    /// The scrollbar overlays the content. Usually with no track background to prevent obscuring content.
    ///
    /// Usually paired with [ScrollbarVisibility::OnHover]
    Overlay,
}

impl From<ScrollbarVisibility> for ScrollbarDisplayStyle {
    fn from(visibility: ScrollbarVisibility) -> Self {
        match visibility {
            ScrollbarVisibility::OnHover => ScrollbarDisplayStyle::Overlay,
            _ => ScrollbarDisplayStyle::Block,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ScrollbarStyle {
    thumb: ScrollbarThumbStyle,
    track: ScrollbarTrackStyle,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ScrollbarThumbStyle {
    /// The color of the thumb.
    color: String,
    /// The opacity of the thumb.
    ///
    /// Overrides the opacity of [Self::color] if set.
    opacity: f32,
    /// The color of the thumb when hovered.
    ///
    /// Overrides the color of [Self::color] and [Self::opacity]
    /// if set and the mouse is over the thumb.
    hover_color: String,
    /// The opacity of the thumb when hovered.
    ///
    /// Overrides [Self::opacity] and the alpha of [Self::color]
    /// if set and the mouse is over the thumb.
    hover_opacity: f32,
    /// The color of the thumb's border.
    /// Setting a color will make the border visible.
    ///
    /// Defaults to a transparent value.
    border_color: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ScrollbarTrackStyle {
    /// The color of the scrollbar track.
    color: String,
    /// The color of the scrollbar track's border, if one is visible.
    /// Setting this to a fully transparent value will hide the border.
    ///
    /// Defaults to the theme's muted border color.
    border_color: String,
    /// The opacity of the scrollbar track.
    ///
    /// Overrides the opacity of [Self::color] if set.
    opacity: f32,
}
