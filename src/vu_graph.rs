//! VU graph module.

use nannou::prelude::*;

/// Style information for a VU graph.
#[derive(Debug)]
pub struct VuStyle {
    pub color_on: Rgba,
    pub color_off: Rgba,
    pub line_width: f32,
}
impl Default for VuStyle {
    fn default() -> Self {
        let sg: Rgb = SPRINGGREEN.into_format();
        VuStyle {
            color_on: rgba(sg.red, sg.green, sg.blue, 1.0),
            color_off: rgba(0.0, 0.0, 0.0, 1.0),
            line_width: 2.0,
        }
    }
}
/// Draw a VU graph, adding it to `draw`.
pub fn make_vu_graph(style: &VuStyle, percent: f32, draw: &nannou::Draw, rect: nannou::geom::Rect) {
    let width = rect.w();
    let gap_width = width / 100.0;
    let line_width = f32::min(gap_width, style.line_width);
    for i in 0..100 {
        let i_f32 = i as f32;
        let x = rect.left() + (gap_width * i_f32) + (style.line_width / 2.0);
        let start_point = pt2(x, rect.top());
        let end_point = pt2(x, rect.bottom());
        if i_f32 < percent.floor() {
            draw.line()
                .start(start_point)
                .end(end_point)
                .weight(line_width)
                .color(style.color_on)
                .finish();
        } else {
            draw.line()
                .start(start_point)
                .end(end_point)
                .weight(line_width)
                .color(style.color_off)
                .finish();
            // Draw partial line
            if i_f32 < percent.ceil() {
                let on_width = line_width * (percent - percent.floor());
                let shift = Vec2::X * ((line_width - on_width) / 2.0);
                draw.line()
                    .start(start_point - shift)
                    .end(end_point - shift)
                    .weight(on_width)
                    .color(style.color_on)
                    .finish();
            }
        }
    }
}
