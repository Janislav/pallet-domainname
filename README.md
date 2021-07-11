# Domain name pallet

This is the repository of the domain name pallet.

## Purpose

This pallet acts as a template for building other pallets.

It currently allows a user to put a `u32` value into storage, which triggers a runtime event.

## Dependencies

### Traits

This pallet depends on the following traits:

- Currency

### Pallets

This pallet does not depend on any other FRAME pallet or externally developed modules.

## Installation

### Runtime `Cargo.toml`

To add this pallet to your runtime, simply include the following to your runtime's `Cargo.toml` file:

```TOML
[dependencies.pallet-domainname]
default_features = false
git = 'https://github.com/substrate-developer-hub/substrate-pallet-template.git'
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

## Pallet design

# In the following section we define the pallet design

- Types
- Trait
  - Event
  - Currency
- Storage
  - Domains map Vec<u8> => accountId
- Calls
  - claim
  - send
  - unregister
- Events
  - Claimed
  - Sended
  - Unregistered
- Errors
  - DomainAlreadyUsed
  - AccountNotFound
- Module
