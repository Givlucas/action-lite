---
owners: ["Lucas Givens"]
state: discovery
priority: true
continuous: false
---

# Notes
- LLMs can only think while writing. So they should have some space to create markdown documents t o reason with while working through action. Some clients have this built in to their thinking models. But it might be best to make it explicit.
- the recursive nature of action might ballon the context for agents depending on the scope of the problem.
- Bot-thoughts-<stage>.md and memory.md documents to pass between stages as scratch pads?

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

