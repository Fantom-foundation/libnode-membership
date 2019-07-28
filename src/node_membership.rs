//! `NodeMembership` is a replicated node membership state machine with an interface to an external
//! networking layer:
//!
//! - `NodeMembership::poll` queries the configured local failure detector for any new failures, and
//! outputs gossip messages for the external networking layer to send to remote nodes.
//!
//! - `NodeMembership::handle_message` handles a message received by the networking layer from a
//! remote node.

use failure::Fail;
use serde::{Deserialize, Serialize};

use crate::failure_detector::{
    Error as FailureDetectorError, FailureDetector, InternalFailureDetector,
};
use crate::graph::{Error as GraphError, Event, Graph, NodeId};

/// A node membership error.
#[derive(Debug, Fail)]
pub enum Error {
    /// A failure detector error.
    #[fail(display = "Failure detector error: {}", _0)]
    FailureDetector(FailureDetectorError),
    /// A gossip graph error.
    #[fail(display = "Gossip graph error: {}", _0)]
    Graph(GraphError),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Message<N: NodeId> {
    Event(Event<N>),
}

unsafe impl<N: NodeId> Send for Message<N> {}
unsafe impl<N: NodeId> Sync for Message<N> {}

/// The state of node group membership.
pub struct NodeMembership<N: NodeId> {
    /// The gossip graph local to this node.
    graph: Graph<N>,
    /// The failure detector subsystem.
    failure_detector: Box<dyn FailureDetector<N>>,
}

impl<N: NodeId + 'static> Default for NodeMembership<N> {
    fn default() -> Self {
        NodeMembership {
            graph: Graph::new(),
            failure_detector: Box::new(InternalFailureDetector::new()),
        }
    }
}

impl<N: NodeId + 'static> NodeMembership<N> {
    /// Constructs a new state of group membership.
    pub fn new() -> NodeMembership<N> {
        NodeMembership::default()
    }

    pub fn graph(&self) -> &Graph<N> {
        &self.graph
    }

    /// Polls the failure detector for any new failures and outputs messages for the networking
    /// layer to send to remote nodes.
    pub fn poll(&mut self) -> Result<Vec<Message<N>>, Error> {
        self.failure_detector
            .poll_failures()
            .map_err(Error::FailureDetector)?;
        let _failures = self.failure_detector.dequeue_failures();
        // TODO: emit events
        Ok(Vec::new())
    }

    /// Handles an incoming message from the networking layer.
    pub fn handle_message(&mut self, _msg: &Message<N>) -> Result<Vec<Message<N>>, Error> {
        // FIXME
        Ok(Vec::new())
    }

    /// Returns the currently known group members.
    pub fn group(&self) -> Vec<N> {
        // FIXME
        Vec::new()
    }
}
