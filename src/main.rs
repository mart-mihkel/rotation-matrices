use std::f32::consts::TAU;

use sfml::{
    graphics::{RenderWindow, CircleShape, Color, RenderTarget, Transformable},
    window::{Style, Event, Key},
    system::{Vector2f},
};

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 400;

const X_ORIGIN: f32 = SCREEN_WIDTH as f32 / 2.0;
const Y_ORIGIN: f32 = SCREEN_HEIGHT as f32 / 2.0;

const CROSS_SECTION_RADIUS: f32 = 10.0;
const REVOLUTION_RADIUS: f32 = 15.0;

const OBJECT_DISTANCE: f32 = CROSS_SECTION_RADIUS + REVOLUTION_RADIUS + 5.0;
const PROJECTION_DISTANCE: f32 = SCREEN_WIDTH as f32 * OBJECT_DISTANCE * 3.0 / (8.0 * (CROSS_SECTION_RADIUS * REVOLUTION_RADIUS));

fn main() {
    let mut window = RenderWindow::new(
        (SCREEN_WIDTH, SCREEN_HEIGHT),
        "Rotating solid of revolution",
        Style::DEFAULT,
        &Default::default(),
    );

    window.set_framerate_limit(60);

    let mut point = CircleShape::new(1.0, 4);
    let mut point_position = Vector2f::default();

    let (mut y_rotation_angle, mut z_rotation_angle): (f32, f32) = (0.0, 0.0);
    while window.is_open() {
        // event loop
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => window.close(),
                _ => {}
            }
        }

        // rendering
        window.clear(Color::BLACK);

        let (y_rotation_sin, y_rotation_cos) = y_rotation_angle.sin_cos();
        let (z_rotation_sin, z_rotation_cos) = z_rotation_angle.sin_cos();

        // solid loop
        let mut solid_angle = 0.0;
        while solid_angle < TAU {
            let (solid_sin, solid_cos) = solid_angle.sin_cos();

            // revolution and rotation loop
            let mut revolution_angle = 0.0;
            while revolution_angle < TAU {
                let (revolution_sin, revolution_cos) = revolution_angle.sin_cos();

                let luminance = revolution_cos * solid_cos * z_rotation_sin - y_rotation_cos * solid_cos * revolution_sin - y_rotation_sin * solid_sin + z_rotation_cos * (y_rotation_cos * solid_sin - solid_cos * y_rotation_sin * revolution_sin);

                if luminance > 0.0 {
                    let (x, y, z) = (
                        (REVOLUTION_RADIUS + CROSS_SECTION_RADIUS * solid_cos) * (z_rotation_cos * revolution_cos + y_rotation_sin * z_rotation_sin * revolution_sin) - (CROSS_SECTION_RADIUS * y_rotation_cos * solid_sin) * z_rotation_sin,
                        (REVOLUTION_RADIUS + CROSS_SECTION_RADIUS * solid_cos) * (revolution_cos * y_rotation_sin - z_rotation_cos * y_rotation_sin * revolution_sin) + (CROSS_SECTION_RADIUS * y_rotation_cos * solid_sin) * z_rotation_cos,
                        (REVOLUTION_RADIUS + CROSS_SECTION_RADIUS * solid_cos) * y_rotation_cos * revolution_sin + CROSS_SECTION_RADIUS * y_rotation_sin * solid_sin,
                    );

                    let projection_multiplier = PROJECTION_DISTANCE / (z + OBJECT_DISTANCE);

                    point_position.x = X_ORIGIN + x * projection_multiplier;
                    point_position.y = Y_ORIGIN - y * projection_multiplier;

                    point.set_position(point_position);
                    point.set_radius(luminance);

                    window.draw(&point);
                }

                revolution_angle += 0.1;
            }

            solid_angle += 0.1;
        }

        window.display();

        y_rotation_angle += 0.03;
        z_rotation_angle += 0.03;
    }
}
