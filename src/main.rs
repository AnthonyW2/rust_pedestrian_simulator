/**
 * Micro-scale Pedestrian Behaviour & etiquette simulator
 * 
 * Anthony Wilson
 * 2024
 */

use raylib::prelude::*;
use std::time;

use std::sync::Arc;

pub mod simulation;
use simulation::simulator::simulator::{SimArea, CrowdSim};
use simulation::pedestrian::pedestrian::Etiquette;


/// Speed multiplier if rendering the simulation
const SIM_SPEED: f64 = 2.0;

/// Run & display the simulation in real time (true), or run the entire simulation immediately & return the results (false)
const RENDER: bool = true;

/// Which simulation to use
/// 0 = calibration (goal: 18.57 ± 3s)
/// 1 = left bias
/// 2 = no bias
/// 3 = vertical calibration
/// 4 = diagonal example
/// 5 = crossroads (experimental)
/// _ = original debug sim
const SIM_TYPE: usize = 0;

/// Total number of pedestrians to simulate
const TOTAL_PEDESTRIANS: u32 = 1000;

/// Walkers per second during peak times
const WALKER_RATE: f64 = 0.8;

/// Create a simulation for callibration purposes
fn create_calibration_sim() -> CrowdSim {
    /// Normalised ratio of left-, non-, and right-biased pedestrians
    const BIAS_RATIOS: (f64, f64, f64) = (0.443877551020408, 0.520408163265306, 0.0357142857142857);
    
    let simulated_area = create_testing_environment();
    
    let mut crowd_simulation = CrowdSim::new(Arc::new(simulated_area), WALKER_RATE);
    
    // Pedestrians moving left-to-right
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.0*0.5) as usize, 0, Etiquette::LeftBias);
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.1*0.5) as usize, 0, Etiquette::NoBias);
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.2*0.5) as usize, 0, Etiquette::RightBias);
    
    // Pedestrians moving right-to-left
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.0*0.5) as usize, 1, Etiquette::LeftBias);
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.1*0.5) as usize, 1, Etiquette::NoBias);
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.2*0.5) as usize, 1, Etiquette::RightBias);
    
    crowd_simulation.randomise_pedestrian_order();
    
    return crowd_simulation;
    
}

/// Create a simulation for testing all pedestrians with a left bias
fn create_left_bias_sim() -> CrowdSim {
    let simulated_area = create_testing_environment();
    
    let mut crowd_simulation = CrowdSim::new(Arc::new(simulated_area), WALKER_RATE);
    
    // Pedestrians moving left-to-right
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*0.5) as usize, 0, Etiquette::LeftBias);
    
    // Pedestrians moving right-to-left
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*0.5) as usize, 1, Etiquette::LeftBias);
    
    crowd_simulation.randomise_pedestrian_order();
    
    return crowd_simulation;
    
}

/// Create a simulation for testing all pedestrians with no bias
fn create_no_bias_sim() -> CrowdSim {
    let simulated_area = create_testing_environment();
    
    let mut crowd_simulation = CrowdSim::new(Arc::new(simulated_area), WALKER_RATE);
    
    // Pedestrians moving left-to-right
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*0.5) as usize, 0, Etiquette::NoBias);
    
    // Pedestrians moving right-to-left
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*0.5) as usize, 1, Etiquette::NoBias);
    
    crowd_simulation.randomise_pedestrian_order();
    
    return crowd_simulation;
    
}

