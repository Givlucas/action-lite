# Action Lite Demo

This directory contains example actions to demonstrate the functionality of Action Lite.

## Try It Out

Here are some commands to explore the example actions:

### List all actions
```bash
action-lite --path examples/actions list
```

### View the dependency graph
```bash
action-lite --path examples/actions graph
```

Expected output showing the dependency tree:
```
● implement-authentication [implementation] [auth-system] [PRIORITY]
  → design-database-schema [published] [auth-system]
  → setup-api-framework [design] [auth-system]

● write-documentation [discovery] [docs-project] [PRIORITY]
  → implement-authentication [implementation] [auth-system] (see above)

● build-user-dashboard [test] [dashboard-project]
  → implement-authentication [implementation] [auth-system] (see above)

● setup-api-framework [design] [auth-system] (see above)

● design-database-schema [published] [auth-system] (see above)
```

### List priority actions
```bash
action-lite --path examples/actions priority
```

Shows only actions tagged with `#priority`:
- implement-authentication
- write-documentation

### Filter by status
```bash
action-lite --path examples/actions status discovery
action-lite --path examples/actions status design
action-lite --path examples/actions status implementation
action-lite --path examples/actions status test
action-lite --path examples/actions status published
```

### Create a new action
```bash
action-lite --path examples/actions new "setup-testing" --project test-suite
```

This creates a new action file with the proper template structure.

### Move an action
```bash
action-lite --path examples/actions move setup-testing.md infrastructure/setup-testing.md
```

This moves the action and updates all references in other files.

## Example Actions Included

1. **implement-authentication** (#implementation #priority)
   - Demonstrates an in-progress action with dependencies
   - Marked as priority
   - Has detailed design section

2. **design-database-schema** (#published)
   - Shows a completed action
   - No dependencies (root action)

3. **setup-api-framework** (#design)
   - Currently in design phase
   - Dependency of authentication action

4. **write-documentation** (#discovery #priority)
   - Early stage action
   - Depends on authentication being complete

5. **build-user-dashboard** (#test)
   - In testing phase
   - Has a meta-graph directory with sub-actions
   - Demonstrates nested action structure

## Understanding the Output

### Status Colors
- **discovery** - Blue (early planning)
- **design** - Cyan (designing solution)
- **implementation** - Yellow (actively coding)
- **test** - Magenta (testing phase)
- **document** - Green (writing docs)
- **publish** - Bright Green (deploying)
- **published** - Bright Blue (completed)

### Graph Indicators
- `●` - Root action (no internal dependencies)
- `→` - Child action or dependency
- `(see above)` - Action already shown earlier in tree

### Priority Marker
- `[PRIORITY]` in red - Action marked as high priority
