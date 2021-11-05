use core::fmt;
use core::mem;
use rand::{Rng, thread_rng};

macro_rules! unchecked_assert {
	($cond:expr) => {{
		debug_assert!($cond);
	}};
}

macro_rules! unchecked_assert_eq {
	($lhs:expr, $rhs:expr) => {{
		debug_assert_eq!($lhs, $rhs);
	}};
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(align(4))]
pub struct Pos {
	x: u8,
	y: u8,
}

impl Pos {
	pub const fn new(x: u8, y: u8) -> Self {
		Self { x, y }
	}
}

#[derive(Clone, Copy, Debug)]
pub enum Dir {
	Right,
	Down,
	Left,
	Up,
}

struct Snake {
	body: Vec<Pos>,
	head: usize,
	tail: usize,
}

impl Snake {
	pub fn new(body: impl Into<Vec<Pos>>) -> Self {
		let mut body = body.into();
		let head = body.len() - 1;
		// Round up length to nearest power of 2
		// How the trick works: if bit n is set, then copy that bit to all bits below n.
		// Then add one -> power of 2!
		let mut l = body.len() - 1;
		(1..mem::size_of::<usize>() * 8).for_each(|i| l |= l >> i);
		l += 1;
		body.resize(l, Pos { x: 0, y: 0 });
		Self {
			body,
			head,
			tail: 0,
		}
	}

	/// # Returns
	///
	/// The new position of the head and the old position of the tail.
	pub fn mov(&mut self, grow: impl FnOnce(Pos) -> bool, direction: Dir) -> Option<(Pos, Pos)> {
		let (dx, dy) = match direction {
			Dir::Right => (1, 0),
			Dir::Down => (0, 1),
			Dir::Left => (-1, 0),
			Dir::Up => (0, -1),
		};
		unchecked_assert_eq!(self.body.len().count_ones(), 1);
		unchecked_assert!(self.head < self.body.len());
		unchecked_assert!(self.tail < self.body.len());
		let tail = self.body[self.tail];
		let head = self.body[self.head];
		let (x, y) = (i16::from(head.x) + dx, i16::from(head.y) + dy);
		if let (Ok(x), Ok(y)) = (x.try_into(), y.try_into()) {
			self.head = (self.head + 1) & (self.body.len() - 1);
			self.body[self.head] = Pos::new(x, y);
			if grow(Pos::new(x, y)) {
				if self.head == self.tail {
					let old_len = self.body.len();
					self.body.resize(old_len * 2, Pos { x: 0, y: 0 });
					// Make sure the snake remains in one piece
					// i.e. ss____ss
					//   -> ss____ss________
					//   -> ss____________ss
					let (old, new) = self.body.split_at_mut(old_len);
					new[self.tail + 1..].copy_from_slice(&old[self.tail + 1..]);
					// Update the tail accordingly
					self.tail += old_len;
				}
			} else {
				self.tail = (self.tail + 1) & (self.body.len() - 1);
			}
			let head = self.body[self.head];
			Some((head, tail))
		} else {
			None
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
	Empty,
	Apple,
	Snake,
}

pub struct Game<const W: u8, const H: u8>
where
	[(); W as usize]: Sized,
	[(); H as usize]: Sized,
{
	snake: Snake,
	cells: [[Cell; W as usize]; H as usize],
}

impl<const W: u8, const H: u8> Game<W, H>
where
	[(); W as usize]: Sized,
	[(); H as usize]: Sized,
{
	/// # Returns
	///
	/// true if the snake lives, false if dead.
	pub fn step(&mut self, direction: Dir) -> bool {
		let pred = |p: Pos| self.cells[usize::from(p.y)][usize::from(p.x)] == Cell::Apple;
		if let Some((head, tail)) = self.snake.mov(pred, direction) {
			match self.cells[usize::from(head.y)][usize::from(head.x)] {
				Cell::Empty => (),
				Cell::Apple => self.place_apple(),
				Cell::Snake => return false,
			}
			self.cells[usize::from(head.y)][usize::from(head.x)] = Cell::Snake;
			self.cells[usize::from(tail.y)][usize::from(tail.x)] = Cell::Empty;
			true
		} else {
			false
		}
	}

	fn place_apple(&mut self) {
		loop {
			let mut r = thread_rng();
			let (x, y) = (r.gen_range(0..W), r.gen_range(0..H));
			if self.cells[usize::from(y)][usize::from(x)] == Cell::Empty {
				self.cells[usize::from(y)][usize::from(x)] = Cell::Apple;
				break;
			}
		}
	}
}

impl<const W: u8, const H: u8> fmt::Display for Game<W, H>
where
	[(); W as usize]: Sized,
	[(); H as usize]: Sized,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut newline = false;
		for row in self.cells.iter() {
			newline.then(|| writeln!(f));
			newline = true;
			for e in row {
				f.write_str(match *e {
					Cell::Empty => "_",
					Cell::Apple => "A",
					Cell::Snake => "s",
				})?;
			}
		}
		Ok(())
	}
}

impl<const W: u8, const H: u8> Default for Game<W, H>
where
	[(); W as usize]: Sized,
	[(); H as usize]: Sized,
{
	fn default() -> Self {
		let mut cells = [[Cell::Empty; W as usize]; H as usize];
		let (x, y) = (W / 2, H / 2);
		let snake = Snake::new([Pos::new(x, y + 1), Pos::new(x, y), Pos::new(x, y - 1)]);
		for p in snake.body.iter().take((snake.head + 1) & (snake.body.len() - 1)) {
			cells[usize::from(p.y)][usize::from(p.x)] = Cell::Snake;
		}
		let mut s = Self {
			cells,
			snake,
		};
		s.place_apple();
		s
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn snake() {
		let mut s = Snake::new([Pos::new(0, 0), Pos::new(1, 0), Pos::new(2, 0)]); // head = 2
		s.mov(false, Dir::Down).unwrap(); // head = 3
		s.mov(false, Dir::Down).unwrap(); // head = 0
		s.mov(false, Dir::Down).unwrap(); // head = 1
		assert_eq!(&s.body, &[Pos::new(2, 2), Pos::new(2, 3), Pos::new(2, 0), Pos::new(2, 1)]);
		s.mov(true, Dir::Right).unwrap(); // head = 2
		s.mov(false, Dir::Right).unwrap(); // head = 3
		s.mov(false, Dir::Right).unwrap(); // head = 0
		assert_eq!(&s.body, &[Pos::new(5, 3), Pos::new(2, 3), Pos::new(3, 3), Pos::new(4, 3)]);
		s.mov(true, Dir::Up).unwrap(); // head = 1
		assert_eq!(&s.body, &[
			Pos::new(5, 3), Pos::new(5, 2), Pos::new(3, 3), Pos::new(4, 3),
			Pos::new(0, 0), Pos::new(0, 0), Pos::new(3, 3), Pos::new(4, 3),
		]);
	}
}
