pub mod pedestrian {
    
    use std::f64::consts::{PI, TAU};
    use std::sync::Arc;
    use raylib::{drawing::{RaylibDrawHandle, RaylibDraw}, color::Color};
    use crate::simulation::simulator::simulator::{SimArea, DRAW_SCALE};
    
    
    /// The acceleration of a pedestrian, in m⋅s^-2
    const PEDESTRIAN_ACCEL: f64 = 0.8;
    
    /// A multiplier applied to the direction alignment function
    const PEDESTRIAN_DIRECTION_CHANGE_FACTOR: f64 = 1.0;
    
    /// The radius of a pedestrian's body, in metres
    const PEDESTRIAN_RADIUS: f64 = 0.4;
    
    /// The radius of a pedestrian's personal space, in metres
    const PEDESTRIAN_SPACE_RADIUS: f64 = 0.6;
    
    /// The intensity of repulsion between pedestrians within the personal space radius (acceleration, m⋅s^-2)
    const PEDESTRIAN_REPULSION: f64 = 0.1;
    
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
        pub fn simulate_timestep(&mut self, time_scale: f64, other_pedestrians_before: &[(f64, f64, f64)], other_pedestrians_after: &[(f64, f64, f64)]) {
            //println!("Simulating one pedestrian timestep...");
            
            // Find the distance and normal vector to each wall/boundary in the simulation
            let wall_normals = self.environment.boundaries.iter().map(|wall| wall.get_normal_vector((self.x, self.y))).collect::<Vec<_>>();
            
            self.apply_decisions(wall_normals, other_pedestrians_before, other_pedestrians_after);
            
            // Apply acceleration/deceleration to change velocity
            if self.inst_speed < self.target_speed {
                self.inst_speed += PEDESTRIAN_ACCEL * time_scale;
            }
            if self.inst_speed > self.target_speed {
                self.inst_speed = self.target_speed;
            }
            
            // Update the facing direction to be better aligned with the destination
            self.facing_direction = self.get_new_direction(time_scale);
            
            // Apply velocity to change position
            self.x += self.inst_speed * self.facing_direction.cos() * time_scale;
            self.y += self.inst_speed * self.facing_direction.sin() * time_scale;
            
            self.resolve_wall_collisions();
            
        }
        
        /// Find a facing direction that is better aligned with the direction of the destination
        fn get_new_direction(&self, time_scale: f64) -> f64 {
            
            let target_x = self.environment.end_positions[self.group][self.target_location].0;
            let target_y = self.environment.end_positions[self.group][self.target_location].1;
            
            // The angle the pedestrian should be facing (between 0 and 2π)
            let target_angle = (target_y - self.y).atan2(target_x - self.x);
            
            // The difference between the facing and target angles
            let mut angle_diff = self.facing_direction - target_angle;
            
            // Constrain angle_diff between -π and π
            if angle_diff > PI {
                angle_diff -= TAU;
            }
            
            // Return the new facing direction of the pedestrian
            return ((self.facing_direction - angle_diff*PEDESTRIAN_DIRECTION_CHANGE_FACTOR*time_scale) + TAU) % TAU;
        }
        
        /// Use the general behaviours and the specific etiquette behaviours to determine the changes to this pedestrian's speed and direction of travel.
        fn apply_decisions(&mut self, wall_normals: Vec<(f64, (f64, f64))>, other_pedestrians_before: &[(f64, f64, f64)], other_pedestrians_after: &[(f64, f64, f64)]) {
            
            // Iterate through all neighbouring pedestrians and check for front-on collisions and side collisions.
            
            // Check yet-to-be-simulated pedestrians first
            
            // Check already-simulated pedestrians next
            
            
            
            // Iterate through all walls of self.environment and ensure that the pedestrian does not walk that way.
            // Worst-case: re-align the direction of travel with the wall.
            
            
        }
        
        /// Check all walls in the relevant environment and resolve any collisions.
        fn resolve_wall_collisions(&mut self) {
            
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
        
        /// Draw this pedestrian with RayLib
        pub fn draw(&self, rl_handle: &mut RaylibDrawHandle, offset: (i32, i32)) {
            
            rl_handle.draw_ellipse(
                offset.0 + ((DRAW_SCALE as f64)*self.x) as i32,
                offset.1 + ((DRAW_SCALE as f64)*self.y) as i32,
                (DRAW_SCALE as f32) * (PEDESTRIAN_RADIUS as f32),
                (DRAW_SCALE as f32) * (PEDESTRIAN_RADIUS as f32),
                Color::from_hex("505050").unwrap()
            );
            
            rl_handle.draw_line(
                offset.0 + ((DRAW_SCALE as f64)*self.x) as i32,
                offset.1 + ((DRAW_SCALE as f64)*self.y) as i32,
                offset.0 + ((DRAW_SCALE as f64)*(self.x + self.inst_speed * self.facing_direction.cos())) as i32,
                offset.1 + ((DRAW_SCALE as f64)*(self.y + self.inst_speed * self.facing_direction.sin())) as i32,
                Color::from_hex("FF0000").unwrap()
            );
            
            let target_x = self.environment.end_positions[self.group][self.target_location].0;
            let target_y = self.environment.end_positions[self.group][self.target_location].1;
            let target_angle = ((target_y - self.y).atan2(target_x - self.x) + TAU) % TAU;
            
            rl_handle.draw_line(
                offset.0 + ((DRAW_SCALE as f64)*self.x) as i32,
                offset.1 + ((DRAW_SCALE as f64)*self.y) as i32,
                offset.0 + ((DRAW_SCALE as f64)*(self.x + target_angle.cos())) as i32,
                offset.1 + ((DRAW_SCALE as f64)*(self.y + target_angle.sin())) as i32,
                Color::from_hex("FF8000").unwrap()
            );
            
            
        }
        
    }
    
}
