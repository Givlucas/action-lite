---
name: software-development-philosophy
description: "Use when doing anything involving software, programming, coding, configuration. Examples, Create a <program>, Design a <program>, Write me a <progam>"
---

## Test Driven Development
Always write the functions signatures and types first, then write tests based on the behavior you want to see, then implement. Never implement without writing tests first. Only test what you need to don't go over bored just for testing sake, only test things you expect to happen EVERY time for a given input. Remember the pyramid of testing you should have: Lots of unit tests > some integration tests > even fewer end to end tests

## Software Development is Engineering
Computer science is an engineering discipline. A solution must solve its goal to be considered correct, even if the edges are somewhat rough. The preferred solution is one that is maintainable (easy to fix, expand, or rework), feasible (achievable with available resources), and cost-productive (worth the effort to implement and maintain). Riskier problems require higher degrees of quality and certainty; fly-by-wire software demands more rigor than a simple CRUD app.

## Design Strategically, Not Tactically
Code should be explicitly designed. You shouldn't throw stuff at a wall to see what sticks. Programming is strategic—you need to think about algorithms, architecture, and long-term implications. Coding is tactical. Be strategic in your approach: understand the problem, model the system, verify assumptions, then implement. Even open source projects are usually designed with deliberate PR control and direction.

## Code is a Liability
The more code you have, the more you have to manage. Less code is generally better as long as it remains readable. This prevents software entropy. Well-written code decreases context size, increases predictability, and makes code more "readable" by storing the context of what the code does in lexical token names rather than relying solely on documentation.

## Consistency Reduces Complexity
Consistency reduces cognitive load and allows you to make safer assumptions, enabling easier reasoning about higher-order concerns. Abstractions can create consistency. When systems behave predictably, developers spend less time deciphering patterns and more time solving actual problems.

## Verify Assumptions
It is dangerous to assume anything. Always verify core assumptions. When you assume, you make an ass out of u and me. Use guard statements in your software and question your work processes and worldviews. Instead of deep nesting, return early if an assumption fails—this prevents the pyramid of doom with lots of nested statements.

## Document the Why
Documentation hides complexity by providing an easy, human-readable way to convey information. The why and goal of a section of code should be captured in comments around that code. Comments should provide context, express boundaries, and explain reasons. Avoid comments like "this is a loop"—explain why the loop exists and what it's accomplishing. If something is not immediately obvious, you probably need to document it.

## Always Shippable
Everything should be functional once it enters main. This is the "no broken windows" philosophy from The Pragmatic Programmer, where non-functional code is seen as technical debt that shouldn't exist. This approach reduces integration problems and ensures the codebase is always in a releasable state. Hide incomplete features behind feature flags rather than having visible but non-functional UI elements.

## Review Code Like an Adversary
Attack and review code like it's your adversary. Code must be critically and fully reviewed. A good developer knows the codebase—where things should and shouldn't go. AI cannot magically make bad ideas work; bad ideas are bad ideas. Developers who don't understand the goal of the project can do immense damage without code reviews.

## Don't Make People Anxious
People appreciate security. People like things that make them comfortable and remember what pisses them off. When solving problems, provide an interim solution with directions for those who still need to work while you implement a fully featured fix. This leaves people feeling more comfortable and assured. Write documentation on what you are doing and what will be done so everyone is on the same page.

## Users Control Their Updates
Users should always be in control of their updates. When things update, they break and change. The user may not be ready or prepped to make changes to their system. An update should never be unexpected.

## Communication Must Be Clear
Being clear is essential to good communication. Don't leave details for people to figure out—it can be stressful. When we aren't clear, people scramble, don't deliver expected results, and panic and suffer. Use concise and specific wording; communication and reasoning about a topic can be very difficult if imprecise language is used.

## Plan Before You Build
You need to plan and learn things first—don't rush. Drafts exist for a reason. They give space to learn, iterate, and feel out the problem. This is what the discovery phase is about: time to draft, time to outline. Divergent thinking explores and expands ideas; convergent thinking brings them together. Both are necessary and can be scheduled deliberately.

