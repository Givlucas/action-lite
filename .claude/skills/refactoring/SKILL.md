---
name: refactoring
description: Code refactoring patterns and best practices. Use when refactoring existing code, planning large-scale changes, or when asked about safe ways to modify production systems. Includes the Dark Mode/Light Mode rollout pattern.
---

# Refactoring Patterns

## Core Principle

Refactor code before implementing new features. Always ask: "Can I make implementing this easier?"

## Clean As You Go

Clean up after yourself as you go along. Don't continually generate large messes to clean later.

Spill? Wipe it up. Smear? The same.

If it takes just two minutes and you aren't in a rush or need to focus—just do it.

## Dark Mode / Light Mode Rollout

For large-scale changes, deliver in these steps:

### 1. Dark Mode
- Call both old and new implementations together
- Compare the results
- Return results from the **old** implementation
- Exercises refactoring with production data without impacting behavior
- Can sample new implementation on % of requests if performance is a concern

### 2. Light Mode
- Call both old and new implementations
- Compare the results
- Return results from the **new** implementation
- Old implementation still present for easy revert if needed

### 3. Sunset
- Remove unused code and data
- Clean up the old implementation

Reference: https://understandlegacycode.com/blog/key-points-of-refactoring-at-scale/

## When Refactoring

- Before adding new features
- When you notice code duplication
- When the current design makes changes difficult
- When tests are hard to write (often a design smell)
