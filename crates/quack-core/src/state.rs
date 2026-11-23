use std::collections::BTreeSet;

use super::event::*;

// Tasks

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: TaskId,
    pub details: TaskDetails,
    pub open: bool,
    pub assignees: BTreeSet<ActorId>,
}
