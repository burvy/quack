use uuid::Uuid;

// Actors

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActorEvent {
    Join { id: ActorId, details: ActorDetails },
    Leave { id: ActorId },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActorId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActorDetails {
    pub name: String,
    pub kind: ActorKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActorKind {
    Human,
    Agent { owner: ActorId },
}

// Tasks

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskEvent {
    Open {
        id: TaskId,
        details: TaskDetails,
    },
    Edit {
        id: TaskId,
        base: EventId,
        details: TaskDetails,
    },
    Close {
        id: TaskId,
    },
    Reopen {
        id: TaskId,
    },
    Assign {
        id: TaskId,
        to: ActorId,
    },
    Unassign {
        id: TaskId,
        to: ActorId,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskDetails {
    pub title: String,
    pub description: String,
    pub parent: Option<TaskId>,
}

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
    pub by: ActorId,
    pub kind: EventKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventKind {
    Actor(ActorEvent),
    Task(TaskEvent),
}