/// Create the simulation environment of interest
fn create_testing_environment() -> SimArea {
    let mut simulated_area = SimArea::new();
    
    simulated_area.add_wall((-1.0,0.0), (32.0,0.0));
    simulated_area.add_wall((-1.0,6.0), (32.0,6.0));
    simulated_area.add_wall((-1.0,0.0), (-1.0,6.0));
    simulated_area.add_wall((32.0,0.0), (32.0,6.0));
    
    // Timing barriers
    simulated_area.add_timing_boundary((3.0,0.0), (3.0,6.0));
    simulated_area.add_timing_boundary((28.0,0.0), (28.0,6.0));
    
    // Start & end group moving left-to-right
    simulated_area.add_start_end_group(
        vec![(0.0,1.0), (0.0,2.0), (0.0,3.0), (0.0,4.0), (0.0,5.0)],
        vec![(30.0,1.0), (30.0,2.0), (30.0,3.0), (30.0,4.0), (30.0,5.0)],
    );
    // Start & end group moving right-to-left
    simulated_area.add_start_end_group(
        vec![(31.0,1.0), (31.0,2.0), (31.0,3.0), (31.0,4.0), (31.0,5.0)],
        vec![(1.0,1.0), (1.0,2.0), (1.0,3.0), (1.0,4.0), (1.0,5.0)],
    );
    
    return simulated_area;
}

fn main() {
    
    let mut crowd_simulation;
    
    match SIM_TYPE {
        0 => {crowd_simulation = create_calibration_sim()},
        1 => {crowd_simulation = create_left_bias_sim()},
        2 => {crowd_simulation = create_no_bias_sim()},
        3 => {crowd_simulation = create_calibration_sim_vertical()},
        4 => {crowd_simulation = create_diagonal_demo_sim()},
        5 => {crowd_simulation = create_crossroads_sim()},
        _ => {crowd_simulation = create_demo_sim_1()}
    }
    
    if !RENDER {
        let results = crowd_simulation.simulate_full(0.02);
        //println!("All results: {:?}", results);
        
        let total_travel_time = results.2.iter().map(|t| t.0).sum::<f64>();
        
        let mean_travel_time: f64 = total_travel_time / (results.2.len() as f64);
        
        let travel_time_variance = results.2.iter().map(|t| {
            let diff = mean_travel_time - t.0;
            
            diff*diff
        }).sum::<f64>() / (results.2.len() as f64);
        let travel_time_stdev = travel_time_variance.sqrt();
        
        println!("Average travel time: {} ± {}s", (mean_travel_time * 100.0).round() / 100.0, (travel_time_stdev * 100.0).round() / 100.0);
        println!("Total simulation time: {} hours", (results.0/3600.0 * 100.0).round() / 100.0);
        println!("Total pedestrian time: {} man-hours", (total_travel_time/3600.0 * 100.0).round() / 100.0);
        
        return;
    }
    
    
    let (mut rl, thread) = raylib::init()
        .size(1500, 500)
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
        rl_handle.draw_text(&format!("Available/Active/Finished: {}/{}/{}", crowd_simulation.get_pedestrian_counts().0, crowd_simulation.get_pedestrian_counts().1, crowd_simulation.get_pedestrian_counts().2), 12, 108, 20, Color::BLACK);
        
        frame_count += 1;
    }
    
}



/// Demonstration & debugging simulation
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

/// Same as the calibration simulation, but using a vertical version of the environment
fn create_calibration_sim_vertical() -> CrowdSim {
    /// Normalised ratio of left-, non-, and right-biased pedestrians
    const BIAS_RATIOS: (f64, f64, f64) = (0.443877551020408, 0.520408163265306, 0.0357142857142857);
    
    let simulated_area = create_testing_environment_vertical();
    
    let mut crowd_simulation = CrowdSim::new(Arc::new(simulated_area), WALKER_RATE);
    
    // Pedestrians moving left-to-right
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.0*0.5) as usize, 0, Etiquette::LeftBias);
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.1*0.5) as usize, 0, Etiquette::NoBias);
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.2*0.5) as usize, 0, Etiquette::RightBias);
    
    // Pedestrians moving right-to-left
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.0*0.5) as usize, 1, Etiquette::LeftBias);
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.1*0.5) as usize, 1, Etiquette::NoBias);
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.2*0.5) as usize, 1, Etiquette::RightBias);
    
    crowd_simulation.randomise_pedestrian_order();
    
    return crowd_simulation;
    
}

