# TDD Integration Guide for Action-Lite

This guide explains how Test-Driven Development (TDD) principles integrate with the action-lite methodology, bridging traditional TDD concepts with specification-driven verification.

## Overview

Action-lite incorporates testing as a mandatory phase (Phase 4) in its workflow, ensuring that "testing is guaranteed, not skipped." While traditional TDD and action-lite approach quality assurance differently, they share core principles: systematic testing, fast feedback, and iterative improvement.

This guide helps you:
- Understand how TDD concepts map to action-lite
- Apply TDD principles within the action-lite workflow
- Write testable specifications
- Think about testing across all phases

## Traditional TDD vs Action-Lite Testing

### Traditional TDD (Red-Green-Refactor)

Traditional Test-Driven Development follows this cycle:

1. **Red**: Write a failing test first
2. **Green**: Write minimal code to make the test pass
3. **Refactor**: Improve the code while keeping tests green

**Key characteristics:**
- Test-first approach (tests drive implementation)
- Design emerges from tests
- Rapid iteration with immediate feedback
- Focus on unit-level testing
- Tests define expected behavior

### Action-Lite Testing

Action-lite follows a different but complementary approach:

1. **Discovery**: Define what needs to be built (specifications)
2. **Design**: Create explicit design before implementation
3. **Implementation**: Build according to design and specifications
4. **Test**: Verify all specifications are met
5. **Document**: Capture learnings and impact
6. **Publish**: Deploy or release

**Key characteristics:**
- Design-first approach (specifications drive design, design drives implementation)
- Explicit design phase (cannot be skipped)
- Testing validates against specifications
- Comprehensive verification at specification level
- Cycle-back mechanism on failure

### Why Action-Lite Differs

Action-lite enforces design as a required phase, preventing the "throw code at the wall" approach. Instead of tests driving implementation directly, specifications define requirements during Discovery, design is created explicitly, and testing validates the complete implementation against all specifications.

**Key differences:**
- **Entry point**: TDD starts with tests, action-lite starts with specifications
- **Design**: TDD design emerges, action-lite design is explicit and required
- **Testing scope**: TDD emphasizes unit tests, action-lite emphasizes specification-level verification
- **Failure handling**: TDD refactors code, action-lite cycles back to Design or Discovery

**Similarities:**
- Both prevent skipping testing
- Both provide fast feedback
- Both support iterative improvement
- Both ensure quality through systematic verification
- Both make requirements explicit (tests vs specifications)

## TDD Principles That Apply to Action-Lite

While the workflows differ, core TDD principles fully apply to action-lite:

### 1. Tests as Specifications

In TDD, tests define expected behavior. In action-lite, **Statement of Specifications** serves the same purpose.

**TDD**: Each test describes a required behavior
**Action-lite**: Each specification item describes a requirement that must be met

Both make expectations explicit and testable.

### 2. Write Tests Before Implementation

In TDD, you write tests first. In action-lite, you write specifications first.

**TDD workflow**: Test → Implementation
**Action-lite workflow**: Specification → Design → Implementation → Verification

Both establish "what success looks like" before building.

### 3. Fast Feedback

TDD provides immediate feedback when tests run. Action-lite provides feedback through the mandatory Test phase.

**TDD**: Run tests frequently during development
**Action-lite**: Verification Agent provides comprehensive feedback after implementation

Both identify issues quickly, preventing waste.

### 4. Refactoring

TDD's refactor step improves code quality. Action-lite's cycle-back mechanism serves the same purpose.

**TDD**: Refactor while keeping tests green
**Action-lite**: Cycle back to Design (for implementation improvements) or Discovery (for specification issues)

Both support iterative improvement without breaking requirements.

### 5. Test Coverage Thinking

TDD emphasizes covering all code paths. Action-lite emphasizes covering all specifications.

**TDD**: Think about edge cases, error conditions, different inputs
**Action-lite**: Think about all requirements, technical constraints, quality attributes

Both ensure comprehensive verification.

### 6. Testability

TDD encourages testable code design. Action-lite encourages testable specifications.

**TDD**: Design code to be easily tested
**Action-lite**: Design systems with verification in mind, write specifications that are verifiable

Both make quality verification practical.

## How to Write Testable Specifications

The quality of your testing depends on the quality of your specifications. Testable specifications are concrete, measurable, and unambiguous.