## Parse Don't Validate
Instead of simply validating data repeatedly, extract it into a type structure with clearly defined rules in your code. This allows you to make more assumptions safely when writing code and saves you from dealing with stringy data. Code should be self-policing: if something shouldn't happen, the code should make it so it can't. Make invalid states impossible. This is sometimes called negative space programming—type systems can automatically enforce these practices, increasing code quality through communication and letting the computer do the work of checking.

## Model Entities by Lifecycle
Many business processes have entities that go through a series of milestones. In a particular state, changes are allowed to certain attributes but not others. Once a subset of properties are valid, the entity can transition to the next stage. Instead of viewing this as a single entity with a bunch of booleans or a CURRENT_STATE attribute (which implies a state machine unknown to consumers), view each state as a different thing—a different type. This makes state transitions explicit and prevents invalid operations at compile time.

## Services Only When State Changes
If a service call always results in the same result for the same parameters, you don't need a service—make a library and avoid the operational overhead. Services only make sense when they change something in the world. State and state changes are unavoidable concerns in a service-based architecture. Start by figuring out what a service does, then you can figure out what it needs to know.

## Data Dominates
If you've chosen the right data structure and organized things well, the algorithms will almost always be self-evident. Data structures, not algorithms, are central to programming. Write stupid code that uses smart objects. Fancy algorithms are slow when n is small, and n is usually small. Fancy algorithms have big constants and are buggier than simple ones. Until you know that n is frequently going to be big, don't get fancy.

## Composition Over Inheritance
Entity Component Systems are a break away from OOP-style project layouts, preferring composition to inheritance. Entities are just identifiers—unique tags that represent that an entity exists, not necessarily describing what it is or does. Components are data structures that hold data representing a particular aspect an entity has. Systems act on components. This pattern creates flexible, decoupled architectures where behavior emerges from combining simple pieces rather than deep inheritance hierarchies.

## Dependency Injection
Dependency injection is a dollar word for a twenty-five cent concept: classes don't construct their dependencies, they are passed in. This decouples components, makes testing easier, and clarifies what each piece of code actually needs to function. Frameworks exist to automate the wiring, but the principle is simple—don't hide dependencies inside constructors, make them explicit parameters.

## Refactor Before Implementing
Refactor code before implementing new features. Ask yourself: can I make implementing this easier? The best time to clean up is before you add complexity, not after. Complex code that is frequently changed creates hotspots—focus refactoring efforts there. If code is a mess but you never touch it, who cares? Focus on complex code that changes often.

## Technical Debt vs Cognitive Debt
Technical debt is bad code—code that doesn't follow sound engineering principles. Cognitive debt is when developers don't understand the system they're working on because things change too quickly. The system becomes untenable to manage. Both are symptoms of software entropy, but they require different solutions. Technical debt needs refactoring; cognitive debt needs documentation, stable pace, and sustainable practices. Good documentation addresses both, but cognitive debt is the more insidious problem—a codebase can be clean yet incomprehensible if knowledge isn't shared.

## WET Can Beat DRY
"Write Everything Twice" can be better than "Don't Repeat Yourself." DRY taken too far leads to premature abstraction, hidden coupling, and code that's harder to understand than the duplication it replaced. Two or three similar implementations are often clearer than one abstraction that tries to handle every case. Wait until you've written something three times before abstracting—by then you'll understand the actual pattern, not a guess at it.

## Premature Optimization is Evil
You cannot predict where a program will spend its time; bottlenecks occur in surprising places. Don't tune for speed until you've measured. Fancy algorithms are slow when n is small, and n is usually small. They have big constants and more bugs than simple ones. Use brute force until measurement proves otherwise. When in doubt, use simple algorithms and simple data structures. The right data structure makes algorithms self-evident—this is more important than algorithmic cleverness.

## Software is Not Art
Software can be art in principle, but professional software development is not. It stops being art when people pay for results. Art is messy, poorly organized, imperfect, and subjective. Professional software should be none of these things unless the art itself is the product. Professional software development is engineering—focused on delivering reliable, organized, well-structured results. The romantic notion of the programmer-artist leads to unmaintainable systems.

