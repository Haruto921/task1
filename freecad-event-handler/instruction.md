Build a Rust CLI application that simulates GUI event handling for interactive annotation objects.

The program reads event commands from stdin and outputs state information. Events include PRESS, DRAG, and RELEASE with coordinates. The handler must properly manage drag state transitions and prioritize dragger events over box selection events.

Key requirements:

1. Implement transparent proxy hit-testing - the full annotation bounds should be pickable, not just the text area. This prevents leader line geometry from stealing picks.

2. Track drag state with proper initialization including base position, text position, pick offset, and plane information. Handle the transition from PRESS to DRAG to RELEASE correctly.

3. Clear click candidate state after completed drags. Without this, subsequent clicks get confused as double-clicks after a box selection operation.

4. For frame rendering, inset borders by half the pen width. This keeps the antialiased stroke inside texture bounds and prevents clipping artifacts at edges.

5. Dragger events take precedence over box selection. When pressing on an annotation's proxy bounds, block selection events.

6. Use a timing window of 250 time units for double-click detection. Events within this window on the same annotation trigger double-click behavior.

Source files go in /app/src/. Build with cargo and run the event_handler binary. Feed events via stdin and read results from stdout.

The spec document at /app/spec.md describes the expected behavior in detail. Read it carefully - each rule is stated once.
