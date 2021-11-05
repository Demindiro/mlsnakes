use core::ops::Mul;

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
