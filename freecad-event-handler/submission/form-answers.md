# Submission form answers — freecad-event-handler (Rust)

Prepared answers for the Terminus-2 submission form. Text fields are written in Filip's voice.

---

## SHORT VERSIONS (1-2 sentences each, recommended)

**Difficulty Explanation**
The task requires understanding multiple interacting components: transparent proxy hit-testing, drag state machine with proper transitions, click candidate timing windows, and frame rendering with border inset. Each rule is stated once in the spec, so you have to read carefully. The difficulty comes from debugging why corner drags fail or why double-clicks trigger incorrectly after box selection.

**Solution Explanation**
I implemented the Annotation struct with proxy bounds for hit-testing instead of text-only bounds. The EventHandler tracks DragState through Pressed and Dragging transitions, clearing click candidates after completed drags. The Renderer insets borders by half pen width to prevent clipping artifacts.

**Verification Explanation**
Tests run the compiled binary fresh with specific event sequences and verify observable behavior: annotation positions change after drags, state returns to idle, click candidates clear, and frame rects have proper inset values. No hardcoded outputs - all values computed from the spec rules.

---

## Difficulty Explanation
(Describe why your task is challenging for humans and agents to solve.)

This task simulates a real GUI bug fix from FreeCAD PR #30900. The challenge comes from multiple interacting components that must work together correctly.

First, transparent proxy hit-testing is counter-intuitive. You need to use full annotation bounds for picking, not just the visible text area. Without this, leader line geometry steals picks at annotation corners, especially the lower-left corner. This is stated once in the spec among other details.

Second, the drag state machine has subtle requirements. State must transition from Idle to Pressed to Dragging and back to Idle. The pick offset calculation during drag initialization is easy to get wrong. Most importantly, click candidate state must be cleared after completed drags - without this, subsequent clicks get confused as double-clicks after box selection operations.

Third, frame rendering requires border inset by half pen width. This keeps antialiased strokes inside texture bounds. It is a small detail but critical for visual correctness.

The spec document contains all rules, but each is stated only once. You have to read carefully and implement all pieces correctly. Partial understanding leads to specific failure modes that the tests catch.

---

## Solution Explanation
(Describe your high-level approach and key insights.)

I started by reading the FreeCAD PR #30900 to understand the original bugs: annotation drag failures at corners, leader line pick stealing, and frame clipping artifacts.

The core insight is the transparent proxy pattern. Instead of testing hits against visible content only, I use full annotation bounds as an invisible pickable area. This ensures corner drags work reliably.

For state management, I implemented a DragState enum with Idle, Pressed, and Dragging variants. The key is tracking pick offsets during drag initialization and applying them consistently during drag updates. After release, state resets completely including clearing click candidates.

The renderer applies a simple but critical fix: inset all borders by half the pen width before generating drawing commands. This prevents antialiasing from extending beyond texture edges.

All code is in Rust with no external dependencies. The binary reads events from stdin and writes results to stdout, making it easy to test with piped input.

---

## Verification Explanation
(Explain how your tests verify correctness.)

Tests are behavior-based, checking observable outputs rather than implementation details. They use subprocess to run the compiled binary with specific event sequences.

Key test categories:
- Basic execution: STATUS command produces state information
- Hit-testing: Press on proxy bounds registers hits, press outside clears candidates
- Drag state: Position updates after drag, state returns to idle, candidates clear
- Frame rendering: RECT commands include proper inset values (left=101, right=199 for 100-wide annotation with pen_width=2)
- Edge cases: Corner hits succeed where text-only would fail
- Determinism: Same input always produces same output

Tests parse actual output using regex patterns, not exact string matching. This allows formatting variations while verifying correct behavior. All expected values derive from spec rules, not hardcoded answers.

The oracle solution demonstrates the correct implementation and passes all tests. Tests cannot be bypassed by hardcoding because they verify computed values like position changes and inset calculations.

---

## Prompt Check (checkbox)
CHECK the box. The instruction is one short prompt describing the goal without revealing implementation details. It points to the spec document but does not give away the rules or answers.

## Generate your Rubric(s) (checkbox)
CHECK "Click here to generate rubric(s)". Let the system generate it during CI.

## Send to reviewer? (checkbox)
LEAVE UNCHECKED on the first submission. Run CI first to verify difficulty and quality.

## Does this task use an approved canonical base image?
YES. Builder uses rust:1.85-slim with digest pin. Runtime uses debian:bookworm-slim with digest pin from the canonical set.

## Did you use a Task Inspiration from the Task Gallery?
YES. Inspired by FreeCAD PR #30900 which fixed annotation label drag handling and frame rendering bugs. The benchmark reproduces the reasoning challenge behind the original bug fix.

## How long did it take you to complete this submission?
[Put your real build time in minutes]

---

## Comments for Reviewer (optional)
This is a HARD difficulty task inspired by a real FreeCAD bug fix. The challenge comes from multiple subtle requirements: proxy hit-testing, state machine transitions, click candidate cleanup, and border inset for rendering. Each rule is stated once in the spec. Tests verify behavior through observable outputs, not implementation details. The oracle solution demonstrates correct implementation. Thanks for the review.
