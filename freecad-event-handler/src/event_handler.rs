/// Event handler with proper state management for GUI interactions
/// 
/// Key behaviors:
/// - Dragger events take precedence over box selection
/// - Click candidate timing window (250ms simulated)
/// - State cleared after completed drags to prevent double-click confusion
#[derive(Debug, Clone, PartialEq)]
pub enum DragState {
    Idle,
    Pressed { press_time: u64, x: f64, y: f64 },
    Dragging { base_x: f64, base_y: f64, current_x: f64, current_y: f64 },
}

#[derive(Debug, Clone)]
pub struct ClickCandidate {
    pub time: u64,
    pub x: f64,
    pub y: f64,
    pub annotation_id: Option<u32>,
}

pub struct EventHandler {
    drag_state: DragState,
    click_candidate: Option<ClickCandidate>,
    pending_events: Vec<String>,
    current_time: u64,
    active_annotation_id: Option<u32>,
}

impl EventHandler {
    pub fn new() -> Self {
        EventHandler {
            drag_state: DragState::Idle,
            click_candidate: None,
            pending_events: Vec::new(),
            current_time: 0,
            active_annotation_id: None,
        }
    }
    
    /// Get current status as string
    pub fn get_status(&self) -> String {
        format!("state={:?}, candidate={:?}, active_ann={:?}", 
            self.drag_state, 
            self.click_candidate.as_ref().map(|c| c.annotation_id),
            self.active_annotation_id)
    }
    
    /// Handle mouse press event
    /// Returns true if this is a dragger event (takes priority over selection)
    pub fn handle_press(&mut self, id: u32, x: f64, y: f64, annotations: &mut Vec<crate::annotation::Annotation>) -> bool {
        self.current_time += 1;
        
        // Check if pressing on an annotation's transparent proxy
        let mut hit_annotation = false;
        let mut hit_annotation_id = None;
        
        for ann in annotations.iter() {
            if ann.hit_test_proxy(x, y) {
                hit_annotation = true;
                hit_annotation_id = Some(ann.id);
                break;
            }
        }
        
        if hit_annotation {
            // This is a potential dragger event - takes priority
            self.drag_state = DragState::Pressed {
                press_time: self.current_time,
                x,
                y,
            };
            self.active_annotation_id = hit_annotation_id;
            
            // Set as click candidate for potential double-click detection
            self.click_candidate = Some(ClickCandidate {
                time: self.current_time,
                x,
                y,
                annotation_id: hit_annotation_id,
            });
            
            return true; // Dragger event blocks selection
        } else {
            // Not hitting annotation - this is a box selection event
            // Clear any existing click candidate to prevent confusion
            self.clear_click_candidate();
            return false;
        }
    }
    
    /// Handle mouse drag event
    pub fn handle_drag(&mut self, id: u32, x: f64, y: f64, annotations: &mut Vec<crate::annotation::Annotation>) {
        self.current_time += 1;
        
        match &self.drag_state {
            DragState::Pressed { press_time, x: start_x, y: start_y } => {
                // Check if moved enough to be considered a drag (not just a click)
                let dx = x - start_x;
                let dy = y - start_y;
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance > 5.0 {
                    // Transition to dragging state
                    self.drag_state = DragState::Dragging {
                        base_x: *start_x,
                        base_y: *start_y,
                        current_x: x,
                        current_y: y,
                    };
                    
                    // Update the active annotation
                    if let Some(ann_id) = self.active_annotation_id {
                        for ann in annotations.iter_mut() {
                            if ann.id == ann_id {
                                ann.start_drag(*start_x, *start_y);
                                break;
                            }
                        }
                    }
                }
            }
            DragState::Dragging { base_x, base_y, .. } => {
                // Continue dragging
                self.drag_state = DragState::Dragging {
                    base_x: *base_x,
                    base_y: *base_y,
                    current_x: x,
                    current_y: y,
                };
                
                // Update annotation position
                if let Some(ann_id) = self.active_annotation_id {
                    for ann in annotations.iter_mut() {
                        if ann.id == ann_id {
                            ann.update_drag(x, y);
                            break;
                        }
                    }
                }
            }
            DragState::Idle => {
                // Spurious drag without press - ignore
            }
        }
    }
    
    /// Handle mouse release event
    pub fn handle_release(&mut self, id: u32, annotations: &mut Vec<crate::annotation::Annotation>) {
        self.current_time += 1;
        
        match &self.drag_state {
            DragState::Pressed { .. } => {
                // Was a click, not a drag
                // Check if this could be a double-click
                if let Some(ref candidate) = self.click_candidate {
                    let time_diff = self.current_time - candidate.time;
                    if time_diff < 250 && candidate.annotation_id == self.active_annotation_id {
                        // Double-click detected
                        self.pending_events.push("DOUBLE_CLICK".to_string());
                    }
                }
                
                // Clear click candidate after processing
                self.clear_click_candidate();
            }
            DragState::Dragging { .. } => {
                // Completed a drag - update annotation final position
                if let Some(ann_id) = self.active_annotation_id {
                    for ann in annotations.iter_mut() {
                        if ann.id == ann_id {
                            ann.end_drag();
                            break;
                        }
                    }
                }
                
                // CRITICAL: Clear state after drag completes
                // This prevents box selection from confusing subsequent clicks
                self.clear_click_candidate();
            }
            DragState::Idle => {}
        }
        
        // Reset to idle state
        self.drag_state = DragState::Idle;
        self.active_annotation_id = None;
    }
    
    /// Clear click candidate state
    fn clear_click_candidate(&mut self) {
        self.click_candidate = None;
    }
    
    /// Get pending events (like double-click)
    pub fn get_pending_events(&self) -> &[String] {
        &self.pending_events
    }
}
