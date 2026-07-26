//! combinatorics.rs — Iterator-based combinatorial primitives
//!
//! The recursive ones (integer_partitions, set_partitions) use
//! `generator::Generator`, since they were built with `yield from`.

use crate::generator::Generator;
use std::sync::mpsc::SyncSender;

// ---------------------------------------------------------------------------
// Permutations
// ---------------------------------------------------------------------------

/// # Examples
/// ```
/// use combigen::combinatorics::permutations;
/// let result: Vec<Vec<i32>> = permutations(&[1, 2, 3], Some(2)).collect();
/// assert_eq!(
///     result,
///     vec![
///         vec![1, 2], vec![1, 3],
///         vec![2, 1], vec![2, 3],
///         vec![3, 1], vec![3, 2]
///     ]
/// );
/// ```
pub fn permutations<T: Clone>(items: &[T], r: Option<usize>) -> Permutations<T> {
    Permutations::new(items, r)
}

pub struct Permutations<T> {
    items: Vec<T>,
    r: usize,
    n: usize,
    indices: Vec<usize>,
    cycles: Vec<usize>,
    first: bool,
    done: bool,
}

impl<T: Clone> Permutations<T> {
    pub fn new(items: &[T], r: Option<usize>) -> Self {
        let items = items.to_vec();
        let n = items.len();
        let r = r.unwrap_or(n);
        let done = r > n;
        let indices: Vec<usize> = (0..n).collect();
        let cycles: Vec<usize> = if !done && r > 0 {
            (n - r + 1..=n).rev().collect()
        } else {
            Vec::new()
        };
        Permutations {
            items,
            r,
            n,
            indices,
            cycles,
            first: true,
            done,
        }
    }

    fn snapshot(&self) -> Vec<T> {
        self.indices[..self.r]
            .iter()
            .map(|&i| self.items[i].clone())
            .collect()
    }
}

impl<T: Clone> Iterator for Permutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.done {
            return None;
        }
        if self.first {
            self.first = false;
            return Some(self.snapshot());
        }
        let mut i = self.r;
        while i > 0 {
            i -= 1;
            self.cycles[i] -= 1;
            if self.cycles[i] == 0 {
                let removed = self.indices[i];
                for k in i..self.n - 1 {
                    self.indices[k] = self.indices[k + 1];
                }
                self.indices[self.n - 1] = removed;
                self.cycles[i] = self.n - i;
            } else {
                let j = self.cycles[i];
                self.indices.swap(i, self.n - j);
                return Some(self.snapshot());
            }
        }
        self.done = true;
        None
    }
}

// ---------------------------------------------------------------------------
// Combinations
// ---------------------------------------------------------------------------

/// # Examples
/// ```
/// use combigen::combinatorics::combinations;
/// let result: Vec<Vec<i32>> = combinations(&[1, 2, 3, 4], 2).collect();
/// assert_eq!(
///     result,
///     vec![
///         vec![1, 2], vec![1, 3], vec![1, 4],
///         vec![2, 3], vec![2, 4],
///         vec![3, 4]
///     ]
/// );
/// ```
pub fn combinations<T: Clone>(items: &[T], r: usize) -> Combinations<T> {
    Combinations::new(items, r)
}

pub struct Combinations<T> {
    items: Vec<T>,
    r: usize,
    indices: Vec<usize>,
    started: bool,
    done: bool,
}

impl<T: Clone> Combinations<T> {
    pub fn new(items: &[T], r: usize) -> Self {
        let items = items.to_vec();
        let n = items.len();
        let done = r > n;
        let indices: Vec<usize> = if done { Vec::new() } else { (0..r).collect() };
        Combinations {
            items,
            r,
            indices,
            started: false,
            done,
        }
    }

    fn snapshot(&self) -> Vec<T> {
        self.indices.iter().map(|&i| self.items[i].clone()).collect()
    }
}

impl<T: Clone> Iterator for Combinations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.done {
            return None;
        }
        let n = self.items.len();
        if !self.started {
            self.started = true;
            if self.r == 0 {
                self.done = true;
            }
            return Some(self.snapshot());
        }
        let mut i = self.r;
        loop {
            if i == 0 {
                self.done = true;
                return None;
            }
            i -= 1;
            if self.indices[i] != i + n - self.r {
                break;
            }
        }
        self.indices[i] += 1;
        for j in (i + 1)..self.r {
            self.indices[j] = self.indices[j - 1] + 1;
        }
        Some(self.snapshot())
    }
}

