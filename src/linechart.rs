use super::{label_stack, sparklines, text_label};
use nannou::{draw::Draw, geom::Rect};

// Style information for a line chart
#[derive(Debug)]
pub struct LineChartStyle {
    pub line_styles: Vec<sparklines::SparkLineStyle>,
    pub labels: Vec<String>,
    pub legend: bool,
}

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
