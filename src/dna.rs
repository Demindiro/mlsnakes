use core::cmp::Ordering;
use core::marker::PhantomData;
use core::ops::Range;
use std::collections::BinaryHeap;
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

pub struct Population<P, T>(Vec<P>, PhantomData<T>)
where
	P: Dna<T>,
	T: Clone;

impl<P, T> Population<P, T>
where
	P: Dna<T>,
	T: Clone,
{
	/// # Returns
	///
	/// The best score achieved in this step.
	pub fn step(&mut self, params: &PopulationParams, test: impl Fn(&mut P) -> usize) -> usize {
		let mut r = thread_rng();

		// Ensure we have enough "elites"
		self.0.resize_with(params.elite_size, P::spawn);

		// Breed population
		let len = self.0.len();
		let mut pop = Vec::with_capacity(params.total_size);
		pop.resize_with(
			params.total_size - len,
			|| P::deserialize(P::mix(&self.0[r.gen_range(0..len)].serialize(), &self.0[r.gen_range(0..len)].serialize())),
		);
		pop.extend(self.0.drain(..));

		// Mutate some specimen
		let len = pop.len();
		params.mutate.clone().for_each(|_| pop[r.gen_range(0..len)].mutate());

		// Test each specimen
		let scores = pop.iter_mut().map(test).collect::<Vec<_>>();

		// Collect the best specimens
		struct Entry<P, T>
		where
			P: Dna<T>,
			T: Clone,
		{
			score: usize,
			specimen: P,
			_marker: PhantomData<T>,
		}

		impl<P, T> PartialEq for Entry<P, T>
		where
			P: Dna<T>,
			T: Clone,
		{
			fn eq(&self, rhs: &Self) -> bool {
				self.score.eq(&rhs.score)
			}
		}

		impl<P, T> Eq for Entry<P, T>
		where
			P: Dna<T>,
			T: Clone,
		{}

		impl<P, T> PartialOrd for Entry<P, T>
		where
			P: Dna<T>,
			T: Clone,
		{
			fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
				self.score.partial_cmp(&rhs.score)
			}
		}

		impl<P, T> Ord for Entry<P, T>
		where
			P: Dna<T>,
			T: Clone,
		{
			fn cmp(&self, rhs: &Self) -> Ordering {
				self.score.cmp(&rhs.score)
			}
		}

		let mut bh = BinaryHeap::default();
		let mut max_score = 0;
		for (specimen, score) in pop.into_iter().zip(scores) {
			max_score = max_score.max(score);
			bh.push(Entry { score, specimen, _marker: PhantomData });
		}

		// Save best specimens.
		self.0.extend(bh.into_iter().take(params.elite_size).map(|e| e.specimen));

		max_score
	}
}

impl<P, T> Default for Population<P, T>
where
	P: Dna<T>,
	T: Clone,
{
	fn default() -> Self {
		Self(Vec::new(), PhantomData)
	}
}

pub struct PopulationParams {
	pub total_size: usize,
	pub elite_size: usize,
	pub mutate: Range<usize>,
}
