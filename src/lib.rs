#![no_std]

//! # `extend-ref`
//!
//! A wrapper struct that implements `Extend` for mutable references.
//!
//!
//! ```rust
//! use extend_ref::ExtendRef;
//!
//! fn unzip_on_refs(
//!     mut squares: &mut impl Extend<i32>,
//!     mut cubes: &mut impl Extend<i32>,
//!     mut tesseracts: &mut impl Extend<i32>
//! ) {
//!     // Create an iterator of a 3-tuple
//!     let iter = (0i32..10).map(|i| (i * i, i.pow(3), i.pow(4)));
//!
//!     // Unzip the iterator into the three referenced collections
//!     (ExtendRef(squares), ExtendRef(cubes), ExtendRef(tesseracts)).extend(iter);
//! }
//! ```
//!



pub struct ExtendRef<'a, T>(pub &'a mut T);

impl<'a, A, T: Extend<A>> Extend<A> for ExtendRef<'a, T> {
	fn extend<U>(&mut self, iter: U)
	where
		U: IntoIterator<Item = A>,
	{
		self.0.extend(iter)
	}
}
