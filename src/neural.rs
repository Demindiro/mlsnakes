use core::mem;
use core::ops::Mul;
use rand::{Rng, thread_rng, rngs::ThreadRng};

#[derive(Clone, Copy, Debug)]
pub struct Vector<const L: usize>(pub [f32; L]);

impl<const L: usize> Vector<L> {
	fn dot(&self, rhs: &Self) -> f32 {
		self.0.iter().copied().zip(rhs.0).map(|(l, r)| l * r).sum()
	}

	fn transform(mut self, mut f: impl FnMut(f32) -> f32) -> Self {
		self.0.iter_mut().for_each(|e| *e = f(*e));
		self
	}
}

const I: usize = 7;
const L0: usize = 8;
const L1: usize = 8;
const O: usize = 4;

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
	layers: (Matrix<I, L0>, Matrix<L0, L1>, Matrix<L1, O>),
}

impl NeuralNet {
	pub fn apply(&self, input: &Vector<I>) -> Vector<O> {
		let v = (&self.layers.0 * input).transform(Self::activate_f);
		let v = (&self.layers.1 * &v).transform(Self::activate_f);
		let v = (&self.layers.2 * &v).transform(Self::activate_f);
		v
	}

	fn activate_f(x: f32) -> f32 {
		assert!(!x.is_nan());
		x / (1.0 + x.abs())
	}
}

impl Default for NeuralNet {
	fn default() -> Self {
		Self {
			layers: (
				Matrix([Vector([0.0; I]); L0]),
				Matrix([Vector([0.0; L0]); L1]),
				Matrix([Vector([0.0; L1]); O]),
			)
		}
	}
}

impl super::Dna<f32> for NeuralNet {
	fn serialize(&self) -> Box<[f32]> {
		let a = self.layers.0.0.iter().flat_map(|r| r.0);
		let b = self.layers.1.0.iter().flat_map(|r| r.0);
		let c = self.layers.2.0.iter().flat_map(|r| r.0);
		a.chain(b).chain(c).collect()
	}

	fn deserialize(dna: Box<[f32]>) -> Self {
		let (a, bc) = dna.split_at(mem::size_of::<Matrix<I, L0>>() / 4);
		let (b, c) = bc.split_at(mem::size_of::<Matrix<L0, L1>>() / 4);
		let a = a.chunks(I).map(|r| Vector(r.try_into().unwrap())).collect::<Vec<_>>();
		let a = a.chunks(L0).map(|m| Matrix(m.try_into().unwrap())).next().unwrap();
		let b = b.chunks(L0).map(|r| Vector(r.try_into().unwrap())).collect::<Vec<_>>();
		let b = b.chunks(L1).map(|m| Matrix(m.try_into().unwrap())).next().unwrap();
		let c = c.chunks(L1).map(|r| Vector(r.try_into().unwrap())).collect::<Vec<_>>();
		let c = c.chunks(O).map(|m| Matrix(m.try_into().unwrap())).next().unwrap();
		Self {
			layers: (a, b, c),
		}
	}

	fn mutate(&mut self) {
		let mut r = thread_rng();
		let mut v = self.serialize();
		v[r.gen_range(0..v.len())] += r.gen_range(-10.0..10.0);
		*self = Self::deserialize(v);
	}

	fn spawn() -> Self {
		let s = (mem::size_of::<Matrix<I, L0>>() + mem::size_of::<Matrix<L0, L1>>() + mem::size_of::<Matrix<L1, O>>()) / 4;
		let mut r = thread_rng();
		Self::deserialize((0..s).map(|_| r.gen_range(-10.0..10.0)).collect())
	}
}
