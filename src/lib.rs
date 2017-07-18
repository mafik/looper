//! Clean abstraction for a single-threaded event loop. Built as a lightweight wrapper around the `std::sync::mpsc` package.
//!
//! # Example usage:
//!
//! ```rust
//! use looper::{Handler, Sender, run};
//!
//! struct ExampleHandler {
//!     data: Vec<i32>,
//! }
//!
//! impl Handler<i32> for ExampleHandler {
//!
//!     // Invoked right after the `run` function is called.
//!     fn start(&mut self, sender: Sender<i32>) {
//!         for elem in vec![1, 2, 3] {
//!             sender.send(elem.clone()).unwrap();
//!         }
//!     }
//!
//!     // Called for every `event` sent to the `sender`.
//!     fn handle(&mut self, i: i32) -> bool {
//!         self.data.push(i);
//!         true
//!     }
//!
//!     // Called after last event is processed or an error occurs.
//!     fn end(self) {
//!         assert_eq!(self.data, vec![1, 2, 3]);
//!     }
//! }
//!
//! // Blocks the current thread until all events are processed.
//! run(ExampleHandler { data: vec![] });
//!
//! ```

#[doc(no_inline)]
pub use std::sync::mpsc::Sender;

#[cfg(test)]
mod tests;

/// Handles events sent to the event loop.
pub trait Handler<EVENT: Send>: Sized {
    /// Called immediately after starting the event loop.
    ///
    /// The `Sender` argument can be used to send new events. It can be cloned and passed to other threads in this method.
    ///
    /// When the last sender is dropped and there are no events pending, the event loop terminates.
    fn start(&mut self, sender: Sender<EVENT>);

    /// Called for every event sent to the event loop.
    fn handle(&mut self, event: EVENT) -> bool;

    /// Called after event loop terminates.
    ///
    /// The default implementation does nothing and can be overriden.
    fn end(self) {}
}

/// Runs the event loop on the current thread.
pub fn run<EVENT: Send, HANDLER: Handler<EVENT>>(mut handler: HANDLER) {
    let (tx, rx) = std::sync::mpsc::channel();
    handler.start(tx);
    let mut running = true;
    while running {
        running = match rx.recv() {
            Ok(event) => handler.handle(event),
            _ => false,
        }
    }
    handler.end();
}
