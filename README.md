# combigen

Rust port of the `combinatorics.py` / `backtracking.py` toolkit. Verified
against the same assertions as `tests.py` (31 tests) plus 7 doctests
translated from the Python docstrings — all passing.

## Layout

| Python              | Rust                    |
|----------------------|--------------------------|
| `combinatorics.py`  | `src/combinatorics.rs`  |
| `backtracking.py`   | `src/backtracking.rs`   |
| `demo.py`            | `src/bin/demo.rs`       |
| `tests.py`           | `tests/tests.rs`        |
| *(new)*              | `src/generator.rs`      |

## Why `src/generator.rs` exists

Python's `yield` has no stable Rust equivalent, and the two files use it
two different ways:

- **Already-iterative** functions (`permutations`, `combinations`,
  `combinations_with_replacements`, `power_set`, `cartesian_product`)
  became hand-written `Iterator` state machines — the same technique
  CPython's own `itertools` uses in C. No tricks needed, and no runtime
  overhead beyond a normal iterator.
- **Recursive, `yield from`-based** functions (`backtrack`, `n_queens`,
  `solve_sudoku`, `find_paths`, `integer_partitions`, `set_partitions`)
  became `Generator<T>`: the recursive search runs on a worker thread and
  streams results back over a zero-capacity channel, which blocks the
  producer until the consumer calls `.next()`. That's what makes
  `n_queens(12).next()` return in ~280µs instead of computing all 14,200
  solutions (~291ms) first — the same laziness `next(n_queens(12))` gets
  you in the Python version.

  The cost is one OS thread per generator call. That's a fair price for a
  first migration pass that needs to stay line-for-line close to the
  original recursive algorithms — but if a specific solver turns out to be
  a hot path, the natural next step is a hand-rolled stack-based
  `Iterator` (no thread), styled like `combinatorics::Permutations`, or
  swapping in an async coroutine crate like `genawaiter`.

## Other deliberate deviations from the Python

- `cartesian_product` is now genuinely lazy. The Python version already
  claims to be lazy in its docstring, but actually builds the full product
  into a list before yielding any of it (`result = [...]` is fully
  materialized ahead of `yield from result`). The Rust version is a real
  odometer that advances one step per `.next()`.
- `n_queens` uses `Vec<bool>` lookup tables instead of `HashSet<i32>` for
  `cols_used`/`diag1_used`/`diag2_used`, since the index domain is small
  and fixed — avoids hashing on the hottest loop in the module.
- `parse_sudoku` returns `Result<Grid, String>` instead of raising.

## Build / run / test

```bash
cargo build
cargo run --bin demo
cargo test          # unit + doctests, mirrors tests.py
```
