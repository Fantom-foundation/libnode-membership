//! The internal local node failure detector.
//!
//! It is internal because all the code to detect node failures is located within this library.
//!
//! It is local because in order to find node failures it only uses the knowledge of the node on
//! which it is running.

use failure::Fail;

use crate::graph::NodeId;

/// A failure detector error.
#[derive(Debug, Fail)]
pub enum Error {
    /// A polling error.
    #[fail(display = "Polling error")]
    Poll,
}

/// A tentative interface to a node failure detector.
pub trait FailureDetector<N: NodeId> {
    /// Finds any new failures and appends those to the failure queue.
    fn poll_failures(&mut self) -> Result<(), Error>;

    /// Takes any unhandled node failures for processing and removes those from the queue of
    /// unhandled failures.
    fn dequeue_failures(&mut self) -> Vec<N>;
}

/// The internal node failure detector.
pub struct InternalFailureDetector<N: NodeId> {
    /// The queue of unhandled node failures.
    failures: Vec<N>,
}

impl<N: NodeId> FailureDetector<N> for InternalFailureDetector<N> {
    fn poll_failures(&mut self) -> Result<(), Error> {
        // TODO
        Err(Error::Poll)
    }

    fn dequeue_failures(&mut self) -> Vec<N> {
        self.failures.drain(..).collect()
    }
}

impl<N: NodeId> InternalFailureDetector<N> {
    /// Creates a new failure detector.
    pub fn new() -> Self {
        InternalFailureDetector {
            failures: Vec::new(),
        }
    }
}
