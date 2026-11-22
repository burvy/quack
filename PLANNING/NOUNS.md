All nouns below have an ID assigned to them.
We do this because the event log is append only and immutable
When editing, things have a `base` version they edit from to solve double-editing issues by just accepting 
the person editing the most recent version

What happens on double-saving is yet to be decided. Check `TODO.md`.

Only VERBS are saved in the log, and these nouns are rebuilt from them. 
Occassionally a snapshot is built to load the current state. instead of replaying 
the entire event log.
The snapshot is entirely disposable so you can rebuild it if it's lost or wrong.

## Actor
A `Human` or an `Agent` that is attached to a `Human`
Has `name`, `kind`, for agents it has `owner`
# Messaging stuff
## Message
A message with a `Body` and the `Channel` it belongs to, along with typical message metadata like `Actor`, `Time`, etc... Also, `Parent` field for replies to a message.
## Channel
A channel with a `Title` and `Description` and parent `Channel`
Has `Open` or `Closed`
# Workspace stuff
## Role
Can be attached to an `Actor`. Has a `Title` and `Description`. Can attach `Goal`s and `Task`s and `Know`s that people under the 
role need to know.
Role attaching to tasks and goals is the scope. If tasks or goals arent covered by any role then there might be an issue there.
Has `Open` or `Closed`
## Goal
Goal with `Title`, `Description`, `Parent` goal, `Task`s it contains. Can only be assigned to `Human`s
Has `Open` or `Closed`
## Task
Task with `Title`, `Description`. Has `Parent` task, and is assigned to `Actor`s
Has `Open` or `Closed`
# Other
## Note
Can be attached to ANY noun. Has `Body`
Has `Open` or `Closed`
# Understanding
## Know
A concept with a `Title`, `Description`, `Goal`s and `Task`s. A `Know` can have a list of `Know`s required to unlock it
Has `Open` or `Closed`, depending on if it's useful to the project anymore.
No cycles either.
## Verdict
Links a `Know` to a `Human`. Has `Evidence`, `Know` -> `Human`. 
It can have a claim: `Knows`: bool  
A status: `Proposed`, `Accepted`, `Rejected`
Verdicts also have a `Time`, and can expire, unlinking a `Know` to a human.
