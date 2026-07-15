# GUI Event Handler Specification

## Overview

This document specifies the behavior of a GUI event handler for interactive annotation objects. The handler manages mouse events (PRESS, DRAG, RELEASE) and maintains proper state transitions.

## Annotation Object Model

### Structure

Each annotation has:
- `id`: Unique identifier (u32)
- `base_x`, `base_y`: Base position (transparent proxy origin)
- `text_x`, `text_y`: Text content position
- `width`, `height`: Dimensions (default 100x50)
- `is_dragging`: Drag state flag
- `pick_offset_x`, `pick_offset_y`: Offset from base to text

### Transparent Proxy Hit-Testing

The annotation uses a transparent proxy pattern for hit-testing:

- **Proxy bounds**: Full annotation rectangle from (base_x, base_y) to (base_x + width, base_y + height)
- **Text bounds**: Small area around text position only

**Critical**: Use proxy bounds for hit-testing, not text bounds. Using text bounds allows leader line geometry to steal picks, especially at annotation corners.

## Event Handler State Machine

### States

```
Idle → Pressed → Dragging → Idle
       ↓
     Released → Idle
```

### DragState Variants

1. **Idle**: No active interaction
2. **Pressed**: Mouse down, waiting to determine click vs drag
   - Fields: press_time, x, y
3. **Dragging**: Active drag in progress
   - Fields: base_x, base_y, current_x, current_y

### Click Candidate Tracking

Click candidates track potential double-clicks:
- `time`: Timestamp of click
- `x`, `y`: Click position
- `annotation_id`: Target annotation (if any)

**Double-click window**: 250 time units

## Event Processing Rules

### PRESS Event

When processing PRESS(id, x, y):

1. Check if (x, y) hits any annotation's **proxy bounds** (not text bounds)
2. If hit:
   - Set state to Pressed
   - Store press_time, x, y
   - Set as click candidate
   - Return true (dragger event blocks selection)
3. If no hit:
   - Clear click candidate (this is box selection)
   - Return false

### DRAG Event

When processing DRAG(id, x, y):

1. If state is Pressed:
   - Calculate distance from press position
   - If distance > 5.0: transition to Dragging
   - Initialize pick offsets
2. If state is Dragging:
   - Update current position
   - Update annotation text position using pick offsets
3. If state is Idle: ignore

Distance formula: sqrt((x2-x1)² + (y2-y1)²)

### RELEASE Event

When processing RELEASE(id):

1. If state is Pressed (was a click):
   - Check for double-click:
     - If time_diff < 250 AND same annotation_id: trigger DOUBLE_CLICK
   - Clear click candidate
2. If state is Dragging:
   - Finalize annotation position (call end_drag)
   - **Clear click candidate** (critical for preventing confusion)
3. Reset state to Idle
4. Clear active_annotation_id

## Frame Rendering

### Border Inset Rule

Frame borders must be inset by half the pen width:

```
inset = pen_width / 2.0
left = base_x + inset
right = base_x + width - inset
top = base_y + inset
bottom = base_y + height - inset
```

**Why**: Without inset, antialiased stroke extends beyond texture bounds and gets clipped.

### Corner Radius Constraint

Effective corner radius must fit within bounds:

```
effective_radius = min(corner_radius, (right-left)/2, (bottom-top)/2)
```

## Priority Rules

1. **Dragger events take precedence over box selection**
   - When pressing on annotation proxy, block selection
2. **State cleanup after drag**
   - Clear click candidate after completed drags
   - Prevents box selection from confusing subsequent clicks
3. **Timing window enforcement**
   - Double-click only valid within 250 time units

## Common Failure Modes

### Failure Mode 1: Text-only Hit Testing

Using text bounds instead of proxy bounds causes:
- Lower-left corner drags fail (hit leader line instead)
- Erroneous box selection when intending to drag

### Failure Mode 2: State Not Cleared

Not clearing click candidate after drag causes:
- Subsequent clicks detected as double-clicks incorrectly
- Confusion between box select and annotation edit modes

### Failure Mode 3: No Border Inset

Rendering without border inset causes:
- Clipped antialiased stroke at texture edges
- Visual artifacts on frame corners

## Command Interface

### Input Format

```
PRESS <id> <x> <y>
DRAG <id> <x> <y>
RELEASE <id>
STATUS
ANNOTATIONS
RENDER
```

### Output Format

```
STATUS: state=<state>, candidate=<candidate>, active_ann=<id>
ANNOTATION <id>: (<base_x>, <base_y>) -> (<text_x>, <text_y>)
FRAME_START <id> <is_dragging>
RECT <left> <top> <right> <bottom> RADIUS <radius>
PEN_WIDTH <width>
FRAME_END
```
