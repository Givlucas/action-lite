# Service Architecture Patterns

## When to Use Services vs Libraries

**Use a Library when:**
- A call always results in the same result for the same parameters
- No state changes occur
- Avoids operational overhead

**Use a Service when:**
- The operation changes something in the world
- State and state changes are core concerns
- External dependencies or side effects are involved

## Entity Lifecycle Pattern

Many business processes have entities that go through a series of milestones. In a particular state, changes are allowed to certain attributes but not others.

### Anti-patterns
- Single entity with many booleans
- CURRENT_STATE attribute implying unknown state machine

### Recommended Pattern
View each state as a *different thing*. This makes the allowed operations at each stage explicit and the transitions clear.

Example: An order might be:
- `DraftOrder` → can modify items, quantities
- `SubmittedOrder` → can cancel, cannot modify
- `FulfilledOrder` → read-only, can initiate returns

Reference: https://www.michaelnygard.com/blog/2018/01/services-by-lifecycle/

## Determining What a Service Needs to Know

1. Start by figuring out what a service does
2. Then determine what information it needs to accomplish that
3. Don't start with the data model—start with the behavior
