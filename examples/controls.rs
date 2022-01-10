use nannou::prelude::*;
use nannou_toys::*;
struct Model {
    rate: f64,
}

fn model(app: &App) -> Model {
    app.new_window()
        .received_character(received_char)
        .build()
        .unwrap();
    Model { rate: 1.0 }
}

fn received_char(app: &App, model: &mut Model, c: char) {
    match c {
        'q' => app.quit(),
        '+' => model.rate += 0.5,
        '-' => model.rate -= 0.5,
        _ => println!("Huh?"),
    };
}

fn update(_: &App, _: &mut Model, _: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let win = app.window_rect();
    let rate = format!("Rate: {:.2}", model.rate);

    let text = text_label::make_label(&rate, &Default::default(), &draw, win);
    let server = Rect::from_w_h(500.0, 60.0)
        .top_left_of(win.pad_top(text.h() + 4.0))
        .align_left_of(text);

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
        -0.2,
        0.2,
        &draw,
        small_spark,
    );
    let lstack = Rect::from_w_h(250.0, 120.0)
        .below(small_spark)
        .align_left_of(small_spark)
        .shift_y(-3.0);
    let lstack_actual = label_stack::make_label_stack(
        ["food", "fo", "bar"],
        &label_stack::StackStyle {
            styles: vec![Default::default(), Default::default(), Default::default()],
            padding: 4.0,
        },
        &draw,
        lstack,
    );
    let lab2 = Rect::from_w_h(250.0, 20.0)
        .below(lstack_actual)
        .align_left_of(lstack);
    let lab2 = text_label::make_label(
        &"Border",
        &text_label::LabelStyle {
            color: rgba(1.0, 0.0, 0.0, 1.0),
            ..Default::default()
        },
        &draw,
        lab2,
    );

    let line_style = linechart::LineChartStyle {
        line_styles: vec![
            sparklines::SparkLineStyle {
                color: rgba(0.3, 0.3, 0.3, 0.8),
                ..Default::default()
            },
            sparklines::SparkLineStyle {
                points: true,
                ..Default::default()
            },
            sparklines::SparkLineStyle {
                points: true,
                color: rgba(0.0, 0.0, 1.0, 1.0),
                ..Default::default()
            },
        ],
        labels: vec!["origin".to_string(), "sin".to_string(), "cos".to_string()],
        legend: true,
    };

    let origin = vec![0.0; 10];
    let v0 = (0..10)
        .map(|i| (std::f32::consts::PI * 0.5 * (i as f32)).sin())
        .collect::<Vec<_>>();
    let w0 = (0..10)
        .map(|i| (std::f32::consts::PI * 0.5 * (i as f32)).cos())
        .collect::<Vec<_>>();
    let line_rect = Rect::from_w_h(300.0, 100.0).below(lab2).align_left_of(lab2);
    linechart::make_linechart(
        &line_style,
        vec![&origin[..], &v0, &w0],
        0,
        -1.3,
        1.3,
        &draw,
        line_rect,
    );
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).view(view).run();
}