/// Same as the testing environment, but mirrored about the line y = x
fn create_testing_environment_vertical() -> SimArea {
    let mut simulated_area = SimArea::new();
    
    simulated_area.add_wall((0.0,-1.0), (0.0,32.0));
    simulated_area.add_wall((6.0,-1.0), (6.0,32.0));
    simulated_area.add_wall((0.0,-1.0), (6.0,-1.0));
    simulated_area.add_wall((0.0,32.0), (6.0,32.0));
    
    // Timing barriers
    simulated_area.add_timing_boundary((0.0,3.0), (6.0,3.0));
    simulated_area.add_timing_boundary((0.0,28.0), (6.0,28.0));
    
    // Start & end group moving top-to-bottom
    simulated_area.add_start_end_group(
        vec![(1.0,0.0), (2.0,0.0), (3.0,0.0), (4.0,0.0), (5.0,0.0)],
        vec![(1.0,30.0), (2.0,30.0), (3.0,30.0), (4.0,30.0), (5.0,30.0)],
    );
    // Start & end group moving bottom-to-top
    simulated_area.add_start_end_group(
        vec![(1.0,31.0), (2.0,31.0), (3.0,31.0), (4.0,31.0), (5.0,31.0)],
        vec![(1.0,1.0), (2.0,1.0), (3.0,1.0), (4.0,1.0), (5.0,1.0)],
    );
    
    return simulated_area;
}

/// Simulation to demonstrate that diagonal boundaries work
fn create_diagonal_demo_sim() -> CrowdSim {
    /// Normalised ratio of left-, non-, and right-biased pedestrians
    const BIAS_RATIOS: (f64, f64, f64) = (0.44, 0.52, 0.04);
    
    let mut simulated_area_diagonal = SimArea::new();
    
    simulated_area_diagonal.add_wall((0.0,4.0), (12.0,16.0));
    simulated_area_diagonal.add_wall((4.0,0.0), (16.0,12.0));
    
    // Start & end group moving top-left to bottom-right
    simulated_area_diagonal.add_start_end_group(
        vec![(1.0,3.0), (2.0,2.0), (3.0,1.0)],
        vec![(14.0,16.0), (16.0,14.0)]
    );
    // Start & end group moving bottom-right to top-left
    simulated_area_diagonal.add_start_end_group(
        vec![(13.0,15.0), (14.0,14.0), (15.0,13.0)],
        vec![(0.0,2.0), (2.0,0.0)]
    );
    
    simulated_area_diagonal.add_timing_boundary((1.0,5.0), (5.0,1.0));
    simulated_area_diagonal.add_timing_boundary((11.0,15.0), (15.0,11.0));
    
    let mut crowd_simulation = CrowdSim::new(Arc::new(simulated_area_diagonal), WALKER_RATE);
    
    // Pedestrians moving left-to-right
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.0*0.5) as usize, 0, Etiquette::LeftBias);
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.1*0.5) as usize, 0, Etiquette::NoBias);
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.2*0.5) as usize, 0, Etiquette::RightBias);
    
    // Pedestrians moving right-to-left
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.0*0.5) as usize, 1, Etiquette::LeftBias);
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.1*0.5) as usize, 1, Etiquette::NoBias);
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.2*0.5) as usize, 1, Etiquette::RightBias);
    
    crowd_simulation.randomise_pedestrian_order();
    
    return crowd_simulation;
    
}

