/// Enables immutable call chaining on all types.
///
/// # Example
///
/// ```rust
/// use chainer::*;
///
/// struct HelloWorld;
/// impl HelloWorld {
///     fn print(&self) -> &'static str {
///         println!("Hello, world!");
///         "It works!"
///     }
/// }
///
/// fn main() {
///     let value: &'static str = HelloWorld
///         .chain(HelloWorld::print)
///         .chain(HelloWorld::print)
///         .chain(HelloWorld::print)
///         .result;
///
///     // Hello, world!
///     // Hello, world!
///     // Hello, world!
///     // It works!
/// }
/// ```
pub trait CallChain<S: ?Sized = Self> {
	/// Enables immutable call chaining on all types.
	///
	/// # Example
	///
	/// ```rust
	/// use chainer::*;
	///
	/// struct HelloWorld;
	/// impl HelloWorld {
	///     fn print(&self) -> &'static str {
	///         println!("Hello, world!");
	///         "It works!"
	///     }
	/// }
	///
	/// fn main() {
	///     let value: &'static str = HelloWorld
	///         .chain(HelloWorld::print)
	///         .chain(HelloWorld::print)
	///         .chain(HelloWorld::print)
	///         .result;
	///
	///     // Hello, world!
	///     // Hello, world!
	///     // Hello, world!
	///     // It works!
	/// }
	/// ```
	fn chain<R, F: FnOnce(&S) -> R>(&self, f: F) -> CallChainResult<'_, S, R>;
}

/// Enables mutable call chaining on all types.
///
/// # Example
///
/// ```rust
/// use chainer::*;
///
/// struct Counter { value: i32 }
/// impl Counter {
///     fn increment(&mut self) -> i32 {
///         self.value += 1;
///         println!("{}", self.value);
///         self.value
///     }
/// }
///
/// fn main() {
///     let value: i32 = Counter { value: 0 }
///         .chain_mut(Counter::increment)
///         .chain_mut(Counter::increment)
///         .chain_mut(Counter::increment)
///         .result;
///
///        println!("{value}");
///
///     // 1
///     // 2
///     // 3
///     // 3
/// }
/// ```
pub trait CallChainMut<S: ?Sized = Self> {
	/// Enables mutable call chaining on all types.
	///
	/// # Example
	///
	/// ```rust
	/// use chainer::*;
	///
	/// struct Counter { value: i32 }
	/// impl Counter {
	///     fn increment(&mut self) -> i32 {
	///         self.value += 1;
	///         println!("{}", self.value);
	///         self.value
	///     }
	/// }
	///
	/// fn main() {
	///     let value: i32 = Counter { value: 0 }
	///         .chain_mut(Counter::increment)
	///         .chain_mut(Counter::increment)
	///         .chain_mut(Counter::increment)
	///         .result;
	///
	///        println!("{value}");
	///
	///     // 1
	///     // 2
	///     // 3
	///     // 3
	/// }
	/// ```
	fn chain_mut<R, F: FnOnce(&mut S) -> R>(&mut self, f: F) -> CallChainResultMut<'_, S, R>;
}
impl<T: ?Sized> CallChain for T {
	#[inline]
	default fn chain<R, F: FnOnce(&T) -> R>(&self, f: F) -> CallChainResult<'_, T, R> {
		CallChainResult {
			result: f(self),
			this: self
		}
	}
}
impl<T: ?Sized> CallChainMut for T {
	#[inline]
	default fn chain_mut<R, F: FnOnce(&mut T) -> R>(&mut self, f: F) -> CallChainResultMut<'_, T, R> {
		CallChainResultMut {
			result: f(self),
			this: self
		}
	}
}

/// A result from a call chain. Dereferences to the return value but can also be used to chain further, immutably.
pub struct CallChainResult<'a, S: ?Sized, R> {
	this: &'a S,

	/// The result of the chained function.
	pub result: R
}

impl<S: ?Sized, R> CallChainResult<'_, S, R> {
	#[inline]
	/// Returns the result of the chained function.
	pub fn into_result(self) -> R {
		self.result
	}
}

impl<S: ?Sized, R> AsRef<S> for CallChainResult<'_, S, R> {
	#[inline]
	fn as_ref(&self) -> &S {
		self.this
	}
}

impl<S: ?Sized, R> core::ops::Deref for CallChainResult<'_, S, R> {
	type Target = R;

	#[inline]
	fn deref(&self) -> &R {
		&self.result
	}
}
impl<S: ?Sized, R> core::ops::DerefMut for CallChainResult<'_, S, R> {
	#[inline]
	fn deref_mut(&mut self) -> &mut R {
		&mut self.result
	}
}

impl<S: ?Sized, T> CallChain<S> for CallChainResult<'_, S, T> {
	#[inline]
	fn chain<R, F: FnOnce(&S) -> R>(&self, f: F) -> CallChainResult<'_, S, R> {
		CallChainResult {
			result: f(self.this),
			this: self.this
		}
	}
}

/// A result from a call chain. Dereferences to the return value but can also be used to chain further, mutably.
pub struct CallChainResultMut<'a, S: ?Sized, R> {
	this: &'a mut S,

	/// The result of the chained function.
	pub result: R
}

impl<S: ?Sized, R> CallChainResultMut<'_, S, R> {
	#[inline]
	/// Returns the result of the chained function.
	pub fn into_result(self) -> R {
		self.result
	}
}

impl<S: ?Sized, R> AsRef<S> for CallChainResultMut<'_, S, R> {
	#[inline]
	fn as_ref(&self) -> &S {
		self.this
	}
}
impl<S: ?Sized, R> AsMut<S> for CallChainResultMut<'_, S, R> {
	#[inline]
	fn as_mut(&mut self) -> &mut S {
		self.this
	}
}

impl<S: ?Sized, R> core::ops::Deref for CallChainResultMut<'_, S, R> {
	type Target = R;

	#[inline]
	fn deref(&self) -> &R {
		&self.result
	}
}
impl<S: ?Sized, R> core::ops::DerefMut for CallChainResultMut<'_, S, R> {
	#[inline]
	fn deref_mut(&mut self) -> &mut R {
		&mut self.result
	}
}

impl<S: ?Sized, T> CallChain<S> for CallChainResultMut<'_, S, T> {
	#[inline]
	fn chain<R, F: FnOnce(&S) -> R>(&self, f: F) -> CallChainResult<'_, S, R> {
		CallChainResult {
			result: f(self.this),
			this: self.this
		}
	}
}
impl<S: ?Sized, T> CallChainMut<S> for CallChainResultMut<'_, S, T> {
	#[inline]
	fn chain_mut<R, F: FnOnce(&mut S) -> R>(&mut self, f: F) -> CallChainResultMut<'_, S, R> {
		CallChainResultMut {
			result: f(self.this),
			this: self.this
		}
	}
}