### Characteristics of Good Specifications

**Concrete**: Specific and detailed, not vague or abstract
- Bad: "The system should be fast"
- Good: "API responses should complete within 200ms for 95% of requests under normal load"

**Measurable**: Observable and verifiable
- Bad: "The UI should be user-friendly"
- Good: "All interactive elements must be keyboard accessible and include ARIA labels"

**Unambiguous**: Clear single interpretation
- Bad: "Handle errors appropriately"
- Good: "On authentication failure, return HTTP 401 with error code and message in JSON format"

**Verifiable**: Can be tested objectively
- Bad: "The code should be maintainable"
- Good: "All public functions must include JSDoc comments describing parameters and return values"

**Independent**: Not dependent on implementation details
- Bad: "Use a Redux store with these specific reducers"
- Good: "Maintain user session state across page refreshes"

**Complete**: Includes all necessary constraints
- Bad: "Accept user input"
- Good: "Accept username (3-20 alphanumeric characters) and password (min 8 characters, require uppercase, lowercase, number)"

### Specification Categories

Organize specifications into clear categories:

**Functional Requirements**: What the system does
- Features and capabilities
- User interactions
- Business logic
- Data processing

**Technical Requirements**: How it's built
- Languages and frameworks
- Architecture patterns
- APIs and integrations
- Data storage

**Performance Requirements**: How well it performs
- Response times
- Throughput
- Resource usage
- Scalability limits

**Quality Requirements**: Non-functional attributes
- Accessibility (WCAG compliance)
- Security (authentication, authorization)
- Reliability (error handling, recovery)
- Maintainability (code standards, documentation)

**Business Constraints**: External limitations
- Compliance requirements
- Budget or resource limits
- Timeline constraints
- Compatibility requirements

### Examples of Testable Specifications

**Example 1: API Endpoint**

Bad specification:
- "Create a REST API for user management"

Good specifications:
- "Implement POST /api/users endpoint accepting JSON with email and password fields"
- "Validate email format (RFC 5322) and return 400 if invalid"
- "Hash passwords using bcrypt with cost factor 12 before storage"
- "Return 201 with user ID and creation timestamp on success"
- "Return 409 if email already exists in database"
- "Include rate limiting: maximum 10 requests per minute per IP"

**Example 2: UI Component**

Bad specification:
- "Build a data table component"

Good specifications:
- "Display data in rows with sortable column headers"
- "Support ascending/descending sort on all columns"
- "Highlight sorted column with visual indicator"
- "Support keyboard navigation (arrow keys move focus, Enter activates sort)"
- "Include ARIA attributes for screen reader accessibility"
- "Handle datasets up to 1000 rows without pagination"
- "Render initial view within 100ms on average hardware"

**Example 3: Configuration Change**

Bad specification:
- "Update the database settings"

Good specifications:
- "Increase PostgreSQL connection pool from 10 to 25 connections"
- "Set connection timeout to 5000ms"
- "Enable SSL for all database connections"
- "Verify settings by successfully connecting with 20 concurrent requests"
- "Document configuration changes in database-config.md"

## TDD Workflows Within Action-Lite

Testing isn't just Phase 4—think about testing throughout all phases.

### Discovery Phase: What Needs Testing?

During Discovery, identify what must be tested:

**Ask these questions:**
- What behaviors must work correctly?
- What edge cases exist?
- What could go wrong?
- What constraints must be enforced?
- What quality attributes matter (performance, security, accessibility)?

**Outputs:**
- Statement of Specifications with testable requirements
- Identification of edge cases and error conditions
- Performance or quality benchmarks

**Example:**
For a login feature, Discovery identifies:
- Valid credentials must grant access
- Invalid credentials must be rejected
- Account lockout after 5 failed attempts
- Session timeout after 30 minutes
- Password minimum requirements
- Accessibility for screen readers

### Design Phase: How Will We Test?

During Design, specify how verification will occur:

**Ask these questions:**
- How will each specification be verified?
- What test data is needed?
- What tools or frameworks will help?
- What makes this design testable?
- Are there integration points to verify?

**Outputs:**
- Statement of Design includes verification approach
- Test data or scenarios identified
- Testability considerations in design

