use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt::Debug;
use std::iter;

use failure::Fail;
use serde::{Deserialize, Serialize};

use crate::hash::{compute_hash, Error as HashError, Hash};

/// A peer node's unique identifier.
pub trait NodeId: Eq + Ord + Clone + Debug + Send + Serialize + Sync {}
impl<N> NodeId for N where N: Eq + Ord + Clone + Debug + Send + Serialize + Sync {}

/// Group membership actions.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Action<N: NodeId> {
    /// Register the initial group.
    Init(BTreeSet<N>),
    /// A proposal to add a node.
    Add(N),
    /// A proposal to remove a node.
    Remove(N),
}

/// A gossip event.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event<N: NodeId> {
    /// The ID of the creator of the event.
    creator_id: N,
    /// The hash of the self-parent event.
    self_parent: Option<Hash>,
    /// The hash of the other-parent event.
    other_parent: Option<Hash>,
    /// The event action.
    action: Action<N>,
}

impl<N: NodeId> Event<N> {
    /// The ID of the creator of the event.
    pub fn creator_id(&self) -> &N {
        &self.creator_id
    }

    /// The index of the self-parent of the event.
    pub fn self_parent(&self) -> Option<&Hash> {
        self.self_parent.as_ref()
    }

    /// The index of the other-parent of the event.
    pub fn other_parent(&self) -> Option<&Hash> {
        self.other_parent.as_ref()
    }

    /// The event action.
    pub fn action(&self) -> &Action<N> {
        &self.action
    }
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

impl<'a, N: NodeId + 'a> EventRef<'a, N> {
    /// The ID of the creator of the event.
    pub fn creator_id(&self) -> &N {
        &self.event.creator_id()
    }

    /// The hash of the self-parent of the event.
    pub fn self_parent(&self) -> Option<&Hash> {
        self.event.self_parent.as_ref()
    }

    /// The hash of the other-parent of the event.
    pub fn other_parent(&self) -> Option<&Hash> {
        self.event.other_parent.as_ref()
    }

    /// The event action.
    pub fn action(&self) -> &Action<N> {
        &self.event.action
    }
}

/// A gossip graph error.
#[derive(Debug, Fail)]
pub enum Error {
    /// A hasher error.
    #[fail(display = "Hasher error: {}", _0)]
    Hash(HashError),
}

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

    /// Gets the event with a given index, if it exists.
    pub fn get_by_index(&self, index: usize) -> Option<EventRef<N>> {
        self.events
            .get(index)
            .map(|event| EventRef { event, index })
    }

    /// Gets the event with a given hash, if it exists.
    pub fn get_by_hash<'a>(&'a self, hash: &Hash) -> Option<EventRef<'a, N>> {
        self.get_index(hash)
            .and_then(|index| self.get_by_index(index))
    }

    /// Gets all the ancestors of an event in the graph.
    pub fn ancestors<'a>(&'a self, event: EventRef<'a, N>) -> AncestorIter<'a, N> {
        AncestorIter {
            graph: self,
            queue: iter::once(event).collect(),
        }
    }
}

/// The state of an iterator over the ancestors of an `Event` in a `Graph`.
pub struct AncestorIter<'a, N: NodeId + 'a> {
    /// The original graph.
    pub graph: &'a Graph<N>,
    /// The queue of nodes to be traversed through to their ancestors.
    pub queue: VecDeque<EventRef<'a, N>>,
}

impl<'a, N: NodeId + 'a> Iterator for AncestorIter<'a, N> {
    type Item = EventRef<'a, N>;

    fn next(&mut self) -> Option<Self::Item> {
        let event = self.queue.pop_back()?;
        let mut add_parent = |parent: Option<&Hash>| {
            parent
                .and_then(|hash| self.graph.get_by_hash(hash))
                .map(|parent| self.queue.push_front(parent))
        };
        add_parent(event.other_parent());
        add_parent(event.self_parent());
        Some(event)
    }
}
