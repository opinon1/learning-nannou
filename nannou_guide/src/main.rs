use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect().pad(25.0);
    draw.background().color(WHITE);

    let square = Rect::from_w_h(100.0, 100.0).top_left_of(win);
    draw.rect()
        .xy(square.xy())
        .wh(square.wh())
        .color(PLUM)
        .z_degrees(45.0);

    let circle = square.below(square).shift_y(-25.0);
    draw.ellipse().xy(circle.xy()).wh(circle.wh()).color(SALMON);

    let start_point = pt2(-30.0, -20.0);
    let end_point = pt2(40.0, 40.0);
    //line
    draw.line()
        .start(start_point)
        .end(end_point)
        .weight(4.0)
        .color(STEELBLUE);

    //sine wave

    let points = (0..100).map(|i| {
        let x = i as f32 / 2f32 - 25.0; //subtract 25 to center the sine wave
        let point = pt2(x, x.sin()) * 20.0; //scale sine wave by 20.0
        (point, STEELBLUE)
    });
    draw.polyline().weight(3.0).points_colored(points);

    //draw to frame
    //
    //equilateral polygon
    let radius = 150.0;
    let points = (0..=360).step_by(120).map(|i| {
        let radian = deg_to_rad(i as f32 + 22.5f32);
        let x = radian.sin() * radius;
        let y = radian.cos() * radius;
        (pt2(x, y), STEELBLUE)
    });
    draw.polygon().points_colored(points);

    draw.to_frame(app, &frame).unwrap();
}
