## Actor
Join { actor: Human | Agent { owner: actor_id } }
Leave { actor }
Rejoin { actor_id }

# Redact
## Redact
Redact { event: id, reason }
# Messaging stuff
## Message
Add { body, channel, parent=None } # message contents are not stored in log
Edit { body } # only allows if editor is original actor
Delete { message } # message contents can be permanently deleted, but the app will know a message was deleted.
## Channel
Open { channel }
Edit { channel }
Close { channel } # only uses the id
Reopen { channel }
# Workspace stuff
## Role
Open { role }
Edit { role } # you can edit what goals/tasks lie under the role
Close { role } # only uses the id
Reopen { role }
Assign { role, actor }
Unassign { role, actor }
## Goal
Open { goal }
Edit { goal }
Close { goal } # only uses the id
Reopen { goal }
## Task
Open { task }
Edit { task }
Close { task } # this only uses the task id, attach note if you want
Reopen { task }
Assign { task, actor }
Unassign { task, actor }
# Other
## Note
Open { note, thing } # allow notes to be attached onto verbs too
Close { note } # only uses the id
Reopen { note, thing }
Reopen { note }
# Understanding
## Know
Open { know }
Edit { know }
Close { know } # only uses the id
Reopen { know }
## Verdict
Propose { know, human, claim, evidence }
Accept  { verdict }
Reject  { verdict }
