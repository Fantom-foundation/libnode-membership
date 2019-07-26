//! `Graph` is a gossip graph with an interface to external networking:
//!
//! - `Graph::poll` queries the configured local failure detector for any new failures, and outputs
//! gossip messages for the external networking layer to send to remote nodes.
//!
//! - `Graph::handle_message` handles a message received by the networking layer from a remote node.

use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

use bit_set::BitSet;
use failure::Fail;
use serde::{Deserialize, Serialize};

use crate::hash::{compute_hash, Error as HashError, Hash};

/// A peer node's unique identifier.
pub trait NodeId: Eq + Ord + Clone + Debug + Send + Serialize + Sync {}
impl<N> NodeId for N where N: Eq + Ord + Clone + Debug + Send + Serialize + Sync {}

/// Observations of network events.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Observation<N: NodeId> {
    /// The genesis group.
    Genesis(BTreeSet<N>),
    Add(N),
    Remove(N),
}

/// A gossip event.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event<N: NodeId> {
    creator_id: N,
    self_parent: Option<Hash>,
    other_parent: Option<Hash>,
    observation: Observation<N>,
}

/// A reference to an `Event`, and its index in the gossip graph.
#[derive(Clone, Debug)]
pub struct EventRef<'a, N: NodeId + 'a> {
    pub event: &'a Event<N>,
    pub index: usize,
}

impl<'a, N: NodeId> PartialEq for EventRef<'a, N> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<'a, N: NodeId> Eq for EventRef<'a, N> {}

impl<'a, N: NodeId> PartialOrd for EventRef<'a, N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.index.partial_cmp(&other.index)
    }
}

impl<'a, N: NodeId> Ord for EventRef<'a, N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

/// A gossip graph error.
#[derive(Debug, Fail)]
pub enum Error {
    /// A failure detector error.
    #[fail(display = "Failure detector error")]
    FailureDetector,
    /// A hasher error.
    #[fail(display = "Hasher error: {}", _0)]
    Hash(HashError),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Message<N: NodeId> {
    Event(Event<N>),
}

unsafe impl<N: NodeId> Send for Message<N> {}
unsafe impl<N: NodeId> Sync for Message<N> {}

/// A gossip graph.
#[derive(Clone, Debug)]
pub struct Graph<N>
where
    N: NodeId,
{
    /// All events in the graph.
    events: Vec<Event<N>>,
    /// A mapping of event hashes to indices of the corresponding events in `events`.
    indices: BTreeMap<Hash, usize>,
}

impl<N> Default for Graph<N>
where
    N: NodeId,
{
    fn default() -> Self {
        Self {
            events: Vec::new(),
            indices: BTreeMap::new(),
        }
    }
}

impl<N> Graph<N>
where
    N: NodeId,
{
    /// Constructs a new, empty gossip graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the index of an event with the given hash.
    pub fn get_index(&self, hash: &Hash) -> Option<usize> {
        self.indices.get(hash).cloned()
    }

    /// Checks whether this graph contains an event with the given hash.
    pub fn contains(&self, hash: &Hash) -> bool {
        self.indices.contains_key(hash)
    }

    /// Inserts a new event into the graph.
    ///
    /// FIXME: handle hash collisions.
    pub fn insert(&mut self, event: Event<N>) -> Result<EventRef<N>, Error> {
        let hash = compute_hash(&event).map_err(Error::Hash)?;
        let index = match self.indices.entry(hash) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let index = self.events.len();
                self.events.push(event);
                entry.insert(index);
                index
            }
        };
        Ok(EventRef {
            event: &self.events[index],
            index,
        })
    }

    /// Gets the event with the given index, if it exists.
    pub fn get_by_index(&self, index: usize) -> Option<EventRef<N>> {
        self.events
            .get(index)
            .map(|event| EventRef { event, index })
    }

    /// Gets the event with the given hash, if it exists.
    pub fn get_by_hash<'a>(&'a self, hash: &Hash) -> Option<EventRef<'a, N>> {
        self.get_index(hash)
            .and_then(|index| self.get_by_index(index))
    }

    pub fn ancestors<'a>(&'a self, _event: EventRef<'a, N>) -> SubGraphIter<'a, N> {
        let nodes = BTreeSet::new();
        SubGraphIter {
            graph: self,
            // FIXME
            nodes,
            seen: BitSet::new(),
        }
    }

    /// Polls the failure detector for any new failures and outputs messages for the networking
    /// layer to send to remote nodes.
    pub fn poll(&mut self) -> Result<Vec<Message<N>>, Error> {
        // FIXME
        Err(Error::FailureDetector)
    }

    /// Handles an incoming message from the networking layer.
    pub fn handle_message(&mut self, _msg: &Message<N>) -> Result<(), Error> {
        // FIXME
        Ok(())
    }
}

/// The state of an iterator over a subset of nodes in a `Graph`.
pub struct SubGraphIter<'a, N: NodeId + 'a> {
    pub graph: &'a Graph<N>,
    pub nodes: BTreeSet<EventRef<'a, N>>,
    pub seen: BitSet,
}
