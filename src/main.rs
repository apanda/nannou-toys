use nannou::prelude::*;
struct Model {}
fn model(_: &App) -> Model {
    Model {}
}
fn update(_: &App, _: &mut Model, _: Update) {}

fn view(app: &App, _: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let win = app.window_rect();
    let text = Rect::from_w_h(100.0, 60.0).
                top_left_of(win.pad(25.0));
    draw.text("Hello")
        .color(WHITE)
        .font_size(32)
        .align_text_middle_y()
        .xy(text.xy())
        .wh(text.wh());
    let server = text.below(text)
                     .shift_y(10.0);
    draw.rect()
        .color(rgb(0.6, 0.6, 0.6))
        .wh(server.wh())
        .xy(server.xy());
    let core = server.pad_left(-30.0);
    draw.rect()
        .color(rgb(0.7, 0.1, 0.1))
        .w_h(30.0, 10.0)
        .xy(core.xy());
    // let text = server.pad(10.0);
    // draw.text("Server")
    //     .color(BLACK)
    //     .font_size(18)
    //     .align_text_middle_y()
    //     .center_justify()
    //     .xy(text.xy())
    //     .wh(text.wh());
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