**Example:**
For the login feature, Design specifies:
- Verification will use manual testing with browser
- Test with valid user account (user@example.com)
- Test with invalid credentials
- Test rapid failed attempts to verify lockout
- Verify session token in browser developer tools
- Check accessibility with screen reader

### Implementation Phase: Build for Testability

During Implementation, build with testing in mind:

**Ask these questions:**
- Am I following the design that was created for testability?
- Are error cases handled as specified?
- Is the code structured for verification?
- Do outputs match specification requirements?

**Outputs:**
- Implementation that meets specifications
- Consideration for how Verification Agent will test
- Clear pass/fail criteria

**Example:**
For the login feature, Implementation:
- Returns correct HTTP status codes (200 success, 401 invalid, 429 locked)
- Includes error messages matching specifications
- Implements session token as designed
- Handles edge cases (empty fields, SQL injection attempts)
- Follows accessibility specifications (ARIA labels, keyboard support)

### Test Phase: Execute Verification

During the Test phase, verify each specification systematically:

**Process:**
1. Read all specifications
2. Plan verification approach for each
3. Execute verification
4. Document results with evidence
5. Report pass/fail for each specification
6. Recommend next state

**The Verification Agent handles this phase** (see verification-agent.md)

**Example:**
For the login feature, Test phase verifies:
- ✓ PASS: Valid credentials (user@example.com / password123) grants access
- ✓ PASS: Invalid credentials returns 401 error
- ✓ PASS: Account locked after 5 failed attempts
- ✗ FAIL: Session timeout occurs at 45 minutes, not 30 as specified
- Recommendation: Cycle back to Implementation to fix timeout

### Document Phase: Record Test Insights

During Documentation, capture what was learned:

**Ask these questions:**
- What testing challenges were encountered?
- What edge cases were discovered during testing?
- How could future testing be improved?
- What verification approaches worked well?

**Outputs:**
- Analysis of Impact includes testing insights
- Documentation of test results
- Lessons for future similar work

### Continuous Testing Mindset

**Think about testing in every phase:**

| Phase | Testing Focus |
|-------|--------------|
| Discovery | What must be tested? (specifications) |
| Design | How will we test? (verification approach) |
| Implementation | Build for testability (following design) |
| Test | Verify everything (comprehensive verification) |
| Document | Capture insights (what we learned) |
| Publish | Final validation (works in production) |

This is similar to TDD's constant test awareness, adapted to action-lite's structured phases.

## Integration with Verification Agent

The Verification Agent implements the Test phase using TDD principles.

### Verification Agent's Role

The Verification Agent:
- Acts as independent inspector (like running a test suite)
- Reads Statement of Specifications (the test requirements)
- Verifies each specification systematically
- Reports pass/fail with evidence
- Recommends next state (document or cycle back)

**This is specification-driven verification**, analogous to TDD's test-driven approach.

### How Verification Agent Applies TDD Principles

**Systematic verification**: Like running a complete test suite
- Each specification is verified independently
- Nothing is assumed to work without evidence
- All requirements must pass

**Clear pass/fail criteria**: Like test assertions
- Each specification either passes or fails
- No ambiguity in results
- Evidence required for all claims

**Fast feedback**: Like test results
- Comprehensive report after implementation
- Identifies failures clearly
- Enables quick iteration

**Cycle-back on failure**: Like refactoring in TDD
- Implementation failures → cycle back to Implementation or Design
- Specification issues → cycle back to Discovery
- Enables iterative improvement

### Verification Reports as TDD Artifacts

The Verification Agent produces reports similar to test output:

**Report contents:**
- List of all specifications
- Pass/fail status for each
- Evidence and reasoning
- Overall assessment
- Recommended next state

**Example report:**
```markdown
## Verification Report: Login Feature

### Specification Results

1. POST /api/login endpoint accepts email and password ✓ PASS
   - Evidence: Tested with curl, endpoint responds correctly

2. Returns 401 for invalid credentials ✓ PASS
   - Evidence: Tested with wrong password, received 401 with error message

3. Locks account after 5 failed attempts ✓ PASS
   - Evidence: Made 5 failed attempts, 6th attempt returned 429

4. Session timeout at 30 minutes ✗ FAIL
   - Evidence: Token still valid after 30 minutes, timeout not implemented
   - Issue: Implementation missing timeout logic

### Overall Status: PARTIAL

### Recommendation
Cycle back to #implementation to fix session timeout.
All other specifications pass.
```

