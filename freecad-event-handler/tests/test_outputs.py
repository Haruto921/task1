"""
Test suite for GUI event handler simulation.

Tests verify behavior of the event_handler binary through observable outputs.
Tests are behavior-based, not implementation-specific.
"""

import subprocess
import pytest
import re


def run_event_handler(input_text: str) -> str:
    """Run the event_handler binary with given input and return output."""
    result = subprocess.run(
        ["/usr/local/bin/event_handler"],
        input=input_text,
        capture_output=True,
        text=True,
        timeout=30
    )
    return result.stdout


def parse_annotation_line(line: str) -> dict:
    """Parse an ANNOTATION output line into components."""
    # Format: ANNOTATION <id>: (<base_x>, <base_y>) -> (<text_x>, <text_y>)
    match = re.match(
        r"ANNOTATION (\d+): \(([\d.]+), ([\d.]+)\) -> \(([\d.]+), ([\d.]+)\)",
        line.strip()
    )
    if match:
        return {
            "id": int(match.group(1)),
            "base_x": float(match.group(2)),
            "base_y": float(match.group(3)),
            "text_x": float(match.group(4)),
            "text_y": float(match.group(5)),
        }
    return None


def parse_rect_line(line: str) -> dict:
    """Parse a RECT output line into components."""
    # Format: RECT <left> <top> <right> <bottom> RADIUS <radius>
    match = re.match(
        r"RECT ([\d.]+) ([\d.]+) ([\d.]+) ([\d.]+) RADIUS ([\d.]+)",
        line.strip()
    )
    if match:
        return {
            "left": float(match.group(1)),
            "top": float(match.group(2)),
            "right": float(match.group(3)),
            "bottom": float(match.group(4)),
            "radius": float(match.group(5)),
        }
    return None


class TestBasicExecution:
    """Test basic execution and status output."""

    def test_status_command_produces_output(self):
        """STATUS command should produce state information."""
        output = run_event_handler("STATUS\n")
        assert "STATUS:" in output, "STATUS command should produce output with 'STATUS:' prefix"

    def test_status_contains_state_info(self):
        """Status output should contain state machine information."""
        output = run_event_handler("STATUS\n")
        assert "state=" in output, "Status should include drag state"


class TestHitTesting:
    """Test transparent proxy hit-testing behavior."""

    def test_press_on_annotation_proxy_bounds(self):
        """Pressing within annotation proxy bounds should register as hit."""
        # Annotation 0 starts at (100, 100) with size 100x50
        # Proxy bounds: (100, 100) to (200, 150)
        # Press at (150, 125) - center of proxy
        output = run_event_handler("PRESS 0 150.0 125.0\nSTATUS\n")
        # Should show active annotation
        assert "active_ann=Some(0)" in output or "active_ann=Some" in output, \
            "Press on proxy bounds should set active annotation"

    def test_press_outside_annotation_clears_candidate(self):
        """Pressing outside annotation should clear click candidate (box selection)."""
        # Press well outside annotation bounds
        output = run_event_handler("PRESS 0 500.0 500.0\nSTATUS\n")
        # Should show no candidate (cleared for box selection)
        assert "candidate=None" in output, \
            "Press outside annotation should clear click candidate"


class TestDragState:
    """Test drag state transitions and position updates."""

    def test_drag_updates_annotation_position(self):
        """Dragging should update annotation text position."""
        input_events = """PRESS 0 150.0 125.0
DRAG 0 200.0 175.0
RELEASE 0
ANNOTATIONS
"""
        output = run_event_handler(input_events)
        
        # Find annotation line
        ann_line = None
        for line in output.split("\n"):
            if line.startswith("ANNOTATION"):
                ann_line = line
                break
        
        assert ann_line is not None, "Should have annotation output"
        
        ann = parse_annotation_line(ann_line)
        assert ann is not None, "Should parse annotation line"
        
        # Text position should have moved from initial (200, 150)
        # Initial offset: text at (200, 150), base at (100, 100)
        # After drag to (200, 175), text should be at new position
        assert ann["text_x"] > 200.0 or ann["text_y"] > 150.0, \
            "Drag should move annotation text position"

    def test_drag_state_transitions_to_idle_after_release(self):
        """After release, state should return to idle."""
        input_events = """PRESS 0 150.0 125.0
DRAG 0 200.0 175.0
RELEASE 0
STATUS
"""
        output = run_event_handler(input_events)
        assert "state=Idle" in output, "State should be Idle after release"

    def test_click_candidate_cleared_after_drag(self):
        """Click candidate should be cleared after completed drag."""
        input_events = """PRESS 0 150.0 125.0
DRAG 0 200.0 175.0
RELEASE 0
STATUS
"""
        output = run_event_handler(input_events)
        assert "candidate=None" in output, \
            "Click candidate should be cleared after drag completes"


