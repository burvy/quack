use std::collections::{BTreeMap, BTreeSet};

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

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub tasks: BTreeMap<TaskId, Task>,
    pub actors: BTreeMap<ActorId, Actor>,
    pub notes: BTreeMap<NoteId, Note>,
}

pub fn apply(state: State, event: &Event) -> State {
    match &event.kind {
        EventKind::Actor(actor_event) => apply_actor(state, actor_event),
        EventKind::Task(task_event) => apply_task(state, task_event),
        EventKind::Note(note_event) => apply_note(state, note_event),
    }
}

fn apply_actor(mut state: State, event: &ActorEvent) -> State {
    match event {
        ActorEvent::Join { id, info } => {
            state.actors.insert(
                *id,
                Actor {
                    id: *id,
                    info: info.clone(),
                    here: true,
                },
            );
            state
        }
        ActorEvent::Leave { id } => {
            if let Some(actor) = state.actors.get_mut(id) {
                actor.here = false;
            };
            state
        }
        ActorEvent::Rejoin { id } => {
            if let Some(actor) = state.actors.get_mut(id) {
                actor.here = true;
            };
            state
        }
    }
}
fn apply_task(state: State, event: &TaskEvent) -> State {
    todo!()
}
fn apply_note(state: State, event: &NoteEvent) -> State {
    todo!()
}