## TDD Terminology Mapping

If you're familiar with TDD, here's how concepts map to action-lite:

| TDD Term | Action-Lite Equivalent | Explanation |
|----------|----------------------|-------------|
| Red (failing test) | Specification not met (FAIL) | Requirement exists but isn't satisfied |
| Green (passing test) | Specification met (PASS) | Requirement is satisfied with evidence |
| Refactor | Cycle back to Design | Improve implementation or design |
| Test | Specification item | A testable requirement |
| Test suite | Statement of Specifications | Complete set of requirements |
| Test coverage | All specifications verified | Every requirement has been checked |
| Test-first | Specification-first | Define requirements before building |
| Assertion | Pass/fail criteria | How to determine if requirement is met |
| Test automation | Verification tooling | Tools that check specifications |
| Test runner | Verification Agent | Executes verification process |
| Test report | Verification report | Documents pass/fail for each requirement |
| Unit test | Specification verification | Verify individual requirement |
| Integration test | System specification verification | Verify cross-component requirements |
| Acceptance criteria | Statement of Specifications | What must be true for success |
| Mock/Stub | Test dependency | Isolated component for verification |

## Common Patterns and Anti-Patterns

### Patterns (Do These)

**Write specifications as acceptance criteria**
- Each specification should be verifiable
- Clear pass/fail criteria
- Independent of implementation details
- Example: "Return JSON with user ID" not "Use JSON.stringify()"

**Design with testability in mind**
- Include verification approach in Statement of Design
- Consider how each specification will be verified
- Identify test data needs during Design
- Example: Design includes "verify with test user account user@test.com"

**Verify each specification independently**
- Don't assume passing one means others pass
- Test each requirement separately
- Provide evidence for each
- Example: Test authentication separately from authorization

**Use cycle-back mechanism for quality**
- Don't compromise specifications to pass testing
- Cycle back to fix issues properly
- Design issues → return to #design
- Specification issues → return to #discovery
- Example: If performance spec fails, cycle back to redesign approach

**Document verification approaches in design**
- Statement of Design includes how to test
- Identifies tools or methods for verification
- Makes testing phase smoother
- Example: "Verify accessibility using NVDA screen reader"

**Think about testing in every phase**
- Discovery: What needs testing?
- Design: How will we test?
- Implementation: Am I building testably?
- Test: Verify everything
- Example: Constant awareness of verification

### Anti-Patterns (Avoid These)

**Vague specifications**
- Problem: Can't be verified objectively
- Bad: "System should handle errors well"
- Good: "On network error, display error message and retry button"

**Skipping test phase**
- Problem: Violates "testing is guaranteed"
- Action-lite prevents this—never skip #test state
- Always verify before documenting

**Not cycling back on failures**
- Problem: Accepting partial implementation
- If any specification fails, must cycle back
- Don't compromise requirements to pass testing

**Over-engineering tests**
- Problem: Testing implementation details instead of specifications
- Test specifications, not internal implementation
- Bad: "Uses Redux reducer named 'userReducer'"
- Good: "Maintains user state across page refreshes"

**Testing implementation details**
- Problem: Tests break when implementation changes even if behavior is correct
- Focus on specifications (what), not implementation (how)
- Bad: "Function calls validateEmail() helper"
- Good: "Rejects invalid email formats"

**Assuming specifications without evidence**
- Problem: Claiming pass without verification
- Every PASS must include evidence
- Bad: "Probably works correctly"
- Good: "Tested with 10 different inputs, all passed"

**Writing untestable specifications**
- Problem: Can't verify if requirement is met
- Bad: "Code should be maintainable"
- Good: "All functions include JSDoc comments with parameter descriptions"

## Practical Examples

### Example 1: API Endpoint Development

**Action File: Implement User Registration Endpoint**

