use nannou::prelude::*;
use rand::thread_rng;
use rand::Rng;

struct Model {
    polyfractal: PolyFractal,
}

fn main() {
    nannou::app(model)
        .event(event)
        .simple_window(view)
        .update(update)
        .run();
}

fn model(_app: &App) -> Model {
    let polyfractal = PolyFractal::new();
    Model { polyfractal }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.polyfractal.step();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    model.polyfractal.display(&draw);

    draw.to_frame(app, &frame).unwrap();
}

struct PolyFractal {
    points: Vec<Vec2>,
}

impl PolyFractal {
    fn new() -> Self {
        let mut points = Vec::new();

        let radius: f32 = 300.0;

        for i in (0..=360).step_by(72) {
            let radian = deg_to_rad(i as f32);
            let x = radian.sin() * radius;
            let y = radian.cos() * radius;
            points.push(Vec2::new(x, y));
        }
        Self { points }
    }
    fn step(&mut self) {
        let mut randg = thread_rng();
        let p1 = &self.points[self.points.len() - 1];
        let rand_i: usize = randg.gen_range(0..5);
        let p2 = &self.points[rand_i];

        self.points
            .push(vec2(p1.x + p2.x, p1.y + p2.y) * 0.333_333_333f32)
    }
    fn display(&self, draw: &Draw) {
        for p in self.points.iter() {
            let _ = &draw.ellipse().xy(*p).color(BLACK).w_h(2f32, 2f32);
        }
    }
}