## Walking Skeleton
A walking skeleton is a minimal end-to-end implementation that gets fleshed out over time. You build the thinnest possible slice that touches every layer of the system—from UI to database—then iteratively add muscle. This approach validates architecture early, exposes integration problems before you've invested heavily, and provides a working system at every stage. It's pragmatic for exploring designs or when features genuinely need multiple iterations to complete. Avoid unless truly necessary.

## Liskov Substitution Principle
If S is a subtype of T, objects of type T should be replaceable with objects of type S without altering program correctness. Subtypes cannot violate the behavioral contract of their parent type—they must maintain the same preconditions, postconditions, and invariants. When inheritance breaks this principle, polymorphism becomes a source of bugs rather than flexibility. Prefer composition when substitutability isn't natural.

## Spec-driven Development
Specifications become the source of truth over reading every line of code. This approach acknowledges a practical reality: comprehensive code reading doesn't scale, especially with large AI-assisted codebases. Well-written specs act as a filter layer providing confidence about system behavior without exhaustive auditing. Still read tests carefully—they're concrete validation of the spec. Trust but verify: specs, descriptions, and actual code don't always match, and debt accumulates when specifications aren't backed by review.

## Divergent and Convergent Thinking
Designing and creating are part of the divergence and convergence mental processes that exist on the same spectrum. Divergent thinking is the discovery phase—branching out, expanding possibilities, generating options without judgment. Convergent thinking is the design phase—bringing ideas together, synthesizing, selecting the best approach. Both modes are necessary and can be scheduled deliberately. Don't try to converge while you're still discovering, and don't keep diverging when it's time to commit. Know which mode you're in.

## Code Review Checklist
When reviewing code, ask: Is this code necessary? Could a library replace it? Does it meet its goals? Is it easy to read? Does this function do one clear thing? Does it explicitly state its assumptions? Does it rely on hard-coded values? Does it abuse language or framework behavior? Does it use strong typing? Does it contain broken windows or stubbed logic? Are there security flaws? What breaks if I merge this? Was it tested? Does it need documentation beyond comments?

## Explore Before Committing
Explore all options before making a choice rather than following a path to its end to determine what works best. Examining multiple approaches upfront avoids the waste of pursuing a path only to discover later it's suboptimal. Understand the full landscape of options available before committing resources. This isn't analysis paralysis—it's informed decision-making. The time spent exploring is usually less than the time wasted backtracking from a bad choice.

## Make Invalid States Impossible
If something shouldn't happen in code, make it physically impossible—don't just recommend against it. If people shouldn't access certain functions on an object, prevent access entirely rather than relying on documentation or convention. This is negative space programming: defining what is NOT allowed rather than just what is. Type systems can automatically enforce these practices. When complete prevention isn't possible, put up explicit warnings. Murphy's Law applies—if something can go wrong, it will. Let the computer do the work of checking rather than relying on human discipline.

## Guard Statements and Early Returns
Instead of deep nesting, return early if an assumption fails. Check first, then early return if invalid—this prevents the pyramid of doom with lots of nested statements. Guard statements serve as built-in validation mechanisms that make code self-policing. In languages with weak type systems, you may need to manually ensure types using guards. Don't assume variables, types, or conditions are valid. Verify rather than assume, fail fast with clear control flow, and make implicit expectations explicit. This defensive approach prevents both logical errors and type-related bugs.

## Enforce Immutability at Construction
Properties that shouldn't change after creation should be immutable by design, not convention. Use language features like C#'s `init` keyword to enforce that properties cannot be modified outside of construction time. This is explicit, compile-time enforced immutability—rather than relying on discipline or runtime checks, declare properties as immutable from the outset. The intent becomes clear and the type system prevents accidental modifications. Immutability reduces bugs by eliminating entire categories of state-change errors.

## Testing as a Forcing Function
You must treat testing as mandatory, not optional—it is the forcing function that surfaces design flaws before production does. When testing is guaranteed in your process, it creates a feedback loop that cycles you back to discovery when something fails to meet specifications. Bad design does not survive production, and you should not want it to. It is dangerous to assume anything works correctly; always verify your core assumptions through deliberate testing. People will not remember that you shipped a feature a day early, but they will remember when that feature breaks. Quality verification is not overhead—it is the mechanism that enables real results and protects the work you have already invested.