// ---------------------------------------------------------------------------
// Combinations with replacement
// ---------------------------------------------------------------------------

/// # Examples
/// ```
/// use combigen::combinatorics::combinations_with_replacement;
/// let result: Vec<Vec<i32>> = combinations_with_replacement(&[1, 2], 2).collect();
/// assert_eq!(result, vec![vec![1, 1], vec![1, 2], vec![2, 2]]);
/// ```
pub fn combinations_with_replacement<T: Clone>(
    items: &[T],
    r: usize,
) -> CombinationsWithReplacement<T> {
    CombinationsWithReplacement::new(items, r)
}

pub struct CombinationsWithReplacement<T> {
    items: Vec<T>,
    r: usize,
    indices: Vec<usize>,
    started: bool,
    done: bool,
}

impl<T: Clone> CombinationsWithReplacement<T> {
    pub fn new(items: &[T], r: usize) -> Self {
        let items = items.to_vec();
        let done = items.is_empty() && r > 0;
        CombinationsWithReplacement {
            items,
            r,
            indices: vec![0; r],
            started: false,
            done,
        }
    }

    fn snapshot(&self) -> Vec<T> {
        self.indices.iter().map(|&i| self.items[i].clone()).collect()
    }
}

impl<T: Clone> Iterator for CombinationsWithReplacement<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.done {
            return None;
        }
        let n = self.items.len();
        if !self.started {
            self.started = true;
            if self.r == 0 {
                self.done = true;
            }
            return Some(self.snapshot());
        }
        if n == 0 {
            self.done = true;
            return None;
        }
        let mut i = self.r;
        loop {
            if i == 0 {
                self.done = true;
                return None;
            }
            i -= 1;
            if self.indices[i] != n - 1 {
                break;
            }
        }
        self.indices[i] += 1;
        for j in (i + 1)..self.r {
            self.indices[j] = self.indices[i];
        }
        Some(self.snapshot())
    }
}

// ---------------------------------------------------------------------------
// Integer partitions
// ---------------------------------------------------------------------------

/// # Examples
/// ```
/// use combigen::combinatorics::integer_partitions;
/// let result: Vec<Vec<u32>> = integer_partitions(4, None).collect();
/// assert_eq!(
///     result,
///     vec![
///         vec![4],
///         vec![3, 1],
///         vec![2, 2],
///         vec![2, 1, 1],
///         vec![1, 1, 1, 1]
///     ]
/// );
/// ```
pub fn integer_partitions(n: u32, max_part: Option<u32>) -> Generator<Vec<u32>> {
    let max_part = max_part.unwrap_or(n);
    Generator::new(move |tx| {
        if n == 0 {
            let _ = tx.send(Vec::new());
            return;
        }
        fn go(
            remaining: u32,
            max_val: u32,
            current: &mut Vec<u32>,
            tx: &SyncSender<Vec<u32>>,
        ) -> bool {
            if remaining == 0 {
                return tx.send(current.clone()).is_ok();
            }
            let top = remaining.min(max_val);
            for part in (1..=top).rev() {
                current.push(part);
                let keep_going = go(remaining - part, part, current, tx);
                current.pop();
                if !keep_going {
                    return false;
                }
            }
            true
        }
        let mut current = Vec::new();
        go(n, max_part, &mut current, &tx);
    })
}

// ---------------------------------------------------------------------------
// Set partitions
// ---------------------------------------------------------------------------

