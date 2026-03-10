---
owners: ["Lucas Givens"]
state: discovery
priority: true
continuous: false
---

# Notes
- LLMs can only think while writing. So they should have some space to create markdown documents t o reason with while working through action. Some clients have this built in to their thinking models. But it might be best to make it explicit.

## Action orchestratoion flow from human perspective.

####
1. Check state of all actions not in published state
2. Check for "priority" actions
3. Determine which actions can be worked on next.

#### Discovery
1. Initial problem  scoping - what is the action we want to preform (Create if not already defined, or understand current action).
2. Initial specs - What we already know we want as a requirements for the out come of the action. Does this action have any inputs?
3. Research problem space - Other solutions tools libraries algorithms internal and external project research
4. Add additional specifications based on research. Re-scope action if necessary.

#### Design
1. Based on research document, plan implementation of action 1-2 levels conceptually down. 1st draft
2. Review design and re-shape if need be. 2nd draft
3. If at high-level state still, create a meta graph directory and break down into sub actions till they finish "design" do not implement. If at sufficeint implementation level already, then move to next step without meta graph

-- Organize sub actions and then return --

#### Implementation
1. If sub actions where created in prvious step and are ready for implementation. Implement them.
2. If sub action fails, re-evaulate sub action. If sub action failure was caused by overal design issue, revert super action to design or discover based on user discretion




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

