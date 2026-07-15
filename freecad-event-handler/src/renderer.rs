/// Frame renderer with proper border inset to prevent clipping artifacts
/// 
/// Key fix: Border must be inset by half pen width to keep antialiased stroke inside texture bounds
#[derive(Debug, Clone)]
pub struct Renderer {
    pen_width: f64,
    corner_radius: f64,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            pen_width: 2.0,
            corner_radius: 5.0,
        }
    }
    
    /// Generate render commands for annotation frame
    /// Returns list of drawing commands with properly inset borders
    pub fn render_frame(&self, annotation: &crate::annotation::Annotation) -> Vec<String> {
        let mut commands = Vec::new();
        
        // CRITICAL FIX: Inset border by half pen width to prevent clipping
        // Without this, the antialiased stroke gets clipped at texture edges
        let inset = self.pen_width / 2.0;
        
        let left = annotation.base_x + inset;
        let right = annotation.base_x + annotation.width - inset;
        let top = annotation.base_y + inset;
        let bottom = annotation.base_y + annotation.height - inset;
        
        // Ensure corner radius stays inside texture bounds
        let effective_radius = self.corner_radius.min((right - left) / 2.0).min((bottom - top) / 2.0);
        
        commands.push(format!("FRAME_START {} {}", annotation.id, annotation.is_dragging));
        commands.push(format!("RECT {:.2} {:.2} {:.2} {:.2} RADIUS {:.2}", 
            left, top, right, bottom, effective_radius));
        commands.push(format!("PEN_WIDTH {:.2}", self.pen_width));
        commands.push(format!("FRAME_END"));
        
        commands
    }
    
    /// Get the required inset value for a given pen width
    pub fn get_required_inset(pen_width: f64) -> f64 {
        pen_width / 2.0
    }
    
    /// Calculate effective corner radius that fits within bounds
    pub fn calculate_effective_radius(corner_radius: f64, width: f64, height: f64) -> f64 {
        corner_radius.min(width / 2.0).min(height / 2.0)
    }
}
