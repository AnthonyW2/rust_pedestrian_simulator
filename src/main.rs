use raylib::prelude::*;
use std::time;

use std::sync::Arc;

pub mod simulation;
use simulation::simulator::simulator::{SimArea, CrowdSim};
use simulation::pedestrian::pedestrian::Etiquette;


const SIM_SPEED: f64 = 4.0;

/// Create a simple demonstration & debugging simulation
fn create_demo_sim() -> CrowdSim {
    let mut simulated_area_1 = SimArea::new();
    
    simulated_area_1.add_wall((-1.0,0.0), (32.0,0.0));
    simulated_area_1.add_wall((-1.0,6.0), (32.0,6.0));
    simulated_area_1.add_wall((-1.0,0.0), (-1.0,6.0));
    simulated_area_1.add_wall((32.0,0.0), (32.0,6.0));
    
    // Timing barriers
    simulated_area_1.add_timing_boundary((3.0,0.0), (3.0,6.0));
    simulated_area_1.add_timing_boundary((28.0,0.0), (28.0,6.0));
    
    // Start & end group moving left-to-right
    simulated_area_1.add_start_end_group(
        vec![(0.0,1.0), (0.0,2.0), (0.0,3.0), (0.0,4.0), (0.0,5.0)],
        vec![(30.0,1.0), (30.0,2.0), (30.0,3.0), (30.0,4.0), (30.0,5.0)],
    );
    // Start & end group moving right-to-left
    simulated_area_1.add_start_end_group(
        vec![(31.0,1.0), (31.0,2.0), (31.0,3.0), (31.0,4.0), (31.0,5.0)],
        vec![(1.0,1.0), (1.0,2.0), (1.0,3.0), (1.0,4.0), (1.0,5.0)],
    );
    
    let mut crowd_simulation = CrowdSim::new(Arc::new(simulated_area_1), 0.8);
    
    // Pedestrians moving left-to-right
    crowd_simulation.add_pedestrian_set(10,0,Etiquette::LeftBias);
    crowd_simulation.add_pedestrian_set(12,0,Etiquette::NoBias);
    crowd_simulation.add_pedestrian_set(1,0,Etiquette::RightBias);
    
    // Pedestrians moving right-to-left
    crowd_simulation.add_pedestrian_set(10,1,Etiquette::LeftBias);
    crowd_simulation.add_pedestrian_set(12,1,Etiquette::NoBias);
    crowd_simulation.add_pedestrian_set(1,1,Etiquette::RightBias);
    
    crowd_simulation.randomise_pedestrian_order();
    
    return crowd_simulation;
    
}

fn main() {
    
    let mut crowd_simulation = create_demo_sim();
    
    let (mut rl, thread) = raylib::init()
        .size(1600, 600)
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
        crowd_simulation.simulate_timestep(SIM_SPEED * frame_time.as_secs_f64());
        crowd_simulation.draw(&mut rl_handle, (100,150));
        
        // Debug text
        rl_handle.draw_text("Pedestrian Behaviour Simulator", 12, 12, 20, Color::BLACK);
        rl_handle.draw_text(&format!("Frame count: {}", frame_count), 12, 36, 20, Color::BLACK);
        rl_handle.draw_text(&format!("Frame time: {}ms", frame_time.as_millis()), 12, 60, 20, Color::BLACK);
        rl_handle.draw_text(&format!("Simulation time: {}s", (crowd_simulation.time_elapsed*100.0).round()/100.0), 12, 84, 20, Color::BLACK);
        
        frame_count += 1;
    }
    
}



// Other demo simulations:
/*
fn create_demo_sim_1() -> CrowdSim {
    let mut simulated_area_1 = SimArea::new();
    
    simulated_area_1.add_wall((0.0,0.0), (20.0,0.0));
    simulated_area_1.add_wall((0.0,8.0), (20.0,8.0));
    // Start & end group moving left-to-right
    simulated_area_1.add_start_end_group(
        vec![(0.0,1.0), (0.0,3.0), (0.0,5.0), (0.0,7.0)],
        vec![(21.0,1.0), (21.0,3.0), (21.0,5.0), (21.0,7.0), (5.0, 5.0)]
    );
    // Start & end group moving right-to-left
    simulated_area_1.add_start_end_group(
        vec![(20.0,1.0), (20.0,3.0), (20.0,5.0), (20.0,7.0)],
        vec![(-1.0,1.0), (-1.0,3.0), (-1.0,5.0), (-1.0,7.0), (5.0, 4.0)]
    );
    
    let mut crowd_simulation = CrowdSim::new(Arc::new(simulated_area_1), 4.0);
    
    // Pedestrians moving left-to-right
    crowd_simulation.add_pedestrian(0, 3, 4, 1.35, Etiquette::LeftBias);
    crowd_simulation.add_pedestrian(0, 0, 2, 1.35, Etiquette::LeftBias);
    crowd_simulation.add_pedestrian(0, 1, 0, 1.35, Etiquette::LeftBias);
    crowd_simulation.add_pedestrian(0, 2, 0, 1.35, Etiquette::LeftBias);
    crowd_simulation.add_pedestrian(0, 2, 1, 1.35, Etiquette::LeftBias);
    crowd_simulation.add_pedestrian(0, 2, 1, 2.5,  Etiquette::LeftBias);
    crowd_simulation.add_pedestrian(0, 2, 1, 2.0,  Etiquette::LeftBias);
    
    // Pedestrians moving right-to-left
    crowd_simulation.add_pedestrian(1, 3, 4, 1.35, Etiquette::NoBias);
    crowd_simulation.add_pedestrian(1, 0, 2, 1.35, Etiquette::NoBias);
    crowd_simulation.add_pedestrian(1, 1, 0, 1.35, Etiquette::NoBias);
    crowd_simulation.add_pedestrian(1, 2, 0, 1.35, Etiquette::NoBias);
    crowd_simulation.add_pedestrian(1, 2, 1, 1.35, Etiquette::NoBias);
    
    crowd_simulation.randomise_pedestrian_order();
    
    return crowd_simulation;
    
}

// */
