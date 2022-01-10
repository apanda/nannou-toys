use super::utils;
use nannou::prelude::*;

/// Style information for a spark line.
#[derive(Debug)]
pub struct SparkLineStyle {
    pub color: Rgba,
    pub line_width: f32,
    pub points: bool,
    pub point_width: f32,
}
impl Default for SparkLineStyle {
    fn default() -> Self {
        let gold: Rgb = GOLD.into_format();
        SparkLineStyle {
            color: rgba(gold.red, gold.green, gold.blue, 1.0),
            line_width: 2.0,
            points: false,
            point_width: 6.0,
        }
    }
}

pub fn make_sparklines(
    style: &SparkLineStyle,
    values: &[f32],
    index: usize,
    ymin: f32,
    ymax: f32,
    draw: &nannou::Draw,
    rect: nannou::geom::Rect,
) {
    let width = rect.w();
    let height = rect.h();
    let gap_width = width / values.len() as f32;
    let y_scale = height / (ymax - ymin);
    let points = utils::ring(values, index)
        .enumerate()
        .map(|(x, y)| {
            let y = f32::max(f32::min(*y, ymax), ymin);
            (
                pt2(
                    (x as f32) * gap_width + rect.left(),
                    (y - ymin) * y_scale + rect.bottom(),
                ),
                style.color,
            )
        })
        .collect::<Vec<_>>();
    if style.points {
        for (pt, color) in points.iter() {
            draw.ellipse()
                .color(*color)
                .w_h(style.point_width, style.point_width)
                .x_y(pt.x, pt.y)
                .finish();
        }
    }
    draw.polyline()
        .weight(style.line_width)
        .points_colored(points)
        .finish();
}