```markdown
#action #test #user-management

# Notes
Need to create user registration endpoint for the application.
Must handle validation, password security, and error cases.

# Statement of Action
Implement POST /api/register endpoint to create new user accounts.
This enables users to sign up for the application and is required
before we can build the login feature.

# Statement of Inputs
- [Database schema design](./User%20Management/Database%20schema%20design.md)
- [Security requirements](./User%20Management/Security%20requirements.md)

# Statement of Specifications
- Accept POST requests to /api/register
- Require JSON body with email, password, and displayName fields
- Validate email format (must match RFC 5322)
- Reject passwords shorter than 8 characters
- Require password contains: uppercase, lowercase, number, special character
- Hash passwords using bcrypt with cost factor 12
- Store user record with email, hashedPassword, displayName, createdAt
- Return 201 status with user ID and createdAt timestamp on success
- Return 400 with validation errors if input invalid
- Return 409 if email already exists
- Include rate limiting: max 5 registration attempts per hour per IP
- All endpoints include CORS headers for web.example.com origin

# Statement of Design

## Output: User Registration API Endpoint

#### Design
Create Express.js route handler at POST /api/register.

**Validation approach:**
- Use validator library for email validation
- Check password length and character requirements
- Return specific error messages for each validation failure

**Security approach:**
- Hash passwords with bcrypt, cost factor 12
- Never log passwords or include in error messages
- Check for existing email before attempting to create

**Database approach:**
- Use PostgreSQL users table
- Email field has unique constraint
- Handle unique constraint violation as 409 error

**Verification approach:**
- Test with curl or Postman
- Test valid registration (test@example.com)
- Test invalid email formats
- Test weak passwords
- Test duplicate email
- Test rate limiting with rapid requests
- Verify password is hashed in database (not plaintext)

## Output: Rate Limiting Middleware

#### Design
Use express-rate-limit middleware configured for registration endpoint.
5 requests per hour per IP address.
Return 429 Too Many Requests when exceeded.

**Verification approach:**
- Make 5 requests in quick succession
- Verify 6th request returns 429

# Analysis of Impact
[To be filled after implementation and verification]
```

**TDD perspective:**
- Statement of Specifications = test suite (12 tests)
- Each specification = one test case
- Statement of Design = testability considerations
- Verification approach specified in design

### Example 2: UI Component

**Action File: Build Sortable Data Table Component**

```markdown
#action #test #component-library

# Notes
Need reusable data table component for displaying user lists,
transaction history, and other tabular data.

# Statement of Action
Create a React data table component with sorting capability.
This will be used across multiple pages in the application
for consistent data display.

# Statement of Inputs
- [Component library structure](./Components/Component%20library%20structure.md)
- [Accessibility standards](./Components/Accessibility%20standards.md)

# Statement of Specifications
- Accept data as array of objects via props
- Accept column definitions specifying field, label, and sortable flag
- Render table with header row and data rows
- Display column labels in header
- Show sort indicator (up/down arrow) on sorted column
- Click column header to sort ascending
- Click again to toggle descending
- Third click returns to unsorted state
- Support keyboard navigation: Tab moves between headers, Enter activates sort
- Include ARIA labels for sort state (aria-sort attribute)
- Include role="table", role="row", role="columnheader", role="cell"
- Support datasets up to 500 rows without performance degradation
- Render initial view within 100ms on average hardware

# Statement of Design

## Output: React DataTable Component

#### Design
Create functional React component using hooks for sort state.

**Component structure:**
- Props: data, columns
- State: sortColumn, sortDirection
- Render table with map over data
- Sort function using Array.sort()

**Sorting approach:**
- Track current sort column and direction in state
- Apply sort to data before rendering
- Toggle direction on repeated clicks
- Support string, number, date sorting

**Accessibility approach:**
- Semantic HTML table elements
- ARIA attributes for screen readers
- Keyboard event handlers on column headers
- Visible focus indicators

**Verification approach:**
- Render with sample data (10 rows, 3 columns)
- Click column headers and verify sort order
- Test keyboard navigation with Tab and Enter
- Test with screen reader (NVDA)
- Verify ARIA attributes in browser dev tools
- Test with 500 rows, measure render time with performance.now()

# Analysis of Impact
[To be filled after implementation and verification]
```

**TDD perspective:**
- 13 specifications = 13 test cases
- Design includes verification approach
- Accessibility and performance are testable specifications
- Clear pass/fail criteria for each requirement

### Example 3: Documentation

**Action File: Create API Documentation**

