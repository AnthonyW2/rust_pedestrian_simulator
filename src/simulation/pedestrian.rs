pub mod pedestrian {
    
    use std::f64::consts::{PI, TAU};
    use std::sync::Arc;
    use raylib::{drawing::{RaylibDrawHandle, RaylibDraw}, color::Color, math::Vector2};
    use rand;
    
    use crate::simulation::simulator::simulator::{SimArea, DRAW_SCALE};
    
    
    /// The acceleration of a pedestrian, in m⋅s^-2
    const PEDESTRIAN_ACCEL: f64 = 0.8;
    
    /// A multiplier applied to destination alignment
    const PEDESTRIAN_DIRECTION_CHANGE_FACTOR: f64 = 1.0;
    
    /// The radius of a pedestrian's body, in metres
    const PEDESTRIAN_RADIUS: f64 = 0.205;
    
    /// Personal space: compressible radius of collision-avoidance, in metres
    const PEDESTRIAN_PSPACE_RADIUS: f64 = 0.856;
    
    /// The distance a pedestrian looks ahead for obstacles, in metres
    const PEDESTRIAN_LOOK_AHEAD_RADIUS: f64 = 2.0;
    /// The distance a pedestrian looks ahead for obstacles, in metres
    const PEDESTRIAN_LOOK_BESIDE_RADIUS: f64 = 1.2;
    
    /// The distance a pedestrian looks ahead for obstacles, in metres
    const PEDESTRIAN_LOOK_AHEAD_FOV: f64 = PI/2.0;
    /// The distance a pedestrian looks ahead for obstacles, in metres
    const PEDESTRIAN_LOOK_BESIDE_FOV: f64 = PI/2.0;
    
    /// Intensity of which a pedestrian changes its facing direction when another pedestrian is in front and travelling in the opposite direction
    const PEDESTRIAN_OPPOSING_REPULSION: f64 = 0.25;
    
    /// The speed at which a pedestrian changes its facing direction when within the personal space radius
    const PEDESTRIAN_PSPACE_REPULSION: f64 = 0.3;
    
    /// The deceleration of a pedestrian when another pedestrian is oncoming
    const PEDESTRIAN_OPPOSING_DECEL: f64 = PEDESTRIAN_ACCEL * 1.5;
    
    /// The intensity of repulsion from a wall within the personal space radius
    const WALL_REPULSION: f64 = 0.2;
    
    /// Intensity of random noise added to pedestrian speed
    const PEDESTRIAN_SPEED_NOISE_FACTOR: f64 = 0.8;
    /// Intensity of random noise added to pedestrian facing direction
    const PEDESTRIAN_DIRECTION_NOISE_FACTOR: f64 = 0.4;
    
    /// Intensity of bias (to facing direction) caused by Etiquette::LEFT_BIAS or Etiquette::RIGHT_BIAS
    const PEDESTRIAN_ETIQUETTE_BIAS_FACTOR: f64 = 0.25;
    
    // Etiquette option enum
    #[derive(PartialEq)]
    pub enum Etiquette {
        LeftBias,   // Stay to the left
        RightBias,  // Stay to the right
        NoBias      // Walk directly towards the destination
    }
    
    pub struct Walker {
        /// Absolute x-coordinate the pedestrian, in metres.
        pub x: f64,
        /// Absolute y-coordinate the pedestrian
        pub y: f64,
        /// Instantaneous direction of travel, in radians (between 0 and 2π). Note: all angles increase clockwise because the y-axis increase downward.
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
        target_location: usize,
        
        /// Whether or not each timing boundary has been hit
        timing_boundary_states: Vec<bool>,
        /// The time since passing the first timing boundary
        timing_boundary_elapsed: Option<f64>,
        
        /// The tested behavioural rule that this pedestrian follows
        etiquette: Etiquette
    }
    
    impl Walker {
        /// Create a new Walker object.
        /// 
        /// * `area` - A `SimArea` object describing the space for the simulation to be set in.
        pub fn new(environment: Arc<SimArea>, group: usize, start: usize, end: usize, target_speed: f64, etiquette: Etiquette) -> Walker {
            let timing_boundary_count = environment.timing_boundaries.len();
            let start_coords = environment.start_positions[group][start];
            let end_coords = environment.end_positions[group][end];
            
            Walker {
                x: start_coords.0,
                y: start_coords.1,
                // Initially point towards destination
                facing_direction: ((end_coords.1 - start_coords.1).atan2(end_coords.0 - start_coords.0) + TAU) % TAU,
                target_speed,
                inst_speed: 0.0,
                environment,
                group,
                target_location: end,
                timing_boundary_states: vec![false; timing_boundary_count],
                timing_boundary_elapsed: None,
                etiquette
            }
        }
        
        /// Simulate a small period of time in a single step.
        /// 
        /// `time_scale`: The amount of time (in seconds) that passes during each timestep
        /// `other_pedestrians_before`: A list of pedestrian positions (that have already been simulated)
        /// `other_pedestrians_after`: A list of pedestrian positions (that are yet to be simulated)
        pub fn simulate_timestep(&mut self, time_scale: f64, other_pedestrians_before: &[(f64, f64, f64)], other_pedestrians_after: &[(f64, f64, f64)]) {
            //println!("Simulating one pedestrian timestep...");
            
            // Apply acceleration/deceleration to change velocity
            self.inst_speed = self.target_speed.min(self.inst_speed + PEDESTRIAN_ACCEL * time_scale);
            
            // Coordinates of the destination
            let target_x = self.environment.end_positions[self.group][self.target_location].0;
            let target_y = self.environment.end_positions[self.group][self.target_location].1;
            
            // The angle the pedestrian should be facing to reach their destination (between 0 and 2π)
            let target_angle = (target_y - self.y).atan2(target_x - self.x);
            
            // Update the facing direction to be better aligned with the destination
            self.facing_direction = nudge_angle(self.facing_direction, target_angle, PEDESTRIAN_DIRECTION_CHANGE_FACTOR*time_scale);
            
            
            // Add bias to movement direction depending on etiquette
            if self.etiquette == Etiquette::LeftBias {
                self.facing_direction -= PEDESTRIAN_ETIQUETTE_BIAS_FACTOR * time_scale;
            } else if self.etiquette == Etiquette::RightBias {
                self.facing_direction += PEDESTRIAN_ETIQUETTE_BIAS_FACTOR * time_scale;
            }
            
            
            self.react_to_neighbours(time_scale, other_pedestrians_after);
            self.react_to_neighbours(time_scale, other_pedestrians_before);
            
            self.apply_noise(time_scale);
            
            
            // Apply velocity to change position
            self.x += self.inst_speed * self.facing_direction.cos() * time_scale;
            self.y += self.inst_speed * self.facing_direction.sin() * time_scale;
            
            self.resolve_wall_collisions(time_scale);
            
            
            self.check_timing_boundaries(time_scale);
            
        }
        
        /// React to neighbouring pedestrians, considering specific etiquette rules
        /// 
        /// * `other_pedestrians` - [(x, y, direction)]
        fn react_to_neighbours(&mut self, time_scale: f64, other_pedestrians: &[(f64, f64, f64)]) {
            
            // Iterate through all neighbouring pedestrians and check for front-on collisions and side collisions.
            
            /* How this method works:
             * Resolve collisions between pedestrians.
             * * Move them apart so that hey are no longer touching.
             * * Change direction of travel so that they are travelling perpendicular to the other pedestrian.
             * * Reduce speed to 0.
             * If a pedestrian is to the right or left: cancel right or left bias effects.
             * If a pedestrian is in front:
             * * If they are walking in the opposite direction:
             * * * Move the direction of travel away from the oncoming pedestrian - either towards the perpendicular or towards the normal.
             * * If they are walking in the same direction, reduce acceleration.
             * * If they are also within the personal space radius, decelerate.
             */
            
            for (n_x, n_y, n_dir) in other_pedestrians {
                let dist = ((self.x - n_x)*(self.x - n_x) + (self.y - n_y)*(self.y - n_y)).sqrt();
                
                // The direction the neighbour is in, between -π and π
                let abs_neighbour_angle = (n_y - self.y).atan2(n_x - self.x);
                
                // Intersecting hitbox
                if dist < 2.0*PEDESTRIAN_RADIUS {
                    //println!("Collision");
                    
                    // Compute the overlap between the two pedestrians
                    let k = 2.0*PEDESTRIAN_RADIUS - dist;
                    
                    // Move the pedestrian away from its neighbour
                    self.x -= abs_neighbour_angle.cos() * k;
                    self.y -= abs_neighbour_angle.sin() * k;
                    
                    // Set facing angle directly away from neighbour
                    self.facing_direction = abs_neighbour_angle + PI;
                    
                    // Set facing angle perpendicular to the direction toward the neighbour, depending on etiquette
                    //if self.etiquette == Etiquette::LeftBias {
                    //    self.facing_direction = (abs_neighbour_angle - PI/2.0 + TAU) % TAU;
                    //} else if self.etiquette == Etiquette::RightBias {
                    //    self.facing_direction = (abs_neighbour_angle + PI/2.0 + TAU) % TAU;
                    //} else {
                    //    // Random direction
                    //    self.facing_direction = ((if rand::random::<f64>() > 0.5 {abs_neighbour_angle - PI/2.0} else {abs_neighbour_angle + PI/2.0}) + TAU) % TAU;
                    //}
                    
                    // Set speed to 0
                    self.inst_speed = 0.0;
                }
                
                // The direction the neighbour is in, relative to the direction of travel of this pedestrian, between 0 and 2π
                let travel_rel_angle = (abs_neighbour_angle - self.facing_direction + TAU + TAU) % TAU;
                
                // Within view to the right
                if dist < PEDESTRIAN_LOOK_BESIDE_RADIUS && travel_rel_angle > PEDESTRIAN_LOOK_AHEAD_FOV/2.0 && travel_rel_angle < PEDESTRIAN_LOOK_AHEAD_FOV/2.0 + PEDESTRIAN_LOOK_BESIDE_FOV {
                    // Cancel right-bias
                    if self.etiquette == Etiquette::RightBias {
                        self.facing_direction -= PEDESTRIAN_ETIQUETTE_BIAS_FACTOR * time_scale;
                    }
                }
                
                // Within view to the left
                if dist < PEDESTRIAN_LOOK_BESIDE_RADIUS && travel_rel_angle < TAU-PEDESTRIAN_LOOK_AHEAD_FOV/2.0 && travel_rel_angle > TAU-(PEDESTRIAN_LOOK_AHEAD_FOV/2.0 + PEDESTRIAN_LOOK_BESIDE_FOV) {
                    // Cancel left-bias
                    if self.etiquette == Etiquette::LeftBias {
                        self.facing_direction += PEDESTRIAN_ETIQUETTE_BIAS_FACTOR * time_scale;
                    }
                }
                
                // Recalculate relative neighbour direction
                let travel_rel_angle = (abs_neighbour_angle - self.facing_direction + TAU + TAU) % TAU;
                
                // Within view in front
                if dist < PEDESTRIAN_LOOK_AHEAD_RADIUS && (travel_rel_angle <= PEDESTRIAN_LOOK_AHEAD_FOV/2.0 || travel_rel_angle >= TAU-PEDESTRIAN_LOOK_AHEAD_FOV/2.0) {
                    let direction_difference = (self.facing_direction - n_dir + TAU) % TAU;
                    
                    if direction_difference > PI/2.0 && direction_difference < 3.0*PI/2.0 {
                        // Oncoming
                        
                        if self.etiquette == Etiquette::LeftBias {
                            // Apply bias
                            //self.facing_direction -= PEDESTRIAN_ETIQUETTE_BIAS_FACTOR * time_scale / 2.0;
                            
                            // The angle that points away from the neighbouring pedestrian, between 0 and 2π
                            //let away_angle = abs_neighbour_angle + PI;
                            let away_angle = (abs_neighbour_angle - PI/2.0 + TAU) % TAU;
                            
                            // Nudge the direction of travel away from the neighbour
                            self.facing_direction = nudge_angle(self.facing_direction, away_angle, PEDESTRIAN_OPPOSING_REPULSION*time_scale);
                            
                        } else if self.etiquette == Etiquette::RightBias {
                            // Apply bias
                            //self.facing_direction += PEDESTRIAN_ETIQUETTE_BIAS_FACTOR * time_scale / 2.0;
                            
                            // The angle that points away from the neighbouring pedestrian, between 0 and 2π
                            //let away_angle = abs_neighbour_angle + PI;
                            let away_angle = (abs_neighbour_angle + PI/2.0 + TAU) % TAU;
                            
                            // Nudge the direction of travel away from the neighbour
                            self.facing_direction = nudge_angle(self.facing_direction, away_angle, PEDESTRIAN_OPPOSING_REPULSION*time_scale);
                            
                        } else {
                            // No directional bias
                            // Slow down a bit
                            self.inst_speed -= PEDESTRIAN_ACCEL*time_scale/2.0;
                            
                            // The angle that points away from the neighbouring pedestrian, between 0 and 2π
                            let away_angle = abs_neighbour_angle + PI;
                            
                            // Nudge the direction of travel away from the neighbour
                            self.facing_direction = nudge_angle(self.facing_direction, away_angle, PEDESTRIAN_OPPOSING_REPULSION*time_scale);
                            
                        }
                        
                    } else {
                        // Moving same direction - reduce acceleration
                        self.inst_speed = 0f64.max(self.inst_speed - PEDESTRIAN_ACCEL * time_scale / 2.0);
                    }
                    
                    // Within personal space as well
                    if dist < PEDESTRIAN_RADIUS + PEDESTRIAN_PSPACE_RADIUS {
                        // Decelerate
                        self.inst_speed = 0f64.max(self.inst_speed - PEDESTRIAN_OPPOSING_DECEL * time_scale);
                    }
                    
                }
                
                // Within personal space
                if dist < PEDESTRIAN_RADIUS + PEDESTRIAN_PSPACE_RADIUS {
                    // Change the direction of travel to align better with the angle facing away from the neighbour
                    
                    // The angle that points away from the neighbouring pedestrian, between 0 and 2π
                    let away_angle = abs_neighbour_angle + PI;
                    
                    // Nudge the direction of travel away from the neighbour
                    // Note that the intensity is inversely proportional to the separation distance
                    self.facing_direction = nudge_angle(self.facing_direction, away_angle, PEDESTRIAN_PSPACE_REPULSION*time_scale/dist);
                    
                }
                
            }
            
        }
        
        /// Apply some small random fluctuations to the facing direction and current speed
        fn apply_noise(&mut self, time_scale: f64) {
            
            self.facing_direction += (2.0 * rand::random::<f64>() - 1.0) * PEDESTRIAN_DIRECTION_NOISE_FACTOR * time_scale;
            self.inst_speed += (2.0 * rand::random::<f64>() - 1.0) * PEDESTRIAN_SPEED_NOISE_FACTOR * time_scale;
            
        }
        
        /// Check all walls in the relevant environment and resolve any collisions.
        fn resolve_wall_collisions(&mut self, time_scale: f64) {
            
            for wall in &self.environment.boundaries {
                // Get the normal vector to the wall
                let (dist, normal) = wall.get_normal_vector((self.x, self.y));
                
                let normal_angle = normal.1.atan2(normal.0);
                
                // Edge case: if the pedestrian is on the line, don't do anything
                if dist == 0.0 {
                    return;
                }
                
                // Check for collision
                if dist < PEDESTRIAN_RADIUS {
                    // Pedestrian needs to be nudged away from the wall by some multiple (k) of the normal vector
                    let k = PEDESTRIAN_RADIUS/dist - 1.0;
                    
                    // Move the pedestrian away from the wall
                    self.x += normal.0 * k;
                    self.y += normal.1 * k;
                    
                    
                    // The angle the pedestrian should be facing to reach their destination (between 0 and 2π)
                    let target_angle = (self.environment.end_positions[self.group][self.target_location].1 - self.y).atan2(self.environment.end_positions[self.group][self.target_location].0 - self.x);
                    
                    // Find the difference between the direction of travel and the target direction
                    let direction_difference = (target_angle - self.facing_direction + TAU + TAU) % TAU;
                    if direction_difference > PI/2.0 && direction_difference < 3.0*PI/2.0 {
                        // Facing away from target
                        // Need to face away from wall
                        self.facing_direction = normal_angle;
                    } else {
                        // Facing toward target
                        // Nudge the direction of travel away from the wall
                        self.facing_direction = nudge_angle(self.facing_direction, normal_angle, time_scale);
                    }
                    
                }
                
                // If the wall is within the pedestrian's personal space radius, nudge the direction vector away slightly
                if dist < PEDESTRIAN_PSPACE_RADIUS {
                    
                    // Nudge the direction of travel away from the wall
                    self.facing_direction = nudge_angle(self.facing_direction, normal_angle, WALL_REPULSION*time_scale);
                    
                }
                
            }
            
        }
        
        /// Return destination coordinates
        pub fn get_dest_coords(&self) -> (f64, f64) {
            return self.environment.end_positions[self.group][self.target_location];
        }
        
        /// Check for collisions with timing boundaries, and log the time taken to travel between two of them
        fn check_timing_boundaries(&mut self, time_scale: f64) {
            
            // Increment time elapsed
            if self.timing_boundary_elapsed.is_some() {
                self.timing_boundary_elapsed = Some(self.timing_boundary_elapsed.unwrap() + time_scale);
            }
            
            let mut touched_boundary_count = 0;
            
            // Check each timing boundary
            for (i, wall) in self.environment.timing_boundaries.iter().enumerate() {
                
                if !self.timing_boundary_states[i] && wall.get_normal_vector((self.x, self.y)).0 <= PEDESTRIAN_RADIUS {
                    self.timing_boundary_states[i] = true;
                    if self.timing_boundary_elapsed.is_none() {
                        self.timing_boundary_elapsed = Some(0.0);
                    }
                    touched_boundary_count += 1;
                } else if self.timing_boundary_states[i] {
                    touched_boundary_count += 1;
                }
                
            }
            
            if self.timing_boundary_elapsed.is_some() && touched_boundary_count == 2 {
                println!("Time: {}s, Group: {}", (self.timing_boundary_elapsed.unwrap()*100.0).round()/100.0, self.group);
                self.timing_boundary_elapsed = None;
            }
            
        }
        
        /// Draw this pedestrian with RayLib
        pub fn draw(&self, rl_handle: &mut RaylibDrawHandle, offset: (i32, i32)) {
            
            // Look-ahead zone
            rl_handle.draw_circle_sector(
                Vector2::new(offset.0 as f32 + (DRAW_SCALE as f32)*(self.x as f32), offset.1 as f32 + (DRAW_SCALE as f32)*(self.y as f32)),
                (DRAW_SCALE as f32) * (PEDESTRIAN_LOOK_AHEAD_RADIUS as f32),
                ((PI/2.0 - self.facing_direction + PEDESTRIAN_LOOK_AHEAD_FOV/2.0)/TAU*360.0) as f32,
                ((PI/2.0 - self.facing_direction - PEDESTRIAN_LOOK_AHEAD_FOV/2.0)/TAU*360.0) as f32,
                10,
                Color::fade(&Color::from_hex("808080").unwrap(), 0.2)
            );
            
            // Look-beside zone
            rl_handle.draw_circle_sector(
                Vector2::new(offset.0 as f32 + (DRAW_SCALE as f32)*(self.x as f32), offset.1 as f32 + (DRAW_SCALE as f32)*(self.y as f32)),
                (DRAW_SCALE as f32) * (PEDESTRIAN_LOOK_BESIDE_RADIUS as f32),
                ((PI/2.0 - self.facing_direction + PEDESTRIAN_LOOK_AHEAD_FOV/2.0 + PEDESTRIAN_LOOK_BESIDE_FOV)/TAU*360.0) as f32,
                ((PI/2.0 - self.facing_direction + PEDESTRIAN_LOOK_AHEAD_FOV/2.0)/TAU*360.0) as f32,
                10,
                Color::fade(&Color::from_hex("808080").unwrap(), 0.2)
            );
            rl_handle.draw_circle_sector(
                Vector2::new(offset.0 as f32 + (DRAW_SCALE as f32)*(self.x as f32), offset.1 as f32 + (DRAW_SCALE as f32)*(self.y as f32)),
                (DRAW_SCALE as f32) * (PEDESTRIAN_LOOK_BESIDE_RADIUS as f32),
                ((PI/2.0 - self.facing_direction - PEDESTRIAN_LOOK_AHEAD_FOV/2.0)/TAU*360.0) as f32,
                ((PI/2.0 - self.facing_direction - PEDESTRIAN_LOOK_AHEAD_FOV/2.0 - PEDESTRIAN_LOOK_BESIDE_FOV)/TAU*360.0) as f32,
                10,
                Color::fade(&Color::from_hex("808080").unwrap(), 0.2)
            );
            
            // Personal space
            rl_handle.draw_ellipse(
                offset.0 + ((DRAW_SCALE as f64)*self.x) as i32,
                offset.1 + ((DRAW_SCALE as f64)*self.y) as i32,
                (DRAW_SCALE as f32) * (PEDESTRIAN_PSPACE_RADIUS as f32),
                (DRAW_SCALE as f32) * (PEDESTRIAN_PSPACE_RADIUS as f32),
                Color::fade(&Color::from_hex("808080").unwrap(), 0.2)
            );
            
            // Collision hitbox
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
    
    /// Given an input angle and a target angle, move the input angle so that it is closer to the target angle
    /// 
    /// * `initial_angle` - Angle in radians, between 0 and 2π
    /// * `target_angle` - Angle in radians, between -π and 2π
    /// * `nudge_ratio` - A multiplier for how much the angle is changed (change in angle = diff(target_angle, initial_angle) * nudge_ratio)
    fn nudge_angle(initial_angle: f64, target_angle: f64, nudge_ratio: f64) -> f64 {
        
        // The difference between the initial and target angles
        let mut angle_diff = initial_angle - target_angle;
        
        // Constrain angle_diff between -π and π
        angle_diff = (angle_diff + TAU + PI) % TAU - PI;
        
        // Return the new angle
        return (initial_angle - angle_diff*nudge_ratio + TAU) % TAU;
    }
    
}
