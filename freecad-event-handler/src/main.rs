mod event_handler;
mod annotation;
mod renderer;

use std::io::{self, BufRead};
use event_handler::EventHandler;
use annotation::Annotation;
use renderer::Renderer;

fn main() {
    let stdin = io::stdin();
    let mut handler = EventHandler::new();
    let mut annotations: Vec<Annotation> = Vec::new();
    
    // Initialize with default annotation for testing
    annotations.push(Annotation::new(0, 100.0, 100.0, 200.0, 150.0));
    
    let mut output_lines: Vec<String> = Vec::new();
    
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        
        match parts[0] {
            "PRESS" => {
                if parts.len() >= 4 {
                    let id: u32 = parts[1].parse().unwrap_or(0);
                    let x: f64 = parts[2].parse().unwrap_or(0.0);
                    let y: f64 = parts[3].parse().unwrap_or(0.0);
                    handler.handle_press(id, x, y, &mut annotations);
                }
            }
            "DRAG" => {
                if parts.len() >= 4 {
                    let id: u32 = parts[1].parse().unwrap_or(0);
                    let x: f64 = parts[2].parse().unwrap_or(0.0);
                    let y: f64 = parts[3].parse().unwrap_or(0.0);
                    handler.handle_drag(id, x, y, &mut annotations);
                }
            }
            "RELEASE" => {
                if parts.len() >= 2 {
                    let id: u32 = parts[1].parse().unwrap_or(0);
                    handler.handle_release(id, &mut annotations);
                }
            }
            "STATUS" => {
                let status = handler.get_status();
                output_lines.push(format!("STATUS: {}", status));
            }
            "ANNOTATIONS" => {
                for ann in &annotations {
                    output_lines.push(format!("ANNOTATION {}: ({:.1}, {:.1}) -> ({:.1}, {:.1})", 
                        ann.id, ann.base_x, ann.base_y, ann.text_x, ann.text_y));
                }
            }
            "RENDER" => {
                let renderer = Renderer::new();
                for ann in &annotations {
                    let commands = renderer.render_frame(ann);
                    for cmd in commands {
                        output_lines.push(cmd);
                    }
                }
            }
            _ => {}
        }
    }
    
    for line in output_lines {
        println!("{}", line);
    }
}