## Fix Broken Windows
The "broken windows" philosophy originates from criminology research showing that visible signs of disorder invite more disorder. In software, if your codebase already contains bad naming conventions like `data`, `temp`, or `x`, commented-out blocks left "just in case," ignored compiler warnings, or inconsistent formatting, you signal to yourself and others that further negligence is acceptable. This creates a vicious cycle where each shortcut justifies the next, accelerating software entropy. You should treat warnings as errors and clean up messes as you encounter them, following the boy scout rule: always leave code better than you found it. 

## Pike's Rules: Measure, Then Optimize
You cannot predict where your program will spend its time. Bottlenecks appear in surprising places, so never add a speed hack until you've proven exactly where the problem lies. Measure first, and even then, only optimize if one part of the code truly overwhelms the rest. When in doubt, use brute force—fancy algorithms are slow when n is small, and n is usually small. Complex solutions carry big constants and more bugs than their simpler counterparts. Use simple algorithms paired with simple data structures until measurement proves you need otherwise. If you've chosen the right data structures, the algorithms will reveal themselves naturally.

## Type Systems as Enforcement
Use your type system as an enforcement mechanism, not just documentation. When you define types with precise constraints, you shift the burden of checking invariants from yourself to the compiler, turning manual verification into automated validation. This is a form of negative space programming: you define what states are valid, and the type system makes everything else impossible. By stating requirements about the shape and form of your data structures, you solve entire classes of bugs before your code ever runs. Runtime errors become compile-time errors, caught before they can reach production. Let the computer do the tedious work of checking your assumptions at every boundary, freeing you to focus on the actual logic of your program. Types are not just labels; they are guardrails that make your code self-policing.

## Declarative vs Imperative Thinking
When you describe what you want rather than how to get it, you're thinking declaratively—you specify the end state and let the system figure out the steps. When you need precise control over execution order or when the path matters as much as the destination, imperative thinking serves you better. Declarative approaches shine when the problem domain has well-understood solutions that can be optimized by the underlying system, like SQL queries, configuration languages, or build systems. Reach for imperative code when you're exploring unfamiliar territory, need fine-grained control over side effects, or when the sequence of operations carries meaning. Start by asking whether you care more about the journey or the destination—if you just need to arrive, declare where you're going and let the system drive.

## Prioritize Hotspots Over Complexity
You shouldn't prioritize refactoring based on complexity alone—instead, focus on the code that changes frequently and causes real friction. Use your git history to identify hotspots: the files you modify most often, especially when those files are also complex. If code is a mess but you never touch it, who cares? The real problem emerges when you're constantly modifying intricate code, creating bottlenecks that slow down future development. Before implementing new features, examine whether you can simplify the surrounding code first—make the task easier before you add to it. By targeting hotspots rather than broadly complex code, you ensure your refactoring efforts directly improve your development velocity.

## Parallel Implementations for Safe Refactoring
You can dramatically reduce refactoring risk by running your old and new implementations in parallel, validating each against production data before fully committing to the change. Start in "dark mode"—execute both implementations but return the old results, allowing you to test the refactoring thoroughly without impacting users. Once you're confident the new implementation is correct, switch to "light mode" by returning the new results while keeping the old implementation as an instant rollback option. This two-phase approach lets you catch subtle bugs and edge cases that only emerge under real-world conditions, which is impossible to discover in staging environments alone. Finally, sunset the old implementation once you've verified the new one is stable and reliable in production. By breaking refactoring into these empirically validated stages, you transform what could be a risky big-bang migration into a series of small, reversible decisions that protect your system and users.

## Concurrency Requires Language Awareness
You must understand that concurrency behavior is fundamentally language-specific—C# returns control to the caller while JavaScript returns to the event loop, and these differences shape how you design async systems. When you write async code, you should await as fast as possible to avoid accumulating synchronous work that blocks other operations. The real concurrency problems emerge when you let a single component juggle multiple async operations simultaneously, creating contention and unpredictability. You need language-aware design patterns because a non-blocking server like Kestrel can only free up threads when they actually yield control—if your code does too much work before awaiting, you defeat the entire purpose. Rather than hoping your async code will magically scale, you should design it with your language's concurrency model explicitly in mind, treating the event loop or thread pool as a shared resource that demands respect.

