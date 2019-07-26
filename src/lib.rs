//! `libnode-membership` is a distributed node group membership library.

mod graph;
mod hash;

pub use crate::graph::{NodeId, Observation, Event, Graph, Error};
