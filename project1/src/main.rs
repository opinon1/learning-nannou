use std::f32::MIN;

use nannou::prelude::*;
use nannou_egui::*;

use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand::Rng;

use std::cmp::Ord;

struct Model {
    universe: Universe,
    egui: Egui,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let rect = Rect::from_w_h(1000.0, 1000.0);
    let window_id = app
        .new_window()
        .size(rect.w() as u32, rect.h() as u32)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    let egui = Egui::from_window(&app.window(window_id).unwrap());
    let universe = Universe::new(1000, rect.right(), rect.top());
    Model { universe, egui }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let win = app.window_rect();

    model.universe.step(win.right(), win.top());

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        // Resolution slider
        ui.label("gravity");
        ui.add(egui::Slider::new(
            &mut model.universe.gravity,
            0.0001..=50.0f32,
        ));

        // Scale slider
        ui.label("dt");
        ui.add(egui::Slider::new(&mut model.universe.dt, 0.000001..=3.0));

        // Rotation slider
        ui.label("size:");
        ui.add(egui::Slider::new(&mut model.universe.radius, 0.2f32..=10.0));

        // Random color button
        let clicked = ui.button("reset").clicked();

        if clicked {
            let win = app.window_rect();
            model.universe.reset(win.right(), win.top());
        }
    });
}
fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    model.universe.display(&draw);

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

struct Body {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    mass: f32,
    radius: f32,
}

impl Body {
    fn new(pos: Vec2, vel: Vec2, radius: f32) -> Self {
        Self {
            pos,
            vel,
            acc: Vec2::ZERO,
            mass: radius * radius * 3.1415,
            radius,
        }
    }
}

struct Universe {
    bodies: Vec<Body>,
    radius: f32,
    velocity: f32,
    gravity: f32,
    dt: f32,
    rgen: ThreadRng,
}

impl Universe {
    fn new(n: usize, top: f32, right: f32) -> Self {
        let mut rgen = thread_rng();

        let mut bodies = Vec::with_capacity(n);
        let velocity = 5f32;
        let dt = 0.05f32;
        let gravity = 5.05;
        let radius = 1f32;

        //SUN
        bodies.push(Body::new(
            Vec2::new(0f32, 0.0f32),
            Vec2::new(0f32, 0f32),
            20f32,
        ));

        //planets:
        for _ in (0..n).skip(1) {
            let pos: Vec2 = Vec2::new(rgen.gen_range((-right)..right), rgen.gen_range((-top)..top));
            let vel: Vec2 = Vec2::new(
                rgen.gen_range(-velocity..velocity),
                rgen.gen_range(-velocity..velocity),
            );
            bodies.push(Body::new(pos, vel, radius));
        }
        Self {
            bodies,
            radius,
            gravity,
            velocity,
            dt,
            rgen,
        }
    }
    fn reset(&mut self, right: f32, top: f32) {
        let n = self.bodies.len();

        self.bodies.clear();

        self.bodies.push(Body::new(
            Vec2::new(0f32, 0.0f32),
            Vec2::new(0f32, 0f32),
            20f32,
        ));

        //planets:
        for _ in (0..n).skip(1) {
            let pos: Vec2 = Vec2::new(
                self.rgen.gen_range(-right..right),
                self.rgen.gen_range(-top..top),
            );
            let vel: Vec2 = Vec2::new(
                self.rgen.gen_range(-self.velocity..self.velocity),
                self.rgen.gen_range(-self.velocity..self.velocity),
            );
            self.bodies.push(Body::new(pos, vel, self.radius));
        }
    }

    fn step(&mut self, right: f32, top: f32) {
        self.update_acc();
        for i in 0..self.bodies.len() {
            let dt = self.dt / 10.0;
            let mut vel = self.bodies[i].vel + self.bodies[i].acc * self.gravity * dt;

            let mut pos = self.bodies[i].pos + vel * dt;

            if pos.x.abs() > right {
                vel.x *= -0.9;
                pos.x *= 0.99;
            }
            if pos.y.abs() > top {
                vel.y *= -0.9;
                pos.y *= 0.99;
            }

            self.bodies[i].pos = pos;
            self.bodies[i].vel = vel;
        }
    }

    fn update_acc(&mut self) {
        for i in 0..self.bodies.len() {
            self.bodies[i].acc = Vec2::ZERO;
        }
        for i in 0..self.bodies.len() {
            let p1 = self.bodies[i].pos;
            let m1 = self.bodies[i].mass;
            let r1 = self.bodies[i].radius;
            for j in (i + 1)..self.bodies.len() {
                if j != i {
                    let p2 = self.bodies[j].pos;
                    let m2 = self.bodies[j].mass;
                    let r2 = self.bodies[j].radius;

                    let r = p2 - p1;
                    let mag_q = r.x * r.x + r.y * r.y;
                    let mag = mag_q.sqrt();

                    //detect collision
                    if mag <= r2 + r1 {
                        self.collision(i, j, mag, (r1 + r2) - mag);
                    } else {
                        let tmp = r / (mag_q.max(MIN) * mag);

                        self.bodies[i].acc += m2 * tmp;
                        self.bodies[j].acc -= m1 * tmp;
                    }
                }
            }
        }
    }
    fn collision(&mut self, i1: usize, i2: usize, mag: f32, depth: f32) {
        let body1 = &self.bodies[i1];
        let body2 = &self.bodies[i2];

        if mag == 0.0 {
            return; // Prevent division by zero in normalization
        }

        let norm = (body2.pos - body1.pos) / mag; // Direction vector from body1 to body2
        let relvel = body2.vel - body1.vel;
        let vel_along_norm = relvel.dot(norm); // Dot product to find component of velocity along normal

        if vel_along_norm > 0.0 {
            return; // They are moving apart already
        }

        let restitution = 0.7; // Coefficient of restitution (0 - inelastic, 1 - perfectly elastic)
        let mut impulse_magnitude = -(1.0 + restitution) * vel_along_norm;
        impulse_magnitude /= 1.0 / body1.mass + 1.0 / body2.mass;

        let impulse = norm * impulse_magnitude;

        //let pos_correction_factor = 0.5; // How to split the penetration correction
        let total_inverse_mass = 1.0 / body1.mass + 1.0 / body2.mass;

        let n_pos1 = norm * (depth * (1.0 / body1.mass) / total_inverse_mass);
        let n_pos2 = norm * (depth * (1.0 / body2.mass) / total_inverse_mass);

        let n_vel1 = impulse / body1.mass;
        let n_vel2 = impulse / body2.mass;

        // Correction to push them apart
        self.bodies[i1].pos -= n_pos1;
        self.bodies[i2].pos += n_pos2;
        // Apply impulse
        self.bodies[i1].vel -= n_vel1;
        self.bodies[i2].vel += n_vel2;
    }

    fn display(&self, draw: &Draw) {
        for body in self.bodies.iter() {
            let r = (body.vel.x.powi(2) + body.vel.y.powi(2)).sqrt();
            let r: u8 = u8::min((r) as u8, 255u8);
            let b = (body.acc.x.powi(2) + body.acc.y.powi(2)).sqrt();
            let b: u8 = u8::min((b * 40.0) as u8, 255u8);
            draw.ellipse()
                .w_h(body.radius * 2f32, body.radius * 2f32)
                .xy(body.pos)
                .color(rgb(255, 255 - b, 255 - r));
        }
    }
}
