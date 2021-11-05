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
		elite_size: 128,
		total_size: 1024,
		mutate: 5..20,
	};
	loop {
		let best = pop.step(&params, |pop| {
			let mut game = Game::<8, 8>::default();
			while game.step(Dir::Up) {
				todo!()
			}
			100
		});
		println!("{}", best);
	}
}
