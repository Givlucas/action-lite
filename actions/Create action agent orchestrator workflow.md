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
- Probably will want to pass through the original prompt as well, or atleast track it in a file

# Statement of Action


# Statement of Inputs
- [Create action-lite protocol](Create%20action-lite%20protocol.md)
- [Create development philosophy claude file skill and development tool context](Create%20development%20philosophy%20claude%20file%20skill%20and%20development%20tool%20context.md)

# Statement of Specifications
- Orchestration limited to the scope of a single action and its sub-actions. Does not need to implement if at too high a level.
- Orchestration agent must be able to be used as a "main agent" but should not rely on "CLAUDE.md" file to do so
- Must include action lite agent skill
- Agents and skills must be packaged as a claude plugin

# Statement of Design

