//! Pure rust implementation of the [`java.util.Random`] class.
//! 
//! # Examples
//! 
//! ```
//! use javarandom::JavaRandom;
//! 
//! // Create a JavaRandom instance with a random seed
//! let mut random = JavaRandom::new();
//! 
//! println!("Random number between 0.0 and 1.0: {}", random.next_float());
//! ```
//! 
//! [`java.util.Random`]: https://docs.oracle.com/javase/8/docs/api/java/util/Random.html

#![warn(missing_docs)]

mod java_random;
pub use java_random::*;
