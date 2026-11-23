use std::collections::BTreeSet;

use super::event::*;

// Tasks

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: TaskId,
    pub info: TaskInfo,
    pub open: bool,
    pub crew: BTreeSet<ActorId>,
}

// Actors

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Actor {
    pub id: ActorId,
    pub info: ActorInfo,
    /// Tracks if this actor is `here` in the workspace
    /// Join/Rejoin: here -> true
    /// Leave: here -> false
    ///
    /// For "online status", use something like `live` or `online`
    pub here: bool,
}

// Notes

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Note {
    pub id: NoteId,
    pub info: NoteInfo,
    pub open: bool,
}