/// # Examples
/// ```
/// use combigen::combinatorics::set_partitions;
/// let result: Vec<Vec<Vec<i32>>> = set_partitions(vec![1, 2, 3]).collect();
/// assert_eq!(result.len(), 5); // B(3) = 5
/// assert_eq!(result[0], vec![vec![1, 2, 3]]);
/// ```
pub fn set_partitions<T: Clone + Send + 'static>(items: Vec<T>) -> Generator<Vec<Vec<T>>> {
    Generator::new(move |tx| {
        if items.is_empty() {
            let _ = tx.send(Vec::new());
            return;
        }
        fn go<T: Clone>(
            remaining: &[T],
            blocks: &mut Vec<Vec<T>>,
            tx: &SyncSender<Vec<Vec<T>>>,
        ) -> bool {
            if remaining.is_empty() {
                return tx.send(blocks.clone()).is_ok();
            }
            let first = remaining[0].clone();
            let rest = &remaining[1..];

            for i in 0..blocks.len() {
                blocks[i].push(first.clone());
                let keep_going = go(rest, blocks, tx);
                blocks[i].pop();
                if !keep_going {
                    return false;
                }
            }
            blocks.push(vec![first]);
            let keep_going = go(rest, blocks, tx);
            blocks.pop();
            keep_going
        }
        let mut blocks: Vec<Vec<T>> = Vec::new();
        go(&items, &mut blocks, &tx);
    })
}

// ---------------------------------------------------------------------------
// Power set
// ---------------------------------------------------------------------------

/// # Examples
/// ```
/// use combigen::combinatorics::power_set;
/// let result: Vec<Vec<i32>> = power_set(&[1, 2, 3]).collect();
/// assert_eq!(
///     result,
///     vec![
///         vec![],
///         vec![1], vec![2], vec![3],
///         vec![1, 2], vec![1, 3], vec![2, 3],
///         vec![1, 2, 3]
///     ]
/// );
/// ```
pub fn power_set<T: Clone>(items: &[T]) -> PowerSet<T> {
    PowerSet::new(items)
}

pub struct PowerSet<T> {
    items: Vec<T>,
    r: usize,
    current: Combinations<T>,
}

impl<T: Clone> PowerSet<T> {
    pub fn new(items: &[T]) -> Self {
        let items = items.to_vec();
        let current = Combinations::new(&items, 0);
        PowerSet {
            items,
            r: 0,
            current,
        }
    }
}

impl<T: Clone> Iterator for PowerSet<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        loop {
            if let Some(v) = self.current.next() {
                return Some(v);
            }
            self.r += 1;
            if self.r > self.items.len() {
                return None;
            }
            self.current = Combinations::new(&self.items, self.r);
        }
    }
}

// ---------------------------------------------------------------------------
// Cartesian product
// ---------------------------------------------------------------------------

/// # Examples
/// ```
/// use combigen::combinatorics::cartesian_product;
/// let bits: Vec<Vec<i32>> = cartesian_product(vec![vec![0, 1]], 3).collect();
/// assert_eq!(bits.len(), 8);
/// assert_eq!(bits[0], vec![0, 0, 0]);
/// assert_eq!(bits[7], vec![1, 1, 1]);
/// ```
pub fn cartesian_product<T: Clone>(
    sequences: Vec<Vec<T>>,
    repeat: usize,
) -> CartesianProduct<T> {
    CartesianProduct::new(sequences, repeat)
}

pub struct CartesianProduct<T> {
    pools: Vec<Vec<T>>,
    indices: Vec<usize>,
    started: bool,
    done: bool,
}

impl<T: Clone> CartesianProduct<T> {
    pub fn new(sequences: Vec<Vec<T>>, repeat: usize) -> Self {
        let mut pools = Vec::with_capacity(sequences.len() * repeat);
        for _ in 0..repeat {
            pools.extend(sequences.iter().cloned());
        }
        let done = pools.iter().any(|p| p.is_empty()) && !pools.is_empty();
        let indices = vec![0; pools.len()];
        CartesianProduct {
            pools,
            indices,
            started: false,
            done,
        }
    }

    fn snapshot(&self) -> Vec<T> {
        self.indices
            .iter()
            .zip(self.pools.iter())
            .map(|(&i, pool)| pool[i].clone())
            .collect()
    }
}

impl<T: Clone> Iterator for CartesianProduct<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.done {
            return None;
        }
        if !self.started {
            self.started = true;
            if self.pools.is_empty() {
                self.done = true;
            }
            return Some(self.snapshot());
        }
        if self.pools.is_empty() {
            self.done = true;
            return None;
        }
        let mut i = self.pools.len();
        loop {
            if i == 0 {
                self.done = true;
                return None;
            }
            i -= 1;
            self.indices[i] += 1;
            if self.indices[i] < self.pools[i].len() {
                break;
            }
            self.indices[i] = 0;
        }
        Some(self.snapshot())
    }
}
