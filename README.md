# Domainname pallet

This is the repository of the domainname pallet.

## Purpose

This pallet gives you the possibility to extend a substrate chain by domain names. A user should have the possibility register a qualified name and use this name to receive funds from other users of the chain without the need of using an address.

## Dependencies

### Traits

This pallet depends on the following traits:

- Currency
- Event

### Pallets

This pallet does not depend on any other FRAME pallet or externally developed modules.

## Installation

### Runtime `Cargo.toml`

To add this pallet to your runtime, simply include the following to your runtime's `Cargo.toml` file:

```TOML
[dependencies.pallet-domainname]
default_features = false
git = 'https://github.com/Janislav/pallet-domainname.git'
```

and update your runtime's `std` feature to include this pallet:

```TOML
std = [
    # --snip--
    'pallet-domainname/std',
]
```

### Runtime `lib.rs`

You should implement it's trait like so:

```rust
/// Used for test_module
impl pallet_template::Config for Runtime {
	type Event = Event;
}
```

and include it in your `construct_runtime!` macro:

```rust
Domainname: pallet_domainname::{Module, Call, Storage, Event<T>},
```

### Genesis Configuration

This template pallet does not have any genesis configuration.

## Reference Docs

You can view the reference docs for this pallet by running:

```
cargo doc --open
```
