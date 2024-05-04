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
        vec![(20.0,1.0), (20.0,3.0), (20.0,5.0), (20.0,7.0), (5.0, 5.0)]
    );
    
    let mut crowd_simulation = CrowdSim::new(Arc::new(simualted_area_1), 10.0);
    
    crowd_simulation.add_pedestrian(0, 3, 4, 1.35);
    
    let (mut rl, thread) = raylib::init()
        .size(1200, 650)
        .resizable() // If the window is not resizable it will float by default
        .title("Hello, World")
        .vsync() // Ensure that the window has vsync enabled (unless overridden by system)
        .msaa_4x()
        .build();
    
    let mut frame_count: u64 = 0;
    let mut curr_time = time::Instant::now();
    
    while !rl.window_should_close() {
        // For calculating frametime
        let prev_time = curr_time;
        curr_time = time::Instant::now();
        let frame_time = curr_time.duration_since(prev_time);
        
        let mut rl_handle = rl.begin_drawing(&thread);
        
        rl_handle.clear_background(Color::WHITE);
        
        // Simulate one timestep & draw the simulation
        //crowd_simulation.simulate_timestep(0.015);
        crowd_simulation.simulate_timestep(frame_time.as_secs_f64());
        crowd_simulation.draw(&mut rl_handle, (100,150));
        
        // Debug text
        rl_handle.draw_text("Pedestrian Behaviour Simulator", 12, 12, 20, Color::BLACK);
        rl_handle.draw_text(&format!("Frame count: {}", frame_count), 12, 36, 20, Color::BLACK);
        rl_handle.draw_text(&format!("Frame time: {}ms", frame_time.as_millis()), 12, 60, 20, Color::BLACK);
        rl_handle.draw_text(&format!("Simulation time: {}s", (crowd_simulation.time_elapsed*100.0).round()/100.0), 12, 84, 20, Color::BLACK);
        
        frame_count += 1;
    }
    
    // */
    
}

