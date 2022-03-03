/// Enables immutable call chaining on all types.
///
/// # Example
///
/// ```rust
/// use chainer::*;
///
/// struct HelloWorld;
/// impl HelloWorld {
///     fn print(&self) {
///         println!("Hello, world!");
///     }
/// }
///
/// fn main() {
///     HelloWorld
///         .chain(HelloWorld::print)
///         .chain(HelloWorld::print)
///         .chain(HelloWorld::print);
///
///     // Hello, world!
///     // Hello, world!
///     // Hello, world!
/// }
/// ```
pub trait CallChain {
	/// Enables immutable call chaining on all types.
	///
	/// # Example
	///
	/// ```rust
	/// use chainer::*;
	///
	/// struct HelloWorld;
	/// impl HelloWorld {
	///     fn print(&self) {
	///         println!("Hello, world!");
	///     }
	/// }
	///
	/// fn main() {
	///     HelloWorld
	///         .chain(HelloWorld::print)
	///         .chain(HelloWorld::print)
	///         .chain(HelloWorld::print);
	///
	///     // Hello, world!
	///     // Hello, world!
	///     // Hello, world!
	/// }
	/// ```
	fn chain<R, F: FnOnce(&Self) -> R>(&self, f: F) -> &Self {
		f(self);
		self
	}
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
///     fn increment(&mut self) {
///         self.value += 1;
///         println!("{}", self.value);
///     }
/// }
///
/// fn main() {
///     Counter { value: 0 }
///         .chain_mut(Counter::increment)
///         .chain_mut(Counter::increment)
///         .chain_mut(Counter::increment);
///
///     // 1
///     // 2
///     // 3
/// }
/// ```
pub trait CallChainMut {
	/// Enables mutable call chaining on all types.
	///
	/// # Example
	///
	/// ```rust
	/// use chainer::*;
	///
	/// struct Counter { value: i32 }
	/// impl Counter {
	///     fn increment(&mut self) {
	///         self.value += 1;
	///         println!("{}", self.value);
	///     }
	/// }
	///
	/// fn main() {
	///     Counter { value: 0 }
	///         .chain_mut(Counter::increment)
	///         .chain_mut(Counter::increment)
	///         .chain_mut(Counter::increment);
	///
	///     // 1
	///     // 2
	///     // 3
	/// }
	/// ```
	fn chain_mut<R, F: FnOnce(&mut Self) -> R>(&mut self, f: F) -> &mut Self {
		f(self);
		self
	}
}

impl<T: ?Sized> CallChain for T {}
impl<T: ?Sized> CallChainMut for T {}