# Documentation Standards

## The Three Questions

All documentation should answer:

1. **What** - What does this system/code do?
2. **Why** - Why was it built this way? What problem does it solve?
3. **How** - How does it accomplish its goals?

This allows you to describe a system before implementing it and answer: "Is this doing what it's supposed to be doing?"

## Code Comments

Comments should provide context. The why and goal of a section of code should be captured in comments around that code.

**Good comment:**
```
// Calculate shipping cost based on weight tiers
// Business rule: orders over 50lbs get freight pricing
```

**Bad comment:**
```
// Loop through items
```

## Architecture Documentation

Architecture documentation should capture:
- What tools are used
- What features are utilized/relied upon
- Integration points and dependencies

### Storage
- Store alongside source code for version control
- Location doesn't matter as long as it's consistent
- Ensure it's easily discoverable and accessible

## User Documentation

User documents should live alongside source code because:
- First resource users will see
- Travels with the code
- Includes pointers to architectural docs, build steps

## What NOT to Document

- Other people's systems (beyond overview/introductions)
- Information easily found via Google, dictionaries, or encyclopedias
- Key assumptions you rely on (unless critical to understanding)

## Writing Style

Use concise and specific wording. Communication and reasoning about a topic can be very difficult if imprecise language is used.

## File Structure Documentation

When creating a file structure, add a README to each directory describing the purpose of subdirectories—unless the purpose is painfully obvious.
