---
owners: ["Lucas Givens"]
state: published
priority: false
continuous: false
---

# Notes
- Instead of tags for metadata frontmatter header

# Statement of Action
Create an action-lite system which is compatible with the limitations of the file system format.

# Statement of Inputs

# Statement of Specifications
- Must be file based.

# Statement of Design

 Action lite is an agile file based task tracking system that uses acyclic directed metagraphs to track task requirements, state, strategy, and dependency. The goal of action is to directly link task management to architecture design. Action items are documents which declare the expected outcome of a system and how to get there in plain English, in terms of direct actionable tasks. By declaring this information, architecture requirements and designs are immediately available as tasks which can be assigned. No need to translate large architecture documents into individual PBIs, duplicating information. It should be possible to take an action tree, start the project from scratch (clean room), and arrive at a similar implementation.

Action lite uses a directory "actions" to store all of its tasks which will be referred to as "actions". Each action is stored in a markdown file and has a frontmatter header for storing metadata. Action items create a dependency graph by declaring what actions they rely on explicitly as inputs, and directly state their expected outcome will be.

Actions items can have their own metagraphs, which track the tasks needed to break down a complex action. This is represented by a directory sharing the same name as the file stored at the same directory level. All actions belonging to the metagraph are stored here. Actions at the same graph level can use each other as "Inputs" creating a dependency structure. However, actions cannot consume actions from different graph levels as inputs.

Actions should be directly actionable like "develop a user page" or "Remediate Bug in A". Actions should not be vague. "Bug in A" or "User page" are not proper names for actions.  If an action would be something that takes place in action workflow it doesn't need its own action. For example "Design User page" or "Gather User page requirements" do not need their own actions because they already take place in the action system.

Action documents have the following sections
1. Notes - research on the task, task specific, things that anyone interacting with the document should know, specific tasks learnings or how it failed in previous attempts. Notes and learnings about tools, personal scratch pad, intersting links and refrences
2. Statement of Action - The task to be performed, more in-depth than the title, may include why the action is needed. Gives background and context to the action.
3. Statement of Specifications - List of requirements the action must meet to be considered completed.
4. Statement of Inputs - a markdown bullet list of .md links to other actions noting dependency relationship
5. Statement of Design - A detailed design for how to the action will be completed

sections are denoted by a markdown "#", with a space and the name of the section following.

``` markdown
# Notes
# Statement of action
# Statement of inputs
# Statement of Specifications
# Statement of Design
```

Action documents have an associated state which tracks what phase of the action workflow they are in. Below are the possible states for an action.
1. discovery - all actions start here
	- outputs: Title, Notes, Statement of action, statement of inputs, statement of specifications
2. design 
	- outputs: Statement of design
3. implementation - Follow design, or build out metagraph
4. test - evaluate that system meets specs. go back to design if test fails.
5. document - produce any needed user documentation for the system ensure code is properly commented. Does not need to be stored in action.
6. published - Available for use. (stable state)

# States
Action states are designed to bake in convergent and divergent thinking at different stages. This ensures that individuals completing actions are given time to research and discover possible solutions before design & implementation. Think of the action workflow as a mix between the waterfall process and scrum. Each metagraph is its own backlog, and each action item is its own mini waterfall.

If at any state or point in development and issue arises users are encouraged to return to any previous stage. However, users may not skip foward

## Discovery - Divergent thinking
The discovery stage is vitally important and serves as the foundation to the rest of the states. Improperly completed discovery may cause a failure in design and implementation. In this state users are encouraged to learn about the problem space. Do external and internal projec research and take notes on what a solution to the stated action would look like, try new tools, test small POCs.

This state generates: notes, Statement of action, Statement of Inputs, and Statement of specifications. It is not required that one user be entirely responsible for all outputs of this stage. One user could create the action item and the statement of action but leave the statement of specifications blank. Another user could continue the action and fill out the statement of specifications.

