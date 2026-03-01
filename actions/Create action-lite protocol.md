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

Action lite is similar to the action design system in goal but reduced in scope to accommodate the restrictions of its medium. Action lite is a file based task tracking system that uses acyclic directed meta graphs to track task requirements and state. Action lite uses a directory "actions" to store all of its actions. Each action is stored in a mark down file and has the following tags \#action, \#(state). If an action has a meta-graph a directory sharing the same name as the file will be stored at the same level. All actions belonging to the meta-graph are stored here. A meta-graph directory can exist without a associated action. Actions at the same directory level can use each other as "Inputs" creating a dependancy structure.

The goal of action is to directly link task management to architecture design. Actions items are actionable achitecture design documents. Actions should not be vauge. "Bug in A" or "User page" are not proper names for actions. Actions should be directly actionable like "develop a user page" or "Remediate Bug in A". If a action would be something that takes place in action workflow it doesn't need its own action. For example "Design User page" or "Gather User page requirements" do not need their own actions because they already take place in the action system.


The structure and types of status are unchanged from normal action, with the action title as its file name.
1. Notes - general notes / research on the task
2. Statement of Action - The task to be preformed, more in depth then title, may include why the action is needed. Gives background and context to the action.
3. Statement of Specifications - List of requirements the action must meet to be considered completed.
4. Statement of Inputs - a markdown bullet list of .md links to other actions noting dependancy relationship
5. Statement of Design - A detailed design for how to the action will be completed

sections are denoted by a markdown "#", with a space and the name of the section following.

# Notes
# Statement of action
# Statement of inputs
# Statement of Specifications
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

# States
Actions phases are designed to bake in Convergent and Divergent thinking at different stages. This Ensures that individuals completeing are given time to research and discover possible solutions before implementaton.

## Discovery - Divergent thinking
The discovery stage is vitally important and serves as the foundation to the rest of the phases. Improprly complete disovery may cause a failure in design and implemnetation. In this phase users are encouraged to learn about the problem space. Do research and take notes on what a solution to the stated action would look like. This phase generates: notes, Statement of action, Statement of Inputs, and Statement of specifications. It is not required that one user be entirely responsible for all outputs of this stage. One user could create the action item and the statement of acton but leave the statement of requirements blank. Another user could continue the action and fill out the requirements.

It is generally expected that the user finalizing the discovery phase will be the user creating the design.

## Design - Convergent thinking
The design state is the next most important stage of action. Here the user plans out how the action will be preformed. Generally an action design only cares about details 1-2 levels conceptually below itself. This prevents over burdneing the user with excessive planning. For example, suppose  we have an action to "create a 'car' class". This design for this action might plan out the member types and function signitures for the class, and produce a UML diagram but will not plan the implentation of these functions.

Depending on the side of the action, it maybe necessary to split up the design into smaller piecies. This is where the metagraph aspect of action comes into play. Evey action item can contain a metagraph. This metagraph is represnted by a directory of the same name as the action item. It can contain any number of sub action items. Steps detailed in the design phase can be broken up into sub actions. Each sub-action may have its own meta graph. It is best if actions start at a high-level then get broken down into smaller pieces.

Generally actions should:
- State the behavior and functions that are required to complete the action correctly.
- Leave irrelevent details up to the completeors choice (decisions that have no impact on the outcome of the action)
- Are fairly verbose
- Should not contain any code beyond perhaps interfaces or basic type signatures, or generic pseudo code for particulary complex algorthims. Out side of this no code should be written in this stage.

The format of a design is users choice. It could be UML documents, graphs, charts, etc. However is recommended that plan language should accompany and exmplain any extra artifacts. 

## Implementation - Converget thinking
This stage handles actually preforming the design laid out by the action. The user completing this phase does not need to be the user who did the design or discovery phase. Here the user will read the design and complete it per specification to the best of their ability. If Issues arise it is expected that the user will contact  the author of the action and return to the design or even discover phase if neccessary. This itterative nature allows for changes based on imperical observation and is encouraged to ensure that the action is properly completed.

Actions cannot move into this phase unless all input actions have been completely and the Design + Discovery phase has been completed.

## Test - Divergent thinking
The test phase involves:
- Stress testing the output
- Evaluating that the output meets specifications outlined in earlier sections

Think if it as a discovery phase against the output itself.

If this phase fails then the user can return to any of the previous phases and re-start the cycle.

## Document - Convergent
Although action handles architecture documentation, it does not handle other forms. Like user or developer documentation. This stage is designed to give the user space to document how to use their output

## Published (end state)
This is the end state of an actio. Once and action has been completed and its outputs have been made avaialable for use, then it can move to this phase.

The definition of "made avaiable for use" varies per action and domain but is vitally important. An action cannot be considered complete if its outputs are not usable for other actions to consume as inputs.

# Depndancy
The graph nature of action ensures designs are consistent. For example. Lets say a "Create HTTP serivce" action is determine to need to be re-worked later in development. Modifying the "Create HTTP serivce" makes all of Actions that rely on its outputs "dirty". This notifys the developer and if they wish, they can review all child actions.

# Format of an action file
Action files start with a frontmatter header that track the following information.
- the owners of an action (list of strings)
- the current state of the action
- priority yes/no
- continuous yes/no

# Statements
Actions may also be tagged with \#priority to mark them as important. But the tag should be removed once completed.

Some actions have no end. These are marked \#continuous and are forever in the "implementation" stage
