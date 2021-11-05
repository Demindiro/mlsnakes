#![feature(associated_type_bounds)]
#![feature(destructuring_assignment)]
#![feature(generic_const_exprs)]

mod dna;
mod game;
mod neural;

use dna::*;
use game::*;
use neural::*;

use std::thread;
use std::time::Duration;
use std::io::{stdout, Write};

fn main() {
	let mut pop = Population::<NeuralNet, _>::default();
	let params = PopulationParams {
		elite_size: 512,
		total_size: 8192 * 4,
		mutate: 8..128,
	};

	let threshold = 50;

	fn determine_action(pop: &NeuralNet, game: &Game<8, 8>) -> Dir {
		let d = |d: f32| d.signum() * (1.0 / (d.abs() + 1.0));

		let (a, h) = (game.apple(), game.head());
		let (dx, dy) = (i16::from(a.x) - i16::from(h.x), i16::from(a.y) - i16::from(h.y));
		let mut v = Vector([0.0, 0.0, 0.0, 0.0, d(f32::from(dx)), d(f32::from(dy)), 1.0]);

		let is_obstacle = |x: i16, y: i16| {
			if let (Ok(x), Ok(y)) = (x.try_into(), y.try_into()) {
				match game.get(Pos::new(x, y)) {
					Some(Cell::Snake) | None => true,
					Some(Cell::Apple) | Some(Cell::Empty) => false,
				}
			} else {
				true
			}
		};

		// Locate obstacles
		let (hx, hy) = (h.x.into(), h.y.into());
		for x in (-1..hx).rev() {
			if is_obstacle(x, hy) {
				v.0[0] = 1.0 / f32::from(hx - x);
				break;
			}
		}
		for x in hx + 1..=8 {
			if is_obstacle(x, hy) {
				v.0[1] = 1.0 / f32::from(x - hx);
				break;
			}
		}
		for y in (-1..hy).rev() {
			if is_obstacle(hx, y) {
				v.0[2] = 1.0 / f32::from(hy - y);
				break;
			}
		}
		for y in hy + 1..=8 {
			if is_obstacle(hx, y) {
				v.0[3] = 1.0 / f32::from(y - hy);
				break;
			}
		}

		// Determine action
		let r = pop.apply(&v).0;
		let (mut m, mut mv) = (0, 0.0);
		for (i, v) in r.iter().enumerate() {
			if *v < mv {
				mv = *v;
				m = i;
			}
		}
		match m {
			0 => Dir::Up,
			1 => Dir::Down,
			2 => Dir::Left,
			3 => Dir::Right,
			_ => unreachable!(),
		}
	}

	let mut best = 0;
	while best < threshold {
		best = pop.step(&params, |pop| {
			let mut game = Game::<8, 8>::default();
			let mut remaining_steps = 20;
			let mut last_apples_eaten = game.apples_eaten;
			while game.step(determine_action(pop, &game)) && remaining_steps > 0 {
				if last_apples_eaten != game.apples_eaten {
					last_apples_eaten = game.apples_eaten;
					remaining_steps += 30;
				}
				remaining_steps -= 1;
			}
			game.apples_eaten
		});
		println!("{}", best);
	}

	let mut game = Game::<8, 8>::default();
	let mut max_steps = 100;
	let mut apples = 0;
	loop {
		println!("{}", &game);
		std::thread::sleep(Duration::from_millis(1000 / 30));

		if !game.step(determine_action(pop.best(), &game)) {
			println!("The snake died!");
			std::thread::sleep(Duration::from_secs(1));
			game = Game::default();
			max_steps = 100;
			apples = game.apples_eaten;
			print!("\x1b[{}A\r", 1);
			print!("               ");
		}

		if apples < game.apples_eaten {
			apples = game.apples_eaten;
			max_steps += 30;
		}

		max_steps -= 1;
		print!("\x1b[{}A\r", 8);
	}
}
