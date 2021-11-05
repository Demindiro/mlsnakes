use core::mem;
use core::ops::Mul;
use rand::{Rng, thread_rng, rngs::ThreadRng};

#[derive(Clone, Copy, Debug)]
pub struct Vector<const L: usize>([f32; L]);

impl<const L: usize> Vector<L> {
	fn dot(&self, rhs: &Self) -> f32 {
		self.0.iter().copied().zip(rhs.0).map(|(l, r)| l * r).sum()
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Matrix<const W: usize, const H: usize>([Vector<W>; H]);

impl<const W: usize, const H: usize> Mul<&Vector<W>> for &Matrix<W, H> {
	type Output = Vector<H>;

	fn mul(self, rhs: &Vector<W>) -> Self::Output {
		let mut v = Vector([0.0; H]);
		v.0.iter_mut().zip(self.0.iter()).for_each(|(o, i)| *o = rhs.dot(i));
		v
	}
}

pub struct NeuralNet {
	layers: [Matrix<4, 4>; 3],
}

impl NeuralNet {
	pub fn apply(&self, input: &Vector<4>) -> Vector<4> {
		let v = &self.layers[0] * input;
		let v = &self.layers[1] * &v;
		let v = &self.layers[2] * &v;
		v
	}
}

impl Default for NeuralNet {
	fn default() -> Self {
		Self { layers: [Matrix([Vector([0.0; 4]); 4]); 3] }
	}
}

impl super::Dna<f32> for NeuralNet {
	fn serialize(&self) -> Box<[f32]> {
		self.layers.iter().flat_map(|m| m.0).flat_map(|r| r.0).collect()
	}

	fn deserialize(dna: Box<[f32]>) -> Self {
		let r = dna.chunks(4).map(|r| Vector(r.try_into().unwrap())).collect::<Vec<_>>();
		let m = r.chunks(4).map(|m| Matrix(m.try_into().unwrap()));
		Self { layers: m.collect::<Vec<_>>().try_into().unwrap() }
	}

	fn mutate(&mut self) {
		let mut r = thread_rng();
		self.layers[r.gen_range(0..3)].0[r.gen_range(0..4)].0[r.gen_range(0..4)] = r.gen();
	}

	fn spawn() -> Self {
		let e = |rng: &mut ThreadRng| rng.gen();
		let v = |rng: &mut _| Vector([e(rng), e(rng), e(rng), e(rng)]);
		let m = |rng: &mut _| Matrix([v(rng), v(rng), v(rng), v(rng)]);
		let mut r = thread_rng();
		Self { layers: [m(&mut r), m(&mut r), m(&mut r)] }
	}
}