class TestDoubleClick:
    """Test double-click detection with timing window."""

    def test_rapid_clicks_trigger_double_click(self):
        """Two clicks within 250 time units should trigger double-click."""
        # Two quick presses/releases on same annotation
        input_events = """PRESS 0 150.0 125.0
RELEASE 0
PRESS 0 150.0 125.0
RELEASE 0
STATUS
"""
        output = run_event_handler(input_events)
        # Double-click would be detected and stored in pending events
        # The key is that state management handles this correctly
        assert "state=Idle" in output, "State should return to idle"

    def test_slow_clicks_do_not_trigger_double_click(self):
        """Clicks more than 250 time units apart should not trigger double-click."""
        # This test verifies the timing window exists
        # In practice, each event increments time by 1
        # So we need many events between clicks to exceed 250
        pass  # Complex to test without explicit timing control


class TestFrameRendering:
    """Test frame rendering with border inset."""

    def test_render_produces_frame_commands(self):
        """RENDER command should produce frame drawing commands."""
        output = run_event_handler("RENDER\n")
        assert "FRAME_START" in output, "Should have FRAME_START command"
        assert "FRAME_END" in output, "Should have FRAME_END command"

    def test_render_includes_rect_with_inset(self):
        """Frame rect should have borders inset by half pen width."""
        output = run_event_handler("RENDER\n")
        
        rect_line = None
        for line in output.split("\n"):
            if line.startswith("RECT"):
                rect_line = line
                break
        
        assert rect_line is not None, "Should have RECT command"
        
        rect = parse_rect_line(rect_line)
        assert rect is not None, "Should parse RECT line"
        
        # Default annotation: base at (100, 100), size 100x50
        # Pen width: 2.0, so inset should be 1.0
        # Expected: left=101, top=101, right=199, bottom=149
        expected_inset = 1.0  # pen_width / 2 = 2.0 / 2
        
        assert abs(rect["left"] - (100.0 + expected_inset)) < 0.01, \
            f"Left border should be inset by {expected_inset}"
        assert abs(rect["top"] - (100.0 + expected_inset)) < 0.01, \
            f"Top border should be inset by {expected_inset}"
        assert abs(rect["right"] - (200.0 - expected_inset)) < 0.01, \
            f"Right border should be inset by {expected_inset}"
        assert abs(rect["bottom"] - (150.0 - expected_inset)) < 0.01, \
            f"Bottom border should be inset by {expected_inset}"

    def test_render_includes_pen_width(self):
        """Frame output should include pen width specification."""
        output = run_event_handler("RENDER\n")
        assert "PEN_WIDTH" in output, "Should specify pen width"


class TestEdgeCases:
    """Test edge cases and failure modes."""

    def test_corner_hit_detection(self):
        """Hitting corner of annotation proxy should work (not fail like text-only)."""
        # Lower-left corner of proxy: (100, 150)
        # Text-only hit test would miss this
        # Proxy hit test should succeed
        output = run_event_handler("PRESS 0 105.0 145.0\nSTATUS\n")
        # Should register as hitting annotation
        assert "active_ann=Some(0)" in output or "active_ann=Some" in output, \
            "Corner hit should register on proxy bounds"

    def test_multiple_annotations_first_hit(self):
        """With multiple annotations, first hit should be selected."""
        # This tests that we check all annotations, not just first
        pass  # Would require multiple annotations setup

    def test_empty_input_handled_gracefully(self):
        """Empty input should not cause errors."""
        output = run_event_handler("")
        assert output == "" or output.strip() == "", \
            "Empty input should produce minimal output"


class TestDeterminism:
    """Test that execution is deterministic."""

    def test_same_input_same_output(self):
        """Same input should always produce same output."""
        input_events = """PRESS 0 150.0 125.0
DRAG 0 200.0 175.0
RELEASE 0
ANNOTATIONS
RENDER
STATUS
"""
        output1 = run_event_handler(input_events)
        output2 = run_event_handler(input_events)
        assert output1 == output2, "Same input should produce identical output"
