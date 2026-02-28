#action #priority
#Design

# Notes
- Instead of tags for meta data frontmatter header

# Statement of action
Create a action-lite system which is compatible with the limitations of the file system format.
# Statement of inputs

# Statement of specifications
- Must be file based.

# Statement of design

Action lite is similar to the action design system in goal but reduced in scope to accommodate the restrictions of its medium. Action lite is a file based task tracking system that uses acyclic directed meta graphs to track task requirements. Action lite uses a directory "actions" to store all of its actions. Each action is stored in a mark down file and has the following tags \#action, \#(state). If an action has a meta-graph a directory sharing the same name as the file will be stored at the same level. All actions belonging to the meta-graph are stored here. A meta-graph directory can exist without a associated action. Actions at the same directory level can use each other as "Inputs" creating a dependancy structure.

The goal of action is to combine task management and architecture design. Actions are actionable achitecture design documents. Actions should not be vauge. "Bug in A" or "User page" are not proper names for actions. Actions should be actionable like "develop a user page" or "Remediate Bug in A". If a action would be something that takes place in action workflow it doesn't need its own action. For example "Design User page" or "Gather User page requirements" do not need their own actions because they already take place in the action system.


The structure and types of status are unchanged from normal action, with the action title as its file name.
1. Notes - general notes / research on the task
2. Statement of Action - The task to be preformed, more in depth then title, may include why the action is needed
3. Statement of Inputs - a markdown bullet list of .md links to other action files. or wiki links
4. Statement of Design - A detailed design for how to the action will be completed

sections are denoted by a markdown "#", with a space and the name of the section following.

# Notes
# Statement of action
# Statement of inputs
# Statement of Design

Below are the possible states for a action. Each tag should be lowercase.
1. discovery - all actions start here
	- outputs: Title, Notes, Statement of action, statement of inputs, statement of specifications
2. design 
	- outputs: Statement of design
3. implementation - Follow design
4. test - evaluate that system meets specs. go back to design if test fails.
5. document - produce any needed user documentation for the system ensure code is properly commented. Does not need to be stored in action.
6. published - Available for use. (stable state)

# Discovery
# Design
# Implementation
# Test
# Document
# Published (end state)


# Workflow
If at any point the developer determines that the current state has failed they can return to any previous state in the workflow. However, developers cannot skip steps. Action is a highly iterative process. For example, if you are in the implementation phase and realize the design is flawed. You can return to the design stage, or even the discovery stage. From here you must follow the workflow progression.

Think of action like breaking down the waterfall process into a bunch of mini-water falls. Each Action haveing its own cycle.

Action designs should only contain details 1-2 levels conceptually down. Its up to user descrition to determine if a action is too big. In general is best to think that actions shouldn't go all the way down to describing the "fuction" level, but defining the function signature, or a "class" or a system are all comfortable levels of detail for an action.

Actions should start very high level "Create a action-lite CLI tool" and from there get broken down into "inputs" and child actions that live in the metagraph for that action.

# Depndancy
The graph nature of action ensures designs are consistent. For example. Lets say a "Create HTTP serivce" action is determine to need to be re-worked later in development. Modifying the "Create HTTP serivce" makes all of Actions that rely on its outputs "dirty". This notifys the developer and if they wish, they can review all child actions.

# Statements

State tags maybe present at any point of the file but only one should occur per file.

Actions may also be tagged with \#priority to mark them as important. But the tag should be removed once completed.

Some actions have no end. These are marked \#continuous and are forever in the "implementation" stage
