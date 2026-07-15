/// Annotation object with transparent proxy for hit-testing
/// 
/// The base position represents the transparent proxy bounds (full pickable area)
/// The text position represents the visual content (should be marked unpickable)
#[derive(Debug, Clone)]
pub struct Annotation {
    pub id: u32,
    pub base_x: f64,
    pub base_y: f64,
    pub text_x: f64,
    pub text_y: f64,
    pub width: f64,
    pub height: f64,
    pub is_dragging: bool,
    pub pick_offset_x: f64,
    pub pick_offset_y: f64,
}

impl Annotation {
    pub fn new(id: u32, base_x: f64, base_y: f64, text_x: f64, text_y: f64) -> Self {
        Annotation {
            id,
            base_x,
            base_y,
            text_x,
            text_y,
            width: 100.0,
            height: 50.0,
            is_dragging: false,
            pick_offset_x: 0.0,
            pick_offset_y: 0.0,
        }
    }
    
    /// Check if a point hits the transparent proxy bounds (full annotation area)
    /// This is the key fix: use proxy bounds, not just text position
    pub fn hit_test_proxy(&self, x: f64, y: f64) -> bool {
        let left = self.base_x;
        let right = self.base_x + self.width;
        let top = self.base_y;
        let bottom = self.base_y + self.height;
        
        x >= left && x <= right && y >= top && y <= bottom
    }
    
    /// Check if a point hits only the text content (vulnerable to leader line stealing)
    pub fn hit_test_text_only(&self, x: f64, y: f64) -> bool {
        let margin = 10.0;
        let left = self.text_x - margin;
        let right = self.text_x + margin + 60.0;
        let top = self.text_y - margin;
        let bottom = self.text_y + margin + 20.0;
        
        x >= left && x <= right && y >= top && y <= bottom
    }
    
    /// Update text position during drag
    pub fn update_drag(&mut self, current_x: f64, current_y: f64) {
        self.text_x = self.base_x + self.pick_offset_x + (current_x - self.base_x);
        self.text_y = self.base_y + self.pick_offset_y + (current_y - self.base_y);
    }
    
    /// Start drag from a hit point
    pub fn start_drag(&mut self, hit_x: f64, hit_y: f64) {
        self.is_dragging = true;
        self.pick_offset_x = self.text_x - self.base_x;
        self.pick_offset_y = self.text_y - self.base_y;
    }
    
    /// End drag
    pub fn end_drag(&mut self) {
        self.is_dragging = false;
        // Update base to new position after drag completes
        self.base_x = self.text_x - self.pick_offset_x;
        self.base_y = self.text_y - self.pick_offset_y;
    }
}
