use uuid::Uuid;

// Actors

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActorEvent {
    Join { id: ActorId, info: ActorInfo },
    Leave { id: ActorId },
    Rejoin { id: ActorId },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActorId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActorInfo {
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
        info: TaskInfo,
    },
    Edit {
        id: TaskId,
        base: EventId,
        info: TaskInfo,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskInfo {
    pub title: String,
    pub description: String,
    pub parent: Option<TaskId>,
}

// Notes

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NoteId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteInfo {
    pub targets: Vec<NoteTarget>,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoteEvent {
    Open {
        id: NoteId,
        info: NoteInfo,
    },
    Edit {
        id: NoteId,
        base: EventId,
        info: NoteInfo,
    },
    Close {
        id: NoteId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoteTarget {
    Actor(ActorId),
    Task(TaskId),
    Event(EventId),
    Note(NoteId),
}

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
    Note(NoteEvent),
}
