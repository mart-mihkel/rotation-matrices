use std::f32::consts::TAU;

use sfml::graphics::{RenderWindow, Color, RenderTarget, PrimitiveType, Vertex, RenderStates};
use sfml::window::{Style, Event, Key};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 400;

const X_ORIGIN: f32 = WIDTH as f32 / 2.;
const Y_ORIGIN: f32 = HEIGHT as f32 / 2.;

const CROSS_SECTION_RADIUS: f32 = 10.;
const REVOLUTION_RADIUS: f32 = 15.;

const OBJECT_DISTANCE: f32 = CROSS_SECTION_RADIUS + REVOLUTION_RADIUS + 5.;
const PROJECTION_DISTANCE: f32 = WIDTH as f32 * OBJECT_DISTANCE * 3. / (8. * (CROSS_SECTION_RADIUS * REVOLUTION_RADIUS));

fn main() {
    let mut window = RenderWindow::new(
        (WIDTH, HEIGHT),
        "Rotating solid of revolution",
        Style::DEFAULT,
        &Default::default(),
    );

    window.set_framerate_limit(60);

    let mut vertices = [Vertex::default(); 2_usize.pow(12)];

    let (mut yaw_angle, mut roll_angle): (f32, f32) = (0., 0.);
    while window.is_open() {
        // event loop
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => window.close(),
                Event::KeyPressed { code: Key::P, .. } => {
                    while let Some(event) = window.wait_event() {
                        if let Event::KeyReleased { code: Key::P, .. } = event {
                            break;
                        }
                    }
                },
                _ => {}
            }
        }

        // rendering
        window.clear(Color::BLACK);

        let (yaw_angle_sin, yaw_angle_cos) = yaw_angle.sin_cos();
        let (roll_angle_sin, roll_angle_cos) = roll_angle.sin_cos();

        // solid loop
        let mut vertex_i = 0;
        let mut solid_angle = 0.;
        while solid_angle < TAU {
            let (solid_sin, solid_cos) = solid_angle.sin_cos();

            // revolution loop
            let mut pitch_angle = 0.;
            while pitch_angle < TAU {
                let (pitch_sin, pitch_cos) = pitch_angle.sin_cos();

                let v0 = REVOLUTION_RADIUS + CROSS_SECTION_RADIUS * solid_cos;
                let v1 = CROSS_SECTION_RADIUS * yaw_angle_cos * solid_sin;
                let (x, y, z) = (
                    v0 * (roll_angle_cos * pitch_cos + yaw_angle_sin * roll_angle_sin * pitch_sin) - v1 * roll_angle_sin,
                    v0 * (pitch_cos * yaw_angle_sin - roll_angle_cos * yaw_angle_sin * pitch_sin) + v1 * roll_angle_cos,
                    v0 * yaw_angle_cos * pitch_sin + CROSS_SECTION_RADIUS * yaw_angle_sin * solid_sin,
                );

                let projection_multiplier = PROJECTION_DISTANCE / (z + OBJECT_DISTANCE);

                vertices[vertex_i].position.x = X_ORIGIN + x * projection_multiplier;
                vertices[vertex_i].position.y = Y_ORIGIN - y * projection_multiplier;

                let luminance = pitch_cos * solid_cos * roll_angle_sin - yaw_angle_cos * solid_cos * pitch_sin - yaw_angle_sin * solid_sin + roll_angle_cos * (yaw_angle_cos * solid_sin - solid_cos * yaw_angle_sin * pitch_sin);

                vertices[vertex_i].color = if luminance > 0. { Color::WHITE } else { Color::BLACK };

                vertex_i += 1;
                pitch_angle += 0.1;
            }

            solid_angle += 0.1;
        }

        window.draw_primitives(
            &vertices[..vertex_i],
            PrimitiveType::LINES,
            &RenderStates::DEFAULT,
        );
        window.display();

        yaw_angle += 0.03;
        roll_angle += 0.03;
    }
}
