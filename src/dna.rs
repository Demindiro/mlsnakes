use rand::{Rng, thread_rng};

pub trait Dna {
	fn serialize(&self) -> Box<[u8]>;

	fn deserialize(dna: Box<[u8]>) -> Self;

	fn mutate(&mut self);

	fn spawn() -> Self;

	fn mix(a: &Box<[u8]>, b: &Box<[u8]>) -> Box<[u8]>{
		assert_eq!(a.len(), b.len(), "dna strings are not of the same size");
		let mut r = thread_rng();
		let mut out = a.iter().map(|_| 0).collect::<Box<_>>();
		for (out, (a, b)) in out.iter_mut().zip(a.iter().zip(b.iter())) {
			*out = *[a, b][r.gen_range(0..2)];
		}
		out
	}
}

pub struct Population<P, const S: usize>([P; S])
where
	P: Dna,
	[(); S]: Sized;

impl<P, const S: usize> Default for Population<P, S>
where
	P: Dna,
	[(); S]: Sized,
{
	fn default() -> Self {
		Self((0..S).map(|_| Dna::spawn()).collect::<Vec<_>>().try_into().map_err(|_| ()).unwrap())
	}
}
