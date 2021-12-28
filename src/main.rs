use nannou::prelude::*;
struct Model {}

/// Style information for a VU graph.
#[derive(Debug)]
pub struct VuStyle {
    color_on: Rgba,
    color_off: Rgba,
    line_width: f32,
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
fn make_vu_graph(style: &VuStyle, percent: f32, draw: &nannou::Draw, rect: nannou::geom::Rect) {
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

pub struct RingIterator<'a, T> {
    slice: &'a [T],
    index: usize,
    len: usize,
    visited: usize,
}

pub fn ring<'a, T>(slice: &'a [T], index: usize) -> RingIterator<'a, T> {
    RingIterator {
        slice,
        index,
        len: slice.len(),
        visited: 0,
    }
}

impl<'a, T> Iterator for RingIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.visited == self.len {
            None
        } else {
            let index = self.index;
            self.index = (self.index + 1) % self.len;
            self.visited += 1;
            Some(&self.slice[index])
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len - self.visited, Some(self.len - self.visited))
    }
}

/// Style information for a spark line.
#[derive(Debug)]
pub struct SparkLineStyle {
    color: Rgba,
    line_width: f32,
}
impl Default for SparkLineStyle {
    fn default() -> Self {
        let gold: Rgb = GOLD.into_format();
        SparkLineStyle {
            color: rgba(gold.red, gold.green, gold.blue, 1.0),
            line_width: 2.0,
        }
    }
}

fn make_sparklines(
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
    let points = ring(values, index).enumerate().map(|(x, y)| {
        (
            pt2(
                (x as f32) * gap_width + rect.left(),
                (y - ymin) * y_scale + rect.bottom(),
            ),
            style.color,
        )
    });
    draw.polyline()
        .weight(style.line_width)
        .points_colored(points)
        .finish();
}

fn model(_: &App) -> Model {
    Model {}
}
fn update(_: &App, _: &mut Model, _: Update) {}

fn view(app: &App, _: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let win = app.window_rect();
    let text = Rect::from_w_h(500.0, 60.0).top_left_of(win.pad(25.0));
    draw.text("Hello")
        .color(WHITE)
        .font_size(32)
        .align_text_middle_y()
        .xy(text.xy())
        .wh(text.wh());
    let server = text.below(text).shift_y(10.0);
    let sin_wave = app.time.sin();
    let p = map_range(sin_wave, -1.0, 1.0, 0.0, 100.0);
    draw.rect().color(WHITE).wh(server.wh()).xy(server.xy());
    make_vu_graph(
        &VuStyle {
            line_width: 4.0,
            color_off: rgba(1.0, 0.1, 0.1, 0.5),
            ..Default::default()
        },
        p,
        &draw,
        server,
    );

    let small_vu = Rect::from_w_h(250.0, 20.0)
        .below(server)
        .shift_y(-3.0)
        .align_left_of(server);
    let cos_wave = (app.time / 4.0).cos();
    let p = map_range(cos_wave, -1.0, 1.0, 0.0, 100.0);
    make_vu_graph(
        &VuStyle {
            line_width: 2.0,
            color_on: rgba(0.1, 0.1, 1.0, 0.5),
            color_off: rgba(1.0, 0.1, 0.1, 0.5),
        },
        p,
        &draw,
        small_vu,
    );

    let small_spark = Rect::from_w_h(250.0, 20.0)
        .below(small_vu)
        .shift_y(-3.0)
        .align_left_of(small_vu);
    let v = (0..50).map(|i| (i as f32).sin()).collect::<Vec<_>>();
    make_sparklines(
        &Default::default(),
        &v,
        ((app.duration.since_start.as_millis() / 100) as usize) % v.len(),
        -1.0,
        1.0,
        &draw,
        small_spark,
    );

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