```markdown
#action #test #documentation

# Notes
Developers need documentation to understand how to use our API.
Currently no documentation exists.

# Statement of Action
Write comprehensive API documentation covering all endpoints,
authentication, and error handling. This enables external
developers to integrate with our API.

# Statement of Inputs
- [Completed API implementation](./API/Completed%20API%20implementation.md)
- [Authentication system](./API/Authentication%20system.md)

# Statement of Specifications
- Document all 8 REST endpoints with method, path, description
- Include request format (headers, body) for each endpoint
- Include response format (status codes, body) for each endpoint
- Provide example request and response for each endpoint
- Document authentication using Bearer token in Authorization header
- Document error response format (status, errorCode, message)
- List all possible error codes with meanings
- Include getting started guide for new developers
- Include rate limiting information (requests per hour limits)
- Store documentation in docs/api.md in the repository
- Use markdown format for version control
- Include table of contents with links

# Statement of Design

## Output: API Documentation File

#### Design
Create docs/api.md structured in sections:
1. Getting Started
2. Authentication
3. Endpoints (one section per endpoint)
4. Error Handling
5. Rate Limiting

**Format approach:**
- Markdown with code blocks for examples
- Consistent structure for each endpoint
- Real working examples that can be copy-pasted

**Verification approach:**
- Read complete documentation
- Verify all 8 endpoints documented
- Test each example with curl (verify examples work)
- Check all error codes listed
- Verify markdown renders correctly on GitHub
- Have another developer review for clarity

# Analysis of Impact
[To be filled after implementation and verification]
```

**TDD perspective:**
- Documentation has testable specifications
- Examples must actually work (verifiable)
- Completeness can be verified (all endpoints documented)
- Verification includes peer review

## When TDD Concepts Don't Directly Apply

Some action-lite work differs from traditional code development where TDD originated:

**Infrastructure and configuration changes:**
- TDD focuses on code, action-lite handles infrastructure too
- Verification may be deployment testing or smoke testing
- Specifications still apply (what must be true after change)

**Documentation work:**
- TDD doesn't traditionally cover documentation
- Action-lite treats documentation as first-class output
- Verification is review, completeness check, example testing

**Exploratory work:**
- Early-stage prototypes or investigations
- May use action-lite with looser specifications
- Testing may be "did we learn what we needed?"

**Design-only actions:**
- Some actions produce designs, not implementations
- Verification checks design quality, completeness, feasibility
- Different from code testing but still systematic

For all these cases, the core TDD principles still help:
- Define success criteria (specifications)
- Systematic verification
- Fast feedback
- Iterative improvement

## Quick Reference

### Action-Lite Testing Checklist

**Discovery Phase:**
- [ ] Identified all functional requirements
- [ ] Identified technical constraints
- [ ] Identified quality attributes (performance, accessibility, security)
- [ ] Identified edge cases and error conditions
- [ ] Wrote specifications that are testable (concrete, measurable, unambiguous)

**Design Phase:**
- [ ] Specified how each specification will be verified
- [ ] Identified test data or scenarios needed
- [ ] Designed for testability
- [ ] Documented verification approach in Statement of Design

**Implementation Phase:**
- [ ] Followed design to ensure testability
- [ ] Implemented all specifications
- [ ] Considered how Verification Agent will test
- [ ] Built in error handling as specified

**Test Phase:**
- [ ] Every specification verified independently
- [ ] Evidence collected for each verification
- [ ] Pass/fail determined objectively
- [ ] Issues identified clearly
- [ ] Cycle-back decision made if any failures

**Document Phase:**
- [ ] Recorded verification results
- [ ] Captured testing insights
- [ ] Noted lessons for future work

### TDD Terminology Quick Reference

- **Red** = Specification not met (FAIL status)
- **Green** = Specification met (PASS status)
- **Refactor** = Cycle back to Design or Discovery
- **Test** = Specification item
- **Test suite** = Statement of Specifications
- **Test coverage** = All specifications verified
- **Test-first** = Specification-first
- **Acceptance criteria** = Statement of Specifications

### Key Principles

1. **Specifications are test requirements** - Write them testably
2. **Design-first, not test-first** - But same goal: define success before building
3. **Testing is Phase 4** - Think about it in all phases
4. **Verification Agent** - Independent inspector, like running tests
5. **Cycle-back on failure** - Fix issues properly, don't compromise
6. **Evidence required** - Every PASS needs proof

---

For more on the action-lite methodology, see [SKILL.md](../SKILL.md).

For implementation of the test phase, see [verification-agent.md](../../../agents/verification-agent.md).
