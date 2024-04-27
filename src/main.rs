use raylib::prelude::*;
use std::time;

pub mod simulation;
//use crate::simulation::simulator;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .resizable() // If the window is not resizable it will float by default
        .title("Hello, World")
        .vsync() // Ensure that the window has vsync enabled (unless overridden by system)
        .msaa_4x()
        .build();
    
    let mut frame_count: u64 = 0;
    let mut curr_time = time::Instant::now();
    
    while !rl.window_should_close() {
        let prev_time = curr_time;
        curr_time = time::Instant::now();
        
        let mut handle = rl.begin_drawing(&thread);
        
        handle.clear_background(Color::WHITE);
        handle.draw_text("Pedestrian Behaviour Simulator", 12, 12, 20, Color::BLACK);
        handle.draw_text(&format!("Frame count: {}", frame_count), 12, 36, 20, Color::BLACK);
        handle.draw_text(&format!("Frame time: {}ms", curr_time.duration_since(prev_time).as_millis()), 12, 60, 20, Color::BLACK);
        
        handle.draw_ellipse(100 + (2*frame_count as i32)%200,200,30.0,20.0,Color::from_hex("FF00FF").unwrap());
        
        frame_count += 1;
    }
    
}

