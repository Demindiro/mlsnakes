#![feature(associated_type_bounds)]
#![feature(generic_const_exprs)]

mod game;
use game::*;

use std::thread;
use std::time::Duration;
use std::io::{stdout, Write};

fn main() {
	let mut game = Game::<8, 8>::default();
	let dirs = [Dir::Up, Dir::Up, Dir::Right, Dir::Down, Dir::Right, Dir::Down, Dir::Left, Dir::Left];
	let dirs = [Dir::Down];
	let mut dirs = dirs.iter().copied().cycle();
	loop {
		println!("{}", &game);
		std::thread::sleep(Duration::from_millis(333));
		if !game.step(dirs.next().unwrap()) {
			println!("The snake died!");
			break;
		}
		print!("\x1b[{}A\r", 8);
	}
}
