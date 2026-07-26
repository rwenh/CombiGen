//! `Generator<T>` — a small helper that lets recursive search code "yield"
//! values lazily, because Rust has no stable `yield` keyword.
//!
//! The trick:
//! - run the recursive search on a background thread,
//! - hand each result back to the caller through a *rendezvous* channel
//!   (buffer size 0).
//!
//! Because the channel has no buffer, `sender.send(value)` blocks until the
//! consumer calls `.next()` — which is exactly the "pause until someone asks
//! for more" behaviour that `demo_generator_laziness()` relies on (e.g. pulling
//! only the first solution out of `n_queens(12)` without exploring the rest of
//! the search tree).
//!
//! Trade-off: every `Generator` spawns one OS thread. This is a first-migration
//! pass that stays close to the shape of the original recursive functions.
//! Hand-crafted `Iterator` state machines or an async coroutine crate would be
//! lighter for hot paths.

use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread::JoinHandle;

pub struct Generator<T> {
    receiver: Option<Receiver<T>>,
    handle: Option<JoinHandle<()>>,
}

impl<T: Send + 'static> Generator<T> {
    /// Runs `body` on a new thread. `body` should call `tx.send(value)`.
    ///
    /// `send` returns `Err` once the consumer drops this `Generator`
    /// (e.g. it stopped after the first solution).
    pub fn new<F>(body: F) -> Self
    where
        F: FnOnce(SyncSender<T>) + Send + 'static,
    {
        let (tx, rx) = sync_channel(0);
        let handle = std::thread::spawn(move || body(tx));
        Generator {
            receiver: Some(rx),
            handle: Some(handle),
        }
    }
}

impl<T> Iterator for Generator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.receiver.as_ref().and_then(|r| r.recv().ok())
    }
}

impl<T> Drop for Generator<T> {
    fn drop(&mut self) {
        // Close the channel *before* joining. If the worker is parked inside
        // `send`, closing wakes it with an `Err` so it can unwind instead of
        // blocking forever on a receiver nobody will poll.
        self.receiver.take();
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}
