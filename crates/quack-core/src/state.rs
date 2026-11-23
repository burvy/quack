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
    ///
    /// DON'T USE THIS!
    /// USE `.here()`
    /// If an agent is here in the workspace and the human it belongs to
    /// is not, this field will be `true` for the agent but `false` in
    /// `.here()`. Only use this if you
    ///
    /// Join/Rejoin: here -> true
    /// Leave: here -> false
    ///
    /// For "online status", use something like `live` or `online`
    here: bool,
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

impl State {
    /// Tracks if an actor is in the workspace.
    ///
    /// For Human IDs, this reflects the actual state of their `here` field.
    ///
    /// For Agent IDs, they are only here if their owner is here.
    ///
    /// If the ID passed in was not found or the owner of an agent does not exist,
    /// this method returns None
    pub fn is_here(&self, id: ActorId) -> Option<bool> {
        if let Some(actor) = self.actors.get(&id) {
            match actor.info.kind {
                ActorKind::Human => Some(actor.here),
                ActorKind::Agent { owner } => self
                    .actors
                    .get(&owner)
                    .map(|owner_info| owner_info.here && actor.here),
            }
        } else {
            None
        }
    }

    /// Tracks if an actor is dormant.
    ///
    /// For Human IDs, this will always be false, because Humans are either here or not.
    ///
    /// For Agent IDs, if the agent is here but the human is not, they are dormant.
    ///
    /// If the ID passed in was not found or the owner of an agent does not exist,
    /// this method returns None
    pub fn is_dormant(&self, id: ActorId) -> Option<bool> {
        if let Some(actor) = self.actors.get(&id) {
            match actor.info.kind {
                ActorKind::Human => Some(false),
                ActorKind::Agent { owner } => self
                    .actors
                    .get(&owner)
                    .map(|owner_info| !owner_info.here && actor.here),
            }
        } else {
            None
        }
    }
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
        // TODO: An actor joining with an existing ID will
        // overwrite the existing info. Please decide.
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
            }
            state
        }
        ActorEvent::Rejoin { id } => {
            if let Some(actor) = state.actors.get_mut(id) {
                actor.here = true;
            }
            state
        }
    }
}
fn apply_task(mut state: State, event: &TaskEvent) -> State {
    match event {
        TaskEvent::Open { id, info } => {
            state.tasks.insert(
                *id,
                Task {
                    id: *id,
                    info: info.clone(),
                    open: true,
                    crew: BTreeSet::new(),
                },
            );
            state
        }
        TaskEvent::Edit { id, info, .. } => {
            if let Some(task) = state.tasks.get_mut(id) {
                task.info = info.clone();
            }
            state
        }
        TaskEvent::Close { id } => {
            if let Some(task) = state.tasks.get_mut(id) {
                task.open = false;
            }
            state
        }
        TaskEvent::Reopen { id } => {
            if let Some(task) = state.tasks.get_mut(id) {
                task.open = true;
            }
            state
        }
        TaskEvent::Assign { id, to } => {
            if let Some(task) = state.tasks.get_mut(id) {
                task.crew.insert(*to);
            }
            state
        }
        TaskEvent::Unassign { id, to } => {
            if let Some(task) = state.tasks.get_mut(id) {
                task.crew.remove(to);
            }
            state
        }
    }
}
fn apply_note(mut state: State, event: &NoteEvent) -> State {
    match event {
        NoteEvent::Open { id, info } => {
            state.notes.insert(
                *id,
                Note {
                    id: *id,
                    info: info.clone(),
                    open: true,
                },
            );
            state
        }
        NoteEvent::Edit { id, info, .. } => {
            if let Some(note) = state.notes.get_mut(id) {
                note.info = info.clone();
            }
            state
        }
        NoteEvent::Close { id } => {
            if let Some(note) = state.notes.get_mut(id) {
                note.open = false;
            }
            state
        }
        NoteEvent::Reopen { id } => {
            if let Some(note) = state.notes.get_mut(id) {
                note.open = true;
            }
            state
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_agent_and_human_joining_and_rejoining() {
        let alice = ActorId(Uuid::from_u128(67));
        let alice_joins = Event {
            id: EventId(Uuid::from_u128(6767)),
            at: Timestamp(671),
            by: alice,
            kind: EventKind::Actor(ActorEvent::Join {
                id: alice,
                info: ActorInfo {
                    name: "Alice".to_string(),
                    kind: ActorKind::Human,
                },
            }),
        };
        let alyx = ActorId(Uuid::from_u128(6742));
        let alyx_joins = Event {
            id: EventId(Uuid::from_u128(6768)),
            at: Timestamp(672),
            by: alyx,
            kind: EventKind::Actor(ActorEvent::Join {
                id: alyx,
                info: ActorInfo {
                    name: "Alyx".to_string(),
                    kind: ActorKind::Agent { owner: alice },
                },
            }),
        };

        let mut state = apply(State::default(), &alice_joins);
        state = apply(state, &alyx_joins);

        assert!(state.is_here(alice).unwrap());
        assert!(state.is_here(alyx).unwrap());
        let alice_leaves = Event {
            id: EventId(Uuid::from_u128(6769)),
            at: Timestamp(673),
            by: alice,
            kind: EventKind::Actor(ActorEvent::Leave { id: alice }),
        };

        state = apply(state, &alice_leaves);

        assert!(!state.is_here(alice).unwrap());
        assert!(!state.is_here(alyx).unwrap()); // her agent also leaves
        assert!(state.is_dormant(alyx).unwrap());
        assert!(!state.is_dormant(alice).unwrap()); // humans are never dormant

        let alice_rejoins = Event {
            id: EventId(Uuid::from_u128(6770)),
            at: Timestamp(674),
            by: alice,
            kind: EventKind::Actor(ActorEvent::Rejoin { id: alice }),
        };

        state = apply(state, &alice_rejoins);

        assert!(state.is_here(alice).unwrap());
        assert!(state.is_here(alyx).unwrap()); // her agent also rejoins
        assert!(!state.is_dormant(alyx).unwrap()); // no longer dormant
        assert!(!state.is_dormant(alice).unwrap()); // humans are never dormant
    }
}
