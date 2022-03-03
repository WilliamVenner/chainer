use crate::*;

#[test]
fn test_immutable() {
	static COUNTER: core::sync::atomic::AtomicU8 = core::sync::atomic::AtomicU8::new(0);

	struct Counter;
	impl Counter {
		fn increment(&self) {
			COUNTER.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
		}
	}

	Counter
		.chain(Counter::increment)
		.chain(Counter::increment)
		.chain(Counter::increment);

	assert_eq!(COUNTER.load(core::sync::atomic::Ordering::Relaxed), 3);
}

#[cfg(not(feature = "results"))]
#[test]
fn test_basic_mutable() {
	struct Counter { value: i32 }
	impl Counter {
		fn increment(&mut self) -> i32 {
			self.value += 1;
			self.value
		}
	}

	let mut counter = Counter { value: 0 };

	counter
		.chain_mut(Counter::increment)
		.chain_mut(Counter::increment)
		.chain_mut(Counter::increment);

	assert_eq!(counter.value, 3);
}

#[cfg(feature = "results")]
#[test]
fn test_results_mutable() {
	struct Counter { value: i32 }
	impl Counter {
		fn increment(&mut self) -> i32 {
			self.value += 1;
			self.value
		}
	}

	let result = Counter { value: 0 }
		.chain_mut(Counter::increment)
		.chain_mut(Counter::increment)
		.chain_mut(Counter::increment)
		.result;

	assert_eq!(result, 3);
}