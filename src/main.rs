use nannou::prelude::*;
struct Model {}

/// Style information for a VU graph.
#[derive(Debug)]
pub struct VuStyle {
    color_on: Rgba,
    color_off: Rgba,
    line_width: f32
}
impl Default for VuStyle {
    fn default() -> Self {
        let sg : Rgb = SPRINGGREEN.into_format();
        VuStyle {
            color_on: rgba(sg.red, sg.green, sg.blue, 1.0),
            color_off: rgba(0.0,0.0,0.0, 1.0),
            line_width: 2.0
        }
    }
}
/// Draw a VU graph, adding it to `draw`.
fn make_vu_graph(style: &VuStyle, 
                  percent: f32, 
                 draw: &nannou::Draw, 
                 rect: nannou::geom::Rect) {
    let width = rect.w();
    let gap_width = width / 100.0;
    let line_width = f32::min(gap_width, style.line_width); 
    for i in 0..100 {
        let if32 = i as f32;
        let x = rect.left() + (gap_width * if32) + (style.line_width / 2.0);
        let start_point = pt2(x, rect.top());
        let end_point = pt2(x, rect.bottom());
        if if32 < percent.floor() {
            draw.line()
                .start(start_point)
                .end(end_point)
                .weight(line_width)
                .color(style.color_on)
                .finish();
        } else if if32 >= percent.ceil() {
            draw.line()
                .start(start_point)
                .end(end_point)
                .weight(line_width)
                .color(style.color_off)
                .finish();
        } else {
            draw.line()
                .start(start_point)
                .end(end_point)
                .weight(line_width)
                .color(style.color_off)
                .finish();
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
    draw.rect().color(WHITE).wh(server.wh()).xy(server.xy());
    let sin_wave = app.time.sin();
    let p = map_range(sin_wave, -1.0, 1.0, 0.0, 100.0);
    make_vu_graph(&VuStyle{line_width: 4.0, 
        color_off: rgba(1.0,0.1,0.1,0.5), ..Default::default()}, 
        p, &draw, server);
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
