---
name: philosophy
description: Core development philosophy and design principles. Use when making architectural decisions, designing systems, writing documentation, refactoring code, or when guidance on best practices is needed. Covers design principles, documentation standards, service architecture, code quality, and learning approaches.
---

# Development Philosophy

## Design Principles

### Explicit Design
Code should be explicitly designed. Never throw stuff at a wall to see what sticks—that is incredibly dangerous.

### Verify Assumptions
It is dangerous to assume anything. Always verify core assumptions before proceeding.

### Information First
Before starting an algorithm, ask yourself: what information do you want to know?

### Service Design
1. Start by figuring out what a service does
2. Then figure out what it needs to know
3. If a call always returns the same result for same parameters → make it a library
4. Services only make sense when they change something in the world

### Entity Lifecycle
Many business processes have entities going through milestones. Instead of viewing this as a single entity with booleans or a CURRENT_STATE attribute, view each state as a *different thing*.

See: `references/services.md` for detailed service architecture patterns.

## Documentation Standards

Documentation should capture:
- **What**: What the system does
- **Why**: Why it was built this way  
- **How**: How it accomplishes its goals

### When to Document
- If something is not immediately obvious → document it
- If you can Google it → don't explain it in your docs
- Comments should provide context around code sections

### What to Document
- The goal and purpose of code sections
- Architecture decisions and tool choices
- Features that are relied upon

### What NOT to Document
- Other people's systems (beyond overviews)
- Information easily found in dictionaries or encyclopedias

See: `references/documentation.md` for documentation templates.

## Code Quality

### Refactor First
Before implementing new features, ask: "Can I make implementing this easier?"

### Clean As You Go
Clean up after yourself as you go along. Don't continually generate large messes to clean later.

### Consistency
Consistency reduces cognitive load and allows you to make more assumptions safely, enabling reasoning about higher-order things more easily.

## Process

### Divergent & Convergent Thinking
- **Divergence (Discovery)**: Allow thought process to branch out and expand
- **Convergence (Design)**: Bring all ideas together

- If stuck, you're probably missing something—enter discovery mode

### Action Framework
1. Discovery → gather requirements, add specifications
2. Design → create the design (baked into the action)
3. Implementation → build it
4. Testing → verify against specifications (may cycle back to discovery)

## Learning Approach

### Read to Understand
Actually READ and understand systems. Don't just skim, especially for new systems where you don't understand the patterns.

### New Systems
New systems may use entirely different patterns. Approach them as if you are a new programmer. You know nothing. Be mindful.

### Progressive Learning
- Languages have levels; learn in pieces
- Start with common constructs and constitution words
- Patterns emerge after multiple exposures
- Only tackle things you really need
