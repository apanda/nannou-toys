//! An abstraction to insert a stack of labels.
use super::text_label::{make_label, LabelStyle};
use super::utils::get_dimensions;
use nannou::draw::Draw;
use nannou::geom::Rect;
pub struct StackStyle {
    pub styles: Vec<LabelStyle>,
    pub padding: f32,
}

/// Calculate the dimensions of the label stack.
pub fn calculate_extents(labels: &[&str], style: &StackStyle, rect: Rect) -> Rect {
    let (w, h) = style
        .styles
        .iter()
        .zip(labels)
        .fold((0.0f32, 0.0f32), |(w, h), (style, text)| {
            let (w_n, h_n) = get_dimensions(text, style.font_size, rect);
            (w + w_n, h + h_n)
        });
    Rect::from_w_h(w, h).top_left_of(rect)
}

/// Draw a label stack with the given set of labels at `rect`.
pub fn make_label_stack(labels: &[&str], style: &StackStyle, draw: &Draw, rect: Rect) -> Rect {
    let left = style
        .styles
        .iter()
        .zip(labels)
        .fold(rect, |rect, (lstyle, text)| {
            let r_text = make_label(text, lstyle, draw, rect);
            rect.pad_top(r_text.h() + style.padding)
        });
    Rect::from_w_h(left.w(), f32::abs(rect.top() - left.top())).top_left_of(rect)
}
