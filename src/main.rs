use raylib::prelude::*;
use std::time;

use std::sync::Arc;

pub mod simulation;
use simulation::simulator::simulator::{SimArea, CrowdSim};

fn main() {
    let mut simualted_area_1 = SimArea::new();
    
    simualted_area_1.add_wall((0.0,0.0), (20.0,0.0));
    simualted_area_1.add_wall((0.0,8.0), (20.0,8.0));
    simualted_area_1.add_start_end_group(
        vec![(0.0,1.0), (0.0,3.0), (0.0,5.0), (0.0,7.0)],
        vec![(20.0,1.0), (20.0,3.0), (20.0,5.0), (20.0,7.0)]
    );
    
    let mut crowd_simulation = CrowdSim::new(Arc::new(simualted_area_1), 10.0);
    
    crowd_simulation.add_pedestrian(0, 0, 0, 1.35);
    
    let (mut rl, thread) = raylib::init()
        .size(1200, 600)
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
        
        let mut rl_handle = rl.begin_drawing(&thread);
        
        rl_handle.clear_background(Color::WHITE);
        rl_handle.draw_text("Pedestrian Behaviour Simulator", 12, 12, 20, Color::BLACK);
        rl_handle.draw_text(&format!("Frame count: {}", frame_count), 12, 36, 20, Color::BLACK);
        rl_handle.draw_text(&format!("Frame time: {}ms", curr_time.duration_since(prev_time).as_millis()), 12, 60, 20, Color::BLACK);
        
        //rl_handle.draw_ellipse(100 + (2*frame_count as i32)%200,200,30.0,20.0,Color::from_hex("FF00FF").unwrap());
        
        crowd_simulation.simulate_timestep(15.0/1000.0);
        crowd_simulation.draw(&mut rl_handle, (100,100));
        
        frame_count += 1;
    }
    
    // */
    
}