It is generally expected that the user finalizing the discovery state will be the user creating the design and the user who preformed the most relevent research.

Actions can move into this state at any point, even if there has been little planning done. This is the initial state of all actions

## Design - Convergent thinking
The design state is the next most important stage of action. Here the user plans out how the action will be performed. Generally an action design only cares about details 1-2 levels conceptually below itself. This prevents overburdening the user with excessive planning. For example, suppose we have an action to "create a 'car' class". This design for this action might plan out the member types and function signatures for the class, and produce a UML diagram but will not plan the implementation of these functions. Sub actions and meta graph should not be created in this step. Design can be broken down into metagraphs if still too high level in the next step. 

Generally actions should:
- State the behavior and functions that are required to complete the action correctly.
- Leave irrelevant details up to the completers' choice (decisions that have no impact on the outcome of the action)
- Are fairly verbose
- Should not contain any code beyond perhaps interfaces or basic type signatures, or generic pseudo code for particularly complex algorithms. Outside of this no code should be written in this stage.

The format of a design is the user's choice. It could be UML documents, graphs, charts, etc. However, it is recommended that plain language should accompany and explain any extra artifacts.

Actions should only move into this state once all of its inputs are in the "published" state

It is the user/teams choice on what a "completed" design is

## Implementation - Convergent thinking
In this stage two things can happen. One, The user determines that the task is too generic, and must be broken down. From here the use can create a metagraph to decompose the action into sub actions. This can happen as man times as needed until sufficent detail is planned. The ammount of detail required is up to the team to determine. Two, the user determines that the action has enough detail and implementation can begin.

The user completing this state does not need to be the user who did the design or discovery state. Here the user will read the design and complete it per specification to the best of their ability. If issues arise it is expected that the user will contact  the author of the action and return to the design or even discover state if necessary. This iterative nature allows for changes based on empirical observation and is encouraged to ensure that the action is properly completed.

Actions cannot move into this state unless the Design + Discovery state has been completed.

Actions cannot move from this state until all sub actions are completed

## Test - Divergent thinking
Actions cannot move into this state unless implementation has been completed

The test state involves:
- Stress testing the output
- Evaluating that the output meets specifications and action statement outlined in earlier sections
- Verifying that solution has proper unit test coverage

Think of it as a discovery state against the output itself.

If this state fails then the user can return to any of the previous states and re-start the cycle.

This state does not produce any documents other than possible learnings which can be added to the "notes" section

## Document - Convergent
Although action handles architecture documentation, it does not handle other forms. Like user or developer documentation. This stage is designed to give the user space to document how to use their output. However architecture documentation should not be duplicated outside of action.

## Published (end state)
This is the end state of an action. Once an action has been completed and its outputs have been made available for use, then it can move to this state.

The definition of "made available for use" varies per action and domain but is vitally important. An action cannot be considered complete if its outputs are not usable for other actions to consume as inputs.

Some actions are "continuous" and are never meant to be completed. Generally they align with behaviors or functions that need to be manually repeated at the owner's discretion.

# action metadata
Action markdown files start with a frontmatter header that track the following information.
- the owners of an action (list of strings)
- the current state of the action
- priority yes/no
- continuous yes/no

# Example action workflow

Note at any point in the workflow the user can return to a previous stage
```
action = f(x) = Discovery -> Design | revert -> implementation | f(x)^n | revert -> test | revert -> Document | revert -> publish

where "revert" is = to returning to any previous step
```
#### Start
1. Check state of all actions not in published state
2. Check for "priority" actions
3. Determine which actions can be worked on next.

for each ready action check the current state and determine which of the following to continue from.

#### Discovery
1. Initial problem  scoping - what is the action we want to preform (Create if not already defined, or understand current action).
2. Initial specs - What we already know we want as a requirements for the out come of the action. Does this action require any inputs?
3. Research problem space - Other solutions, tools, libraries, algorithms, general internal and external project research
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
