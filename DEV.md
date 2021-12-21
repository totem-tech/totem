# Usual issues and quirks

This document is intended to keep the information useful to the developers.

## Assert failure

```
error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
   |
   | assert_eq_size!(usize, u32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

That means that a crate dependency has been added without updating the `std` feature.

## Updating the branch of dependencies

Just after changing the branch of a git dependency, run the script `./scripts/update-deps.sh`
before causing any update to the `Cargo.lock`. Otherwise, the dependencies cannot be updated anymore.

That means that RLS must be deactivated before.