## Schedule Time for Discovery
You need to treat divergent thinking as a first-class part of your development process, not something that happens by accident. Backlog refinement is fundamentally a convergent flow—it narrows down possibilities into actionable items—but it requires a divergent thinking step beforehand to generate and align those possibilities. Sometimes this means scheduling specific time for discovery and ideation as its own work item, giving your team explicit permission to explore the problem space. Drafts and outlines exist to give you space to learn, iterate, and feel out the problem before committing to a solution. By protecting this discovery phase, you avoid the trap of rushing into implementation with incomplete understanding. The payoff is a backlog grounded in genuine insight rather than assumptions.

## Naming is Documentation
You communicate intent through the names you choose, and poor naming becomes technical debt that compounds over time. Variables named `data`, `temp`, or `x` without context are broken windows—small violations that signal deeper problems in your codebase and invite further decay. When you invest in good naming conventions, you're establishing consistency that scales across your entire system, making code self-explanatory and reducing the cognitive load on anyone reading it. Your naming choices are a form of documentation; they eliminate the need for clarifying comments by making purpose and context obvious. By treating naming as a discipline rather than an afterthought, you prevent ambiguity and create code that explains itself through clarity and structure.

## Interfaces Define Contracts
When you define an interface, you're creating a contract that specifies how components communicate without dictating how they work internally. This contract is why WSGI works so elegantly—nginx, gunicorn, and Flask can each focus on their own implementation while confidently talking to each other through a well-defined protocol. Similarly, the Model Context Protocol lets models interact with APIs in a defined way, freeing you from worrying about the implementation details of each tool. You can think of modules as composable units that enforce this contract principle: they expose a clean interface (their options) while keeping the messy implementation details hidden. This separation means you can swap implementations without breaking dependent code, and your codebase stays flexible as requirements change. By designing around contracts instead of implementation specifics, you make your system easier to test, extend, and reason about.

## Keep Documentation with Code
Documentation serves as a bridge between your code and those who read it, making knowledge discoverable right where it's needed. You should store all documentation—whether architectural overviews, user guides, or implementation details—alongside your source code so it evolves together and maintains a complete history. This co-location ensures that when you update code, the relevant documentation sits in the same version control commit, preventing your docs from becoming stale or divorced from reality. You should capture the reasoning behind your design decisions and the goals of your systems in comments and files within your repository, making it unnecessary for others to reverse-engineer your intent. However, you should avoid documenting other people's systems in detail; instead, focus on clear overviews or introductions that point readers to their authoritative sources.

## Structure Logs for Machines
You should structure your logs so that machines can parse them, not just humans read them. When you write logs as single lines or follow a consistent format, you make it easier for log aggregation tools and monitoring systems to process them automatically. You must respect logging levels—they exist to help you and your tools filter what matters at any given moment. By using your language's built-in logging libraries with standardized configuration, you ensure that logs across your codebase are consistent and machine-readable. Don't force people and tools to manually untangle your log output when a little structure upfront saves everyone time.

## Lock Dependencies for Reproducibility
Your lock files are the source of truth for reproducible builds, containing commit hashes that pin your dependencies to specific versions. When you need the latest changes, you update your lock file—a deliberate action that gives you control over when breaking changes enter your project. This approach balances reproducibility with the flexibility to innovate, ensuring your builds work consistently across machines and time. Your Nix flakes are strictly tied to Git, meaning unstaged files are invisible to the system, so you must explicitly commit your changes for them to be recognized. By locking your dependencies, you eliminate the uncertainty of "it works on my machine" and create builds that anyone can reproduce exactly as you intended.

## Review Early to Prevent Multiplication
You must build high-leverage human review into every stage of your development pipeline, because a bad line of planning can cascade into hundreds of bad lines of code. You should establish a structured workflow—from research through spec, design, and implementation—where human reviewers critically examine work at each gate before it multiplies downstream. You catch problems early when they're still isolated ideas rather than waiting until they've propagated through your codebase. You review plans and designs like an adversary, questioning assumptions and edge cases before implementation even begins. You understand that this upfront investment in rigorous review prevents exponential costs later, when fixing a flawed architecture means rewriting thousands of lines. You make human judgment the lever that prevents small mistakes from becoming catastrophic problems.

