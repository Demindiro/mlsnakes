use core::marker::PhantomData;
use core::mem::MaybeUninit;
use rand::{Rng, thread_rng};

pub trait Dna<T>
where
	T: Clone,
{
	fn serialize(&self) -> Box<[T]>;

	fn deserialize(dna: Box<[T]>) -> Self;

	fn mutate(&mut self);

	fn spawn() -> Self;

	fn mix(a: &Box<[T]>, b: &Box<[T]>) -> Box<[T]>{
		assert_eq!(a.len(), b.len(), "dna strings are not of the same size");
		let mut r = thread_rng();
		let mut o = a.clone();
		o.iter_mut().zip(b.iter()).for_each(|(o, b)| { r.gen::<bool>().then(|| *o = b.clone()); });
		o
	}
}

pub struct Population<P, const S: usize, T>([P; S], PhantomData<T>)
where
	P: Dna<T>,
	[(); S]: Sized,
	T: Clone;

impl<P, const S: usize, T> Population<P, S, T>
where
	P: Dna<T>,
	[(); S]: Sized,
	T: Clone,
{
	fn step(&mut self, params: PopulationParams) {
		let mut r = thread_rng();
		let mut plebs = (0..params.elite_factor).map(|_| {
			P::deserialize(P::mix(&self.0[r.gen_range(0..S)].serialize(), &self.0[r.gen_range(0..S)].serialize()))
		}).collect::<Box<_>>();
	}
}

impl<P, const S: usize, T> Default for Population<P, S, T>
where
	P: Dna<T>,
	[(); S]: Sized,
	T: Clone,
{
	fn default() -> Self {
		Self((0..S).map(|_| Dna::spawn()).collect::<Vec<_>>().try_into().map_err(|_| ()).unwrap(), PhantomData)
	}
}

pub struct PopulationParams {
	pub elite_factor: usize,
	pub mutation_rate: f32,
}
