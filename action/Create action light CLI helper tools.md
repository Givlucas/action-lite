#action #action #priority #AI
#Discovery

# notes

## sonnet 4.5 action cli - post mortem Attempt 2
How it failed. Application worked and code appeared to be written well. However it did not meet specifications. I did a tree again instead of the proper behavior.

Also of note. it seemed to expand on action ALOT. It would write alot of code in the design and was a overly verbose in its testing.

### What could have been done better
- removed statement of impact, it didn't really need this. Seems extra for action anyway
- opus 4.5 at some stage?
- Used git frequently
- VM for fully autonomous agent self management 
- rust project guidelines skill
- nix project guideline skill
	- needs to know that its on a nix system
- reproducibility guidelines skill
- researcher agent
- Action orchestra-tor still asked me too many questions. Probably need to give it more guidance. More structured workflow. Was not automnus enough
- wouldn't use agents unless specifically asked
## what went right
- no crashes on the first run after the AI said it was finished

# Statement of inputs
- [[Create action-lite protocol]]
# Statement of specifications
- Built in rust
- contains a nix flake for development and packaging
- features
	- list all actions and their status
	- list priority items
	- list continuous items
	- list items by status
	- create a new empty action with all sections
	- add new inputs to an action
	- move actions and update all input references to them
	- change the status of an action
	- graph the actions in terminal as a flow chart specify relation ships between metagraphs and actions on the same level
	- colored output
	

# Statement of Design
## Output
### design
using Claude code to create CLI helper tools for working with the action lite framework ... and more to come