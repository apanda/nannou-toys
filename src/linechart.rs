
//! Draw linecharts in Nannou.
//! 
//! Styles are provided by LineChartStyle.
use super::{label_stack, sparklines, text_label};
use nannou::{draw::Draw, geom::Rect};

/// Style information for a line chart
#[derive(Debug)]
pub struct LineChartStyle {
    /// A vector of styles used to draw each of the lines.
    pub line_styles: Vec<sparklines::SparkLineStyle>,
    /// Labels used when drawing a legend.
    pub labels: Vec<String>,
    /// A boolean indicating whether a legend should be drawn.
    pub legend: bool,
}

/// Draw a line chart using values from `values`, starting out at `index`.
/// Similar to the [sparklines](sparklines) each series is treated as a ring buffer,
/// i.e., indices wrap around starting at index.
/// The line chart (including index) fits within `rect`.
pub fn make_linechart<'a, I: IntoIterator<Item = &'a [f32]>>(
    style: &LineChartStyle,
    values: I,
    index: usize,
    ymin: f32,
    ymax: f32,
    draw: &Draw,
    rect: Rect,
) {
    let rect = if style.legend {
        let lstyle = label_stack::StackStyle {
            styles: style
                .line_styles
                .iter()
                .map(|s| text_label::LabelStyle {
                    color: s.color,
                    ..Default::default()
                })
                .collect(),
            padding: 2.0,
        };
        let lrect =
            label_stack::calculate_extents(style.labels.iter().map(|s| &s[..]), &lstyle, rect);
        let trect = lrect.align_right_of(rect);
        label_stack::make_label_stack(style.labels.iter().map(|s| &s[..]), &lstyle, draw, trect);
        rect.pad_right(trect.w())
    } else {
        rect
    };
    for (vals, style) in values.into_iter().zip(style.line_styles.iter()) {
        sparklines::make_sparklines(style, vals, index, ymin, ymax, draw, rect);
    }
}
