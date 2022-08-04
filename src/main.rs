#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(mixed_script_confusables)]

extern crate piston_window;
mod flow;
mod std_game;
use std_game::*;
use libm::{cos, sin};
use piston_window::*;
use std::f64::consts::PI;

const UP_BUTTON:    Button = Button::Keyboard(Key::W);
const DOWN_BUTTON:  Button = Button::Keyboard(Key::S);
const LEFT_BUTTON:  Button = Button::Keyboard(Key::A);
const RIGHT_BUTTON: Button = Button::Keyboard(Key::D);

const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLUE:  [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

fn main() {
	let title  = "Piston Window";
	let width  = 640.0;
	let height = 480.0;

    let mut window: PistonWindow = create_window(title, width, height);
	let mut airplane_engine = flow::Frame::new();
	let mut time = flow::Frame::new();

	let velocity = 1.0;

	let x0 = width / 2.0;
	let y0 = height / 2.0;
	let mut Θ = 0.0;
	let mut v_Θ = 0.0;
	let a_Θ = 0.00001 / 1000.0;
	let mut dx = 0.0;
	let mut dy = 0.0;

	let mut Up    = flow::Frame::new();
	let mut Down  = flow::Frame::new();
	let mut Left  = flow::Frame::new();
	let mut Right = flow::Frame::new();

	fn crate_object(w: f64, h: f64) -> [f64; 4] {
		[-w / 2.0, -h / 2.0, w, h]
	}

	let airplane_collider_mesh = Mesh(vec![
		crate_object(110.0, 180.0),
		[-280.0, -60.0, 560.0, 20.0]
	]);

    while let Some(event) = window.next() {
		time.start();

		Up.end();
		Down.end();
		Right.end();
		Left.end();

		if Up.delta_time > 500.0 {
			dy -= velocity;
		}
		if Down.delta_time > 500.0 {
			dy += velocity;
		}
		if Left.delta_time > 500.0 {
			dx += velocity;
		}
		if Right.delta_time > 500.0 {
			dx -= velocity;
		}

		airplane_engine.end();
		v_Θ += 0.5 * a_Θ * (airplane_engine.delta_time * airplane_engine.delta_time);
		Θ += v_Θ * time.delta_time;

		if let Some(button) = event.press_args() {
			if button == UP_BUTTON {
				Up.start();
				dy += velocity;
			}
			else if button == DOWN_BUTTON {
				Down.start();
				dy -= velocity;
			}
			else if button == LEFT_BUTTON {
				Left.start();
				dx -= velocity;
			}
			else if button == RIGHT_BUTTON {
				Right.start();
				dx += velocity;
			}
			else if button == Button::Keyboard(Key::E) {
				v_Θ = 0.0;
				airplane_engine.start(); // restart the airplane engine
			}
		}
		
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
			let transform = context
				.transform
				.trans(x0, y0)
				.trans(dx, -dy);

			// BEGIN AIRPLANE
			rectangle( // control panel
				shadow(BLUE, 0.4),
				crate_object(70.0, 40.0),
				transform.trans(0.0, -50.0),
				graphics
			);
			ellipse( // wing
				shadow(BLUE, 0.5),
				crate_object(600.0, 20.0),
				transform.trans(0.0, -50.0),
				graphics
			);
			ellipse( // right wheel
				shadow(BLUE, 0.3),
				crate_object(15.0, 25.0),
				transform.trans(50.0, 80.0),
				graphics
			);
			ellipse( // left wheel
				shadow(BLUE, 0.3),
				crate_object(15.0, 25.0),
				transform.trans(-50.0, 80.0),
				graphics
			);
			rectangle( // right leg
				shadow(BLUE, 0.2),
				crate_object(130.0, 10.0),
				transform.trans(-15.0, 24.0).rot_rad(-PI / 3.0),
				graphics
			);
			rectangle( // left leg
				shadow(BLUE, 0.2),
				crate_object(130.0, 10.0),
				transform.trans(15.0, 24.0).rot_rad(PI * (4.0 / 3.0)),
				graphics
			);
			ellipse( // body
				shadow(BLUE, 0.1),
				crate_object(100.0, 110.0),
				transform,
				graphics
			);
			ellipse( // engine
				[0.0, 0.0, 1.0, 1.0],
				crate_object(100.0, 100.0),
				transform,
				graphics
			);
			ellipse( // propeller blade 1
				RED,
				[-3.0, -10.0, 120.0, 20.0],
				transform.rot_rad(Θ),
				graphics,
			);
			ellipse( // propeller blade 2
				RED,
				[3.0, -10.0, 120.0, 20.0],
				transform.rot_rad(Θ + PI),
				graphics,
			);
			ellipse( // propeller body
				[1.0, 1.0, 0.0, 1.0],
				[-8.0, -8.0, 16.0, 16.0],
				transform,
				graphics,
			);
			// END AIRPLANE
        });
		
		time.end();
	}
}