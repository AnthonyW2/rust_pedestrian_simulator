pub mod pedestrian {
    
    //use std::f64::consts::TAU;
    
    use std::sync::Arc;
    
    use crate::simulation::simulator::simulator::SimArea;
    
    /// The acceleration of a pedestrian, in m⋅s^-2
    const PEDESTRIAN_ACCEL: f64 = 0.1;
    
    /// The speed at which a pedestrian changes its facing direction, in radians per second.
    const PEDESTRIAN_DIRECTION_SPEED: f64 = 0.5;
    
    /// The radius of a pedestrian's body, in metres
    pub const PEDESTRIAN_RADIUS: f64 = 0.4;
    
    /// The radius of a pedestrian's personal space, in metres
    pub const PEDESTRIAN_SPACE_RADIUS: f64 = 0.4;
    
    /// The intensity of repulsion between pedestrians within the personal space radius (acceleration, m⋅s^-2)
    pub const PEDESTRIAN_REPULSION: f64 = 0.1;
    
    /// The speed at which a pedestrian changes its facing direction when within the personal space radius, in radians per second.
    const PEDESTRIAN_DIRECTION_REPULSION: f64 = 0.5;
    
    // Some constants that denote a particular behaviour
    pub const ETIQUETTE_LEFT_BIAS: usize = 0;
    pub const ETIQUETTE_RIGHT_BIAS: usize = 1;
    pub const ETIQUETTE_RANDOM: usize = 2;
    
    pub struct Walker {
        /// Absolute x-coordinate the pedestrian, in metres.
        pub x: f64,
        /// Absolute y-coordinate the pedestrian
        pub y: f64,
        /// Instantaneous direction of travel, in radians (between 0 and 2π).
        pub facing_direction: f64,
        
        /// Preferred walking speed, in m/s.
        target_speed: f64,
        /// Instantaneous walking speed, in m/s.
        inst_speed: f64,
        
        /// The 2D environment that the pedestrian is within
        environment: Arc<SimArea>,
        /// The group that the pedestrian is a part of
        group: usize,
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
                group,
                target_location: end
            }
        }
        
        /// Simulate a small period of time in a single step.
        /// 
        /// `time_scale`: The amount of time (in seconds) that passes during each timestep
        /// `other_pedestrians_before`: A list of pedestrian positions (that have already been simulated)
        /// `other_pedestrians_after`: A list of pedestrian positions (that are yet to be simulated)
        pub fn simulate_timestep(&mut self, time_scale: f64, other_pedestrians_before: &[(f64, f64)], other_pedestrians_after: &[(f64, f64)]) {
            println!("Simulating one pedestrian timestep...");
            
            // Find the distance and normal vector to each wall/boundary in the simulation
            let wall_normals = self.environment.boundaries.iter().map(|wall| wall.get_normal_vector((self.x, self.y))).collect::<Vec<_>>();
            
            self.apply_decisions(wall_normals, other_pedestrians_before, other_pedestrians_after);
            
            // Apply acceleration/deceleration to change velocity
            if self.inst_speed < self.target_speed {
                self.inst_speed += PEDESTRIAN_ACCEL;
            }
            if self.inst_speed > self.target_speed {
                self.inst_speed = self.target_speed;
            }
            
            let target_x = self.environment.end_positions[self.group][self.target_location].0;
            let target_y = self.environment.end_positions[self.group][self.target_location].1;
            
            // Angle = y.atan2(x)
            
            
            // Apply velocity to change position
            self.x += self.inst_speed * self.facing_direction.cos();
            self.y += self.inst_speed * self.facing_direction.sin();
            
            self.resolve_wall_collisions();
            
        }
        
        /// Use the general behaviours and the specific etiquette behaviours to determine the changes to this pedestrian's speed and direction of travel.
        pub fn apply_decisions(&mut self, wall_normals: Vec<(f64, (f64, f64))>, other_pedestrians_before: &[(f64, f64)], other_pedestrians_after: &[(f64, f64)]) {
            
            // Iterate through all neighbouring pedestrians and check for front-on collisions and side collisions.
            
            
            // Iterate through all walls of self.environment and ensure that the pedestrian does not walk that way.
            // Worst-case: re-align the direction of travel with the wall.
            
            
        }
        
        /// Check all walls in the relevant environment and resolve any collisions.
        pub fn resolve_wall_collisions(&mut self) {
            
            for wall in &self.environment.boundaries {
                // Get the normal vector to the wall
                let (dist, normal) = wall.get_normal_vector((self.x, self.y));
                
                // Edge case: if the pedestrian is on the line, don't do anything
                if dist == 0.0 {
                    return;
                }
                
                if dist < PEDESTRIAN_RADIUS {
                    // Pedestrian needs to be nudged away from the wall by some multiple (k) of the normal vector
                    let k = PEDESTRIAN_RADIUS/dist - 1.0;
                    
                    // Move the pedestrian away from the wall
                    self.x += normal.0 * k;
                    self.y += normal.1 * k;
                }
                
                // Change the facing direction of the pedestrian
                
                
                // If a collision occurred: pedestrian should face perpendicular to the wall
                
                
                // If the wall is within the pedestrian's personal space radius, nudge the direction vector away slightly
                
                
            }
            
        }
        
    }
    
}