## Prefer Simple Over Clever
Fancy algorithms are buggier than simple ones, and they're much harder to implement—so you should reach for simple algorithms and simple data structures first. When you build with simple scripts, services, and configurations, you avoid unnecessary complexity that creates maintenance burden and introduces bugs. If you organize your systems declaratively and lay things out in a sane manner, making changes becomes straightforward and intuitive. You'll find that complex algorithms rarely justify their cognitive overhead; instead, they demand more careful implementation and vigilant debugging. By choosing clarity over cleverness, you make your codebase more accessible to your team and your future self, reducing the surface area for bugs and making the entire system more resilient.

## Explore Broadly, Learn Deeply
When facing a new project, resist the temptation to master every framework before you start. Instead, spend time exploring the landscape of available options, understanding their core features and trade-offs, then commit to learning one deeply that fits your needs. This breadth-first exploration gives you the strategic knowledge to make informed choices, while depth-first learning on your selected framework builds the expertise needed to use it effectively. By being selective rather than exhaustive, you avoid the trap of perpetual learning and start shipping real solutions faster. The key is knowing when to stop exploring and start diving deep—pick the tool that solves your immediate problem best, and become genuinely skilled with it.

## Classes Conflate Three Concepts
A traditional OOP class secretly bundles three distinct things: a record type (data fields), a module (functions operating on that data), and an interface (polymorphic behavior through virtual methods). This conflation is a foot gun because it obscures which concept you actually need. Inheritance from a class without virtual methods is just syntactic sugar for composition—saying "B inherits from A" really means "B has all the fields and methods of A," a has-a relationship masquerading as is-a. When you inherit from a class that defines both state and interface, you force every implementation to adopt that specific state structure, which may be incongruent with what the new implementation actually needs. Prefer composition over inheritance: instead of saying a circle is a shape, recognize that a circle has shape properties. By separating records, modules, and interfaces into distinct constructs, you avoid the hidden coupling that makes OOP hierarchies brittle and hard to extend.

## Libraries Over Services When Stateless
If a service call always results in the same result for the same parameters, you don't need a service—make a library and avoid the operational overhead. Ask yourself whether a third-party library could easily replace your custom implementation and make the code less confusing. Pure deterministic functions have no business running as services; they belong in libraries where they're easier to test, deploy, and reason about. Before you build a service, evaluate whether a library would solve the problem more elegantly and reduce operational complexity. Reserve services for situations where state management, external dependencies, or shared resources genuinely require a networked architecture.

## Prototypes Are for Learning
Prototypes are where you can afford to be experimental—get artsy, try unconventional approaches, and learn by doing. The whole point is to practice and iterate, to try things even if they might not work, because that's how you really understand a problem. But the moment you move from prototype to production, the rules change fundamentally. Real software needs to be treated like a living thing that requires care and maintenance, which means every line of code must be thoroughly reviewed. Don't skip the review process; it's not bureaucracy, it's how you ensure your software actually survives in the real world.

## Open Source Still Needs Governance
You might think open source means a free-for-all where anyone can push changes directly, but the reality is far different. Even the most community-driven projects have gatekeepers—maintainers who review pull requests and decide what gets merged. This isn't a limitation but a feature: you need someone with veto power to steer the project's direction and filter out ideas that don't serve the codebase's long-term health. In fact, open source projects often maintain *higher* quality standards than corporate software because there's no profit motive to rush features or cut corners. You can always fork the project if you disagree, but you can't force your changes upstream—and that's exactly why open source thrives.

## Define and Protect Scope
You must explicitly define the boundaries of what you are building before you start building it. Without clear scope, you will find yourself chasing an ever-expanding target, adding features that dilute your focus and delay delivery. Validate that your scope is achievable and that all stakeholders agree on what's included—ambiguous requirements are the quicksand of software development. Protect your defined scope fiercely; every feature request that arrives mid-project is a potential derailment disguised as opportunity. When new features demand your attention, treat them with the discipline of a gatekeeper: evaluate them against your scope, plan them deliberately for future iterations, and resist the temptation to collapse them into work already underway.

