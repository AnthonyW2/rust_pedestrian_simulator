pub mod pedestrian {
    
    use std::f64::consts::TAU;
    
    use std::sync::Arc;
    
    use crate::simulation::simulator::simulator::SimArea;
    
    // The radius of a pedestrian, in metres
    pub const PEDESTRIAN_RADIUS: f64 = 2.0;
    
    // Some constants that denote a particular behaviour
    pub const ETIQUETTE_LEFT_BIAS: usize = 0;
    pub const ETIQUETTE_RIGHT_BIAS: usize = 1;
    pub const ETIQUETTE_RANDOM: usize = 2;
    
    pub struct Walker {
        /// Absolute x-coordinate the pedestrian, in metres.
        pub x: f64,
        /// Absolute y-coordinate the pedestrian
        pub y: f64,
        /// Instantaneous direction of travel, between 0 and 1, where 0 is east and the angle increases counterclockwise.
        pub facing_direction: f64,
        
        /// Preferred walking speed, in m/s.
        target_speed: f64,
        /// Instantaneous walking speed, in m/s.
        inst_speed: f64,
        
        /// The 2D environment that the pedestrian is within
        environment: Arc<SimArea>,
        /// The ID of the target location that the pedestrian walks towards
        target_location: usize
    }
    
    impl Walker {
        /// Create a new Walker object.
        /// 
        /// * `area` - A `SimArea` object describing the space for the simulation to be set in.
        pub fn new(environment: Arc<SimArea>, group: usize, start: usize, end: usize, target_speed: f64) -> Walker {
            Walker {
                x: environment.start_positions[group][start].0,
                y: environment.start_positions[group][start].1,
                facing_direction: 0.0,
                target_speed,
                inst_speed: 0.0,
                environment,
                target_location: end
            }
        }
        
        /// Simulate a small period of time in a single step.
        /// 
        /// time_scale: The amount of time (in seconds) that passes during each timestep
        pub fn simulate_timestep(&mut self, time_scale: f64) {
            println!("Simulating one pedestrian timestep...");
            
            self.apply_decisions();
            
            self.x += self.inst_speed * (self.facing_direction * TAU).cos();
            self.y += self.inst_speed * (self.facing_direction * TAU).sin();
            
            self.resolve_wall_collisions();
            
        }
        
        /// Use the general behaviours and the specific etiquette behaviours to determine the changes to this pedestrian's speed and direction of travel.
        pub fn apply_decisions(&mut self) {
            
            // Iterate through all neighbouring pedestrians and check for front-on collisions and side collisions.
            
            // Iterate through all walls of self.environment and ensure that the pedestrian does not walk that way.
            // Worst-case: re-align the direction of travel with the wall.
            
            
            
        }
        
        /// Check all walls in the relevant environment and resolve any collisions.
        pub fn resolve_wall_collisions(&mut self) {
            
            for wall in &self.environment.boundaries {
                let nudge = wall.get_walker_collision_vector(self);
                self.x += nudge.0;
                self.y += nudge.1;
            }
            
        }
        
    }
    
}
