use nannou::prelude::*;

mod vu_graph;
mod utils;
mod sparklines;

struct Model {}


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
    vu_graph::make_vu_graph(
        &vu_graph::VuStyle {
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
    vu_graph::make_vu_graph(
        &vu_graph::VuStyle {
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
    sparklines::make_sparklines(
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
