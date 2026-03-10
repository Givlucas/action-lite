---
owners: ["Lucas Givens"]
state: discovery
priority: true
continuous: false
---

# Notes
- LLMs can only think while writing. So they should have some space to create markdown documents t o reason with while working through action. Some clients have this built in to their thinking models. But it might be best to make it explicit.
- the recursive nature of action might ballon the context for agents depending on the scope of the problem.

## Action orchestratoion flow from human perspective.

####
1. Check state of all actions not in published state
2. Check for "priority" actions
3. Determine which actions can be worked on next.

for each ready action check the current state and determine which of the following to continue from.

#### Discovery
1. Initial problem  scoping - what is the action we want to preform (Create if not already defined, or understand current action).
2. Initial specs - What we already know we want as a requirements for the out come of the action. Does this action have any inputs?
3. Research problem space - Other solutions tools libraries algorithms internal and external project research
4. Add additional specifications based on research. Re-scope action if necessary.

#### Design
1. Based on research document, plan implementation of action 1-2 levels conceptually down. 1st draft
2. Review design and re-shape if need be. 2nd draft

#### Implementation
1. If at high-level state still, create a meta graph directory and break down into sub actions. To complete this action, continue to follow the action workflow against the sub actions until they reach the "published" state.
2. If design is alread broken down enough continue straight to implementation in code.

#### Test
1. Verify against specifications, and statement of action. Does it meet the original goals? If not, revert to discovery, design, add additional specifications as needed, leave notes on important discoveries in note section.
2. ensure code has good test coverage.
3. Manually test feature. Does it look how you expected it to?

#### Document
1. Write user or developer documentation for the tool. Do not restate action or architecture. Architecture documentation lives in action only

#### Publish
1. Make avaiable for use, create and close a PR, publish to whatever location allows the end rusult to be used.

# Statement of Action


# Statement of Inputs
- [Create action-lite protocol](Create%20action-lite%20protocol.md)
- [Create philosophy document and development tool context](Create%20philosophy%20document%20and%20development%20tool%20context.md)

# Statement of Specifications
- [ ] Orchestration limited to the scope of a single action and its sub-actions. Does not need to implement if at to high a level.
- [ ] Orchestration agent must be able to be used as a "main agent" but should not rely on "CLAUDE.md" file to do so
- [ ] Must include action lite agent skill
- [ ] agents and skills must be packages as a claude plugin

# Statement of Design

