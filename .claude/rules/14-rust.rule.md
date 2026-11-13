# Rust rules

Rust code in every crate of this repository. Loaded in every session. The general rules apply first; this file adds what is specific to Rust.

## Tests

1. **Tests live outside the source folder, inside the project.** Every test of a crate lives in that crate's folder, outside its `src/`. No file under `src/` holds a test: no `#[test]` function, no `#[cfg(test)]` module and no doc comment whose example Cargo runs as a doc test. What only the tests use, such as fixtures, goldens and test helpers, lives with them, outside `src/`. A test never leaves its crate for another crate or a folder of its own elsewhere in the repository (testing rules, item 1).

```text
Do:     <crate>/tests/<name>.rs
Don't:  <crate>/src/<name>.rs holding a #[cfg(test)] module
        <crate>/src/<name>_test.rs
        tests/<crate>/<name>.rs, outside the crate
```

## Definitions and implementations

2. **Definitions and implementations live in separate files.** A definition declares a type or a trait: a `struct`, an `enum` or a `trait`. An implementation is an `impl` block, whether it gives a type its own methods or implements a trait for it. A file that declares a type or a trait holds no `impl` block, and a file that holds an `impl` block declares no type or trait. A `#[derive]` stays on its definition.

```rust
// Do: reservation.rs
pub struct Reservation {
	pub estimate: Money,
	pub margin: Money,
}

// Do: reservation_total.rs
impl Reservation {
	pub fn total(&self) -> Money { … }
}

// Don't: reservation.rs
pub struct Reservation { … }

impl Reservation { … }
```

## Formatting

3. **rustfmt formats as Biome does.** Tabs shown two wide, a line width of 80, and every struct literal with one field per line. The crate's `rustfmt.toml` sets them (`hard_tabs`, `tab_spaces = 2`, `max_width = 80`, `struct_lit_width = 0`), and `cargo fmt --check` passes before a commit.

```rust
// Do:
let session = Session {
	directory,
	sink,
};

// Don't:
let session = Session { directory, sink };
```
