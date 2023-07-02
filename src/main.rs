use std::f32::consts::TAU;
use sfml::graphics::{RenderWindow, Color, RenderTarget, PrimitiveType, Vertex, RenderStates};
use sfml::system::{Vector2f, Vector2u};
use sfml::window::{Style, Event, Key, VideoMode};

fn main() {
    let window_size = Vector2u::new(1920, 1080);
    let window_size_f: Vector2f = window_size.as_other();
    let mut window = RenderWindow::new(
        VideoMode::new(window_size.x, window_size.y, 24),
        "Spinning thing",
        Style::FULLSCREEN,
        &Default::default(),
    );

    window.set_framerate_limit(60);
    window.set_mouse_cursor_visible(false);

    let mut vertices = vec![Vertex::with_pos_color(Vector2f::default(), Color::WHITE); 2_usize.pow(12)];
    let mut yaw: f32 = 0.0;
    let mut roll: f32 = 0.0;
    let mut pitch: f32 = 0.0;

    while window.is_open() {
        // event loop
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => window.close(),
                _ => {}
            }
        }

        // iterate over all knots
        let mut i = 0;
        let mut knot: f32 = 0.0;
        while knot <= 1.0 {

            // roll around the x-axis and apply the current pitch and yaw
            while roll < TAU {
                let (yaw_s, yaw_c) = yaw.sin_cos();
                let (roll_s, roll_c) = roll.sin_cos();
                let (pitch_s, pitch_c) = pitch.sin_cos();

                let f_knot = curve_f(knot);

                // multiply all the rotation matrices and the coordinate column of the current knot
                let x = knot * pitch_c * yaw_c + f_knot * (roll_s * pitch_s * yaw_c - roll_c * yaw_s);
                let y = knot * pitch_c * yaw_s + f_knot * (roll_s * pitch_s * yaw_s + roll_c * yaw_c);
                let z = knot * -pitch_s + f_knot * roll_s * pitch_c;

                // project points onto a screen on the z-axis
                let object_distance = 2.5;
                let screen_distance = window_size_f.x * object_distance * 0.04;

                vertices[i].position.x = window_size_f.x * 0.5 + x * screen_distance / (z + object_distance);
                vertices[i].position.y = window_size_f.y * 0.5 - y * screen_distance / (z + object_distance);

                i += 1;

                roll += 0.05;
            }
            roll = 0.0;

            knot += 0.05;
        }

         yaw += 0.02;
         pitch += 0.03;

        // rendering
        window.clear(Color::BLACK);
        window.draw_primitives(
            &vertices[..i],
            PrimitiveType::POINTS,
            &RenderStates::DEFAULT,
        );
        window.display();
    }
}

fn curve_f(knot: f32) -> f32 {
    -6.66667 * knot.powi(4) + 5.66667 * knot.powi(2) + 1.0
}