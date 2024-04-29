pub mod pedestrian {
    
    use std::f64::consts::TAU;

    use crate::simulation::simulator::simulator::SimArea;
    
    // The radius of a pedestrian, in metres
    const PEDESTRIAN_RADIUS: f64 = 0.4;
    
    const ETIQUETTE_LEFT_BIAS: usize = 0;
    const ETIQUETTE_RIGHT_BIAS: usize = 1;
    const ETIQUETTE_RANDOM: usize = 2;
    
    pub struct Walker<'a> {
        /// Absolute x-coordinate the pedestrian, in metres.
        x: f64,
        /// Absolute y-coordinate the pedestrian
        y: f64,
        /// Preferred walking speed, in m/s.
        target_speed: f64,
        /// Instantaneous walking speed, in m/s.
        inst_speed: f64,
        /// Instantaneous direction of travel, between 0 and 1, where 0 is east and the angle increases counterclockwise.
        facing_direction: f64,
        
        /// The 2D environment that the pedestrian is within
        environment: &'a SimArea,
        /// The ID of the target location that the pedestrian walks towards
        target_location: usize
    }
    
    //pub struct Behaviour {
    //    
    //}
    
    impl Walker<'_> {
        /// Create a new Walker object.
        /// 
        /// * `area` - A `SimArea` object describing the space for the simulation to be set in.
        pub fn new(environment: &'static SimArea, group: usize, start: usize, end: usize, target_speed: f64) -> Walker<'static> {
            Walker {
                x: environment.start_positions[group][start].0,
                y: environment.start_positions[group][start].1,
                target_speed,
                inst_speed: 0.0,
                facing_direction: 0.0,
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
            
            //self.x += self.inst_speed * cos(self.direction_angle * TAU);
            self.x += self.inst_speed;
            self.y += self.inst_speed;
            
        }
        
        /// Use the general behaviours and the specific etiquette behaviours to determine the changes to this pedestrian's speed and direction of travel.
        pub fn apply_decisions(&mut self) {
            
            // Iterate through all neighbouring pedestrians and check for front-on collisions and side collisions.
            
            // Iterate through all walls of self.environment and ensure that the pedestrian does not walk that way.
            // Worst-case: re-align the direction of travel with the wall.
            
            
            
        }
        
        /// Check all walls in the relevant environment and resolve any collisions.
        pub fn resolve_wall_collisions(&mut self) {
            
        }
        
    }
    
}
