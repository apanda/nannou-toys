//! Reusable text label: add text to a given rect.
use super::utils;
use nannou::color::{rgba, Rgba};
use nannou::geom::Rect;
use nannou::text::FontSize;
use nannou::Draw;

pub struct LabelStyle {
    pub color: Rgba,
    pub font_size: FontSize,
}

impl Default for LabelStyle {
    fn default() -> Self {
        LabelStyle {
            color: rgba(1.0, 1.0, 1.0, 1.0),
            font_size: 12,
        }
    }
}

/// Draw a left justified text label aligned to the left of `rect`.
/// Returns the actual `rect` occupied by the text. Note, graphs and
/// other things we draw, the text size is not determined by the input
/// `rect`.
pub fn make_label(t: &str, style: &LabelStyle, draw: &Draw, rect: Rect) -> Rect {
    let (w, h) = utils::get_dimensions(t, style.font_size, rect);
    let r = Rect::from_w_h(w, h).top_left_of(rect);
    draw.text(t)
        .color(style.color)
        .left_justify()
        .font_size(style.font_size)
        .xy(r.xy())
        .wh(r.wh())
        .finish();
    r
}
