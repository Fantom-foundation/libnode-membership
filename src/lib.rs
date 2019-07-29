//! Distributed node group membership library.
//!
//! The public interface of the library has the form of a replicated state machine,
//! `NodeMembership`. The interface connects it with an external networking layer:
//!
//! - `NodeMembership::poll` queries the configured local failure detector for any new failures, and
//! outputs gossip messages for the external networking layer to send to remote nodes.
//!
//! - `NodeMembership::handle_message` handles a message received by the networking layer from a
//! remote node.
//!
//! The set of currently known group members can be obtained by calling
//!
//! - `NodeMembership::group`.

mod failure_detector;
mod graph;
mod hash;
mod node_membership;

pub use node_membership::NodeMembership;
