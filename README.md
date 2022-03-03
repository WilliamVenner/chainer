[![crates.io](https://img.shields.io/crates/v/chainer.svg)](https://crates.io/crates/chainer)
[![docs.rs](https://docs.rs/chainer/badge.svg)](https://docs.rs/chainer/)
[![license](https://img.shields.io/crates/l/chainer)](https://github.com/WilliamVenner/chainer/blob/master/LICENSE)

# chainer

A cursed crate that allows for global call chaining with access to chained function results

**Currently the `results` feature requires the `min_specialization` Rust Nightly feature.**

# Usage

```toml
[dependencies]
chainer = "*"

# or, with a nightly compiler

[dependencies]
chainer = { version = "*", features = ["results"] }
```

# Examples

## Basic usage (default)

### Immutable call chaining

```rust
use chainer::*;

struct HelloWorld;
impl HelloWorld {
    fn print(&self) {
        println!("Hello, world!");
    }
}

fn main() {
    HelloWorld
        .chain(HelloWorld::print)
        .chain(HelloWorld::print)
        .chain(HelloWorld::print);

    // Hello, world!
    // Hello, world!
    // Hello, world!
}
```

### Mutable call chaining

```rust
use chainer::*;

struct Counter { value: i32 }
impl Counter {
    fn increment(&mut self) {
        self.value += 1;
        println!("{}", self.value);
    }
}

fn main() {
    Counter { value: 0 }
        .chain_mut(Counter::increment)
        .chain_mut(Counter::increment)
        .chain_mut(Counter::increment);

    // 1
    // 2
    // 3
}
```

## `features = ["results"]`

### Immutable call chaining

```rust
use chainer::*;

struct HelloWorld;
impl HelloWorld {
    fn print(&self) -> &'static str {
        println!("Hello, world!");
        "It works!"
    }
}

fn main() {
    let value: &'static str = HelloWorld
        .chain(HelloWorld::print)
        .chain(HelloWorld::print)
        .chain(HelloWorld::print)
        .result;

    // Hello, world!
    // Hello, world!
    // Hello, world!
    // It works!
}
```

### Mutable call chaining

```rust
use chainer::*;

struct Counter { value: i32 }
impl Counter {
    fn increment(&mut self) -> i32 {
        self.value += 1;
        println!("{}", self.value);
        self.value
    }
}

fn main() {
    let value: i32 = Counter { value: 0 }
        .chain_mut(Counter::increment)
        .chain_mut(Counter::increment)
        .chain_mut(Counter::increment)
        .result;

    println!("{value}");

    // 1
    // 2
    // 3
    // 3
}
```