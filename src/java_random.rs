#![allow(clippy::needless_return)]

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

use std::num::Wrapping;

/// Rust implementation of the [`java.util.Random`] class.
/// 
/// [`java.util.Random`]: https://docs.oracle.com/javase/8/docs/api/java/util/Random.html
#[derive(Eq, PartialEq, Clone, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JavaRandom
{
	seed: Wrapping<i64>
}

impl Default for JavaRandom
{
	fn default() -> JavaRandom
	{
		return JavaRandom::new();
	}
}

impl JavaRandom
{
	/// Creates a new JavaRandom instance with a random seed.
	pub fn new() -> JavaRandom
	{
		return JavaRandom::with_seed(unsafe { std::mem::transmute(std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_secs()) });
	}

	/// Creates a new JavaRandom instance with a given seed.
	pub fn with_seed(seed: i64) -> JavaRandom
	{
		let seed = Wrapping(seed);

		return JavaRandom
		{
			seed: (seed ^ Wrapping(0x5DEECE66Di64)) & Wrapping((1i64 << 48) - 1)
		};
	}

	/// Set the seed of this JavaRandom instance. This is identical to creating a new instance with the given seed.
	pub fn set_seed(&mut self, seed: i64)
	{
		self.seed = (Wrapping(seed) ^ Wrapping(0x5DEECE66Di64)) & Wrapping((1i64 << 48) - 1);
	}

	/// Get the seed of this JavaRandom instance.
	pub fn seed(&self) -> i64
	{
		return self.seed.0;
	}

	fn next(&mut self, bits: i32) -> i32
	{
		self.seed = (self.seed * Wrapping(0x5DEECE66Di64) + Wrapping(0xBi64)) & Wrapping((1i64 << 48) - 1);

		return unsafe { std::mem::transmute::<u64, i64>(std::mem::transmute::<i64, u64>(self.seed.0) >> (48 - bits)) } as i32;
	}

	/// Replace the content of bytes with the next random bytes.
	pub fn next_bytes(&mut self, bytes: &mut Vec<u8>)
	{
		let mut i = 0;
		while i < bytes.len()
		{
			let mut rnd = self.next_int(None);
			let mut n = (bytes.len() - i).min(4);

			while n > 0
			{
				bytes[i] = unsafe { std::mem::transmute::<i8, u8>(rnd as i8) };

				i += 1;
				
				n -= 1;
				rnd >>= 8;
			}
		}
	}

	/// Get a random i32 from this JavaRandom instance.
	pub fn next_int(&mut self, bound: Option<u32>) -> i32
	{
		return match bound
		{
			Some(bound) =>
			{
				assert!(bound > 0);
				
				let bound = bound as i32;

				if (bound & -bound) == bound // i.e., bound is a power of 2
				{
					return (((bound as i64) * (self.next(31) as i64)) >> 31) as i32;
				}

				let mut bits;
				let mut val;

				loop
				{
					bits = self.next(31);
					val = bits % bound;

					if bits - val + (bound - 1) >= 0
					{
						return val;
					}
				}
			},
			None => self.next(32)
		};
	}

	/// Get a random i64 from this JavaRandom instance.
	pub fn next_long(&mut self) -> i64
	{
		return ((self.next(32) as i64) << 32) + self.next(32) as i64;
	}

	/// Get a random bool from this JavaRandom instance.
	pub fn next_boolean(&mut self) -> bool
	{
		return self.next(1) != 0;
	}

	/// Get a random f32 from this JavaRandom instance
	pub fn next_float(&mut self) -> f32
	{
		return self.next(24) as f32 / (1 << 24) as f32;
	}

	/// Get a random f64 from this JavaRandom instance.
	pub fn next_double(&mut self) -> f64
	{
		return (((self.next(26) as i64) << 27) + self.next(27) as i64) as f64 / (1i64 << 53) as f64;
	}
}
