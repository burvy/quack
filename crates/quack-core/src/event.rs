use uuid::Uuid;

// Actors

/// A human or an agent owned by a human
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Actor {
    pub name: String,
    pub id: ActorId,
    pub kind: ActorKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActorKind {
    Human,
    Agent { owner: ActorId },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActorId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActorEvent {
    Join { actor: Actor },
    Leave { actor: ActorId },
}

// Tasks

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(pub Uuid);

// Notes

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoteId(pub Uuid);

// Events

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: EventId,
    pub at: Timestamp,
    pub actor: ActorId,
    pub kind: EventKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EventKind {
    Actor(ActorEvent),
}