/// Experimental simulation with two crossing pathways
fn create_crossroads_sim() -> CrowdSim {
    /// Normalised ratio of left-, non-, and right-biased pedestrians
    const BIAS_RATIOS: (f64, f64, f64) = (0.44, 0.52, 0.04);
    
    let mut simulated_area_crossroads = SimArea::new();
    
    simulated_area_crossroads.add_wall((-1.0,12.5), (11.5,12.5));
    simulated_area_crossroads.add_wall((19.5,12.5), (32.0,12.5));
    simulated_area_crossroads.add_wall((-1.0,18.5), (11.5,18.5));
    simulated_area_crossroads.add_wall((19.5,18.5), (32.0,18.5));
    simulated_area_crossroads.add_wall((-1.0,12.5), (-1.0,18.5));
    simulated_area_crossroads.add_wall((32.0,12.5), (32.0,18.5));
    
    simulated_area_crossroads.add_wall((12.5,-1.0), (12.5,11.5));
    simulated_area_crossroads.add_wall((12.5,19.5), (12.5,32.0));
    simulated_area_crossroads.add_wall((18.5,-1.0), (18.5,11.5));
    simulated_area_crossroads.add_wall((18.5,19.5), (18.5,32.0));
    simulated_area_crossroads.add_wall((12.5,-1.0), (18.5,-1.0));
    simulated_area_crossroads.add_wall((12.5,32.0), (18.5,32.0));
    
    simulated_area_crossroads.add_wall((12.5,11.5), (11.5,12.5));
    simulated_area_crossroads.add_wall((18.5,11.5), (19.5,12.5));
    simulated_area_crossroads.add_wall((19.5,18.5), (18.5,19.5));
    simulated_area_crossroads.add_wall((11.5,18.5), (12.5,19.5));
    
    // Start & end group moving left-to-right
    simulated_area_crossroads.add_start_end_group(
        vec![(0.0,13.5), (0.0,14.5), (0.0,15.5), (0.0,16.5), (0.0,17.5)],
        vec![(30.0,13.5), (30.0,14.5), (30.0,15.5), (30.0,16.5), (30.0,17.5)],
    );
    // Start & end group moving right-to-left
    simulated_area_crossroads.add_start_end_group(
        vec![(31.0,13.5), (31.0,14.5), (31.0,15.5), (31.0,16.5), (31.0,17.5)],
        vec![(1.0,13.5), (1.0,14.5), (1.0,15.5), (1.0,16.5), (1.0,17.5)],
    );
    
    // Start & end group moving top-to-bottom
    simulated_area_crossroads.add_start_end_group(
        vec![(13.5,0.0), (14.5,0.0), (15.5,0.0), (16.5,0.0), (17.5,0.0)],
        vec![(13.5,30.0), (14.5,30.0), (15.5,30.0), (16.5,30.0), (17.5,30.0)],
    );
    // Start & end group moving bottom-to-top
    simulated_area_crossroads.add_start_end_group(
        vec![(13.5,31.0), (14.5,31.0), (15.5,31.0), (16.5,31.0), (17.5,31.0)],
        vec![(13.5,1.0), (14.5,1.0), (15.5,1.0), (16.5,1.0), (17.5,1.0)],
    );
    
    // Timing barriers
    simulated_area_crossroads.add_timing_boundary((3.0,12.5), (3.0,18.5));
    simulated_area_crossroads.add_timing_boundary((28.0,12.5), (28.0,18.5));
    simulated_area_crossroads.add_timing_boundary((12.5,3.0), (18.5,3.0));
    simulated_area_crossroads.add_timing_boundary((12.5,28.0), (18.5,28.0));
    
    let mut crowd_simulation = CrowdSim::new(Arc::new(simulated_area_crossroads), WALKER_RATE);
    
    // Pedestrians moving left-to-right
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.0*0.25) as usize, 0, Etiquette::LeftBias);
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.1*0.25) as usize, 0, Etiquette::NoBias);
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.2*0.25) as usize, 0, Etiquette::RightBias);
    
    // Pedestrians moving right-to-left
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.0*0.25) as usize, 1, Etiquette::LeftBias);
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.1*0.25) as usize, 1, Etiquette::NoBias);
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.2*0.25) as usize, 1, Etiquette::RightBias);
    
    //// Pedestrians moving top-to-bottom
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.0*0.25) as usize, 2, Etiquette::LeftBias);
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.1*0.25) as usize, 2, Etiquette::NoBias);
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.2*0.25) as usize, 2, Etiquette::RightBias);
    //
    //// Pedestrians moving bottom-to-top
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.0*0.25) as usize, 3, Etiquette::LeftBias);
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.1*0.25) as usize, 3, Etiquette::NoBias);
    crowd_simulation.add_pedestrian_set(((TOTAL_PEDESTRIANS as f64)*BIAS_RATIOS.2*0.25) as usize, 3, Etiquette::RightBias);
    
    crowd_simulation.randomise_pedestrian_order();
    
    return crowd_simulation;
    
}

