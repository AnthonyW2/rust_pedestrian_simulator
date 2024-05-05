pub mod simulator {
    
    use std::sync::Arc;
    use raylib::{drawing::{RaylibDrawHandle, RaylibDraw}, color::Color};
    use rand::{thread_rng, seq::SliceRandom};
    
    use crate::simulation::pedestrian::pedestrian;
    //use crate::simulation::pedestrian::pedestrian::Walker;
    
    
    /// The distance from a target location that a pedestrian needs to be to qualify as having reached it
    pub const TARGET_LOCATION_RADIUS: f64 = 1.0;
    
    /// How many pixels in a metre
    pub const DRAW_SCALE: i32 = 50;
    
    /// Contains all information related to a crowd simulation
    pub struct CrowdSim {
        /// The 2D space where the simulation takes place
        area: Arc<SimArea>,
        /// The amount of time simulated, in seconds
        pub time_elapsed: f64,
        /// All the walkers contained in the simulation
        available_pedestrians: Vec<pedestrian::Walker>,
        /// All the walkers currently walking
        active_pedestrians: Vec<pedestrian::Walker>,
        /// All the walkers that have reached their destinations
        finished_pedestrians: Vec<pedestrian::Walker>,
        /// The number of pedestrians added to the simulation per second
        pedestrian_add_rate: f64
    }
    
    /// Describes a 2 dimensional environment where a simulation takes place
    pub struct SimArea {
        pub boundaries: Vec<Wall>,
        pub start_positions: Vec<Vec<(f64, f64)>>,
        pub end_positions: Vec<Vec<(f64, f64)>>
    }
    
    /// Describes an impassable linear barrier with a start and end point
    pub struct Wall {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    }
    
    impl CrowdSim {
        /// Create a new CrowdSim object.
        /// 
        /// * `area` - A `SimArea` object describing the space for the simulation to be set in.
        /// * `pedestrian_add_rate` - The number of pedestrians added to the simulation per second.
        pub fn new(area: Arc<SimArea>, pedestrian_add_rate: f64) -> CrowdSim {
            CrowdSim {
                area,
                time_elapsed: 0.0,
                available_pedestrians: Vec::new(),
                active_pedestrians: Vec::new(),
                finished_pedestrians: Vec::new(),
                pedestrian_add_rate
            }
        }
        
        /// Randomise the order of the pedestrians
        pub fn randomise_pedestrian_order(&mut self) {
            self.available_pedestrians.shuffle(&mut thread_rng());
        }
        
        /// Simulate a small period of time in a single step.
        /// 
        /// * `time_scale` - The amount of time (in seconds) that passes during each timestep
        pub fn simulate_timestep(&mut self, time_scale: f64) {
            //println!("Simulating one timestep...");
            
            self.update_active();
            
            // Collect the position and facing direction of every pedestrian to pass to Walker.simulate_timestep(), so that a pedestrian can see its neighbours.
            // This is an ugly way to do this, but I don't have time to implement a "nice" way right now.
            // (x, y, direction)
            let pedestrian_positions = self.active_pedestrians.iter().map(|ped| (ped.x, ped.y, ped.facing_direction)).collect::<Vec<_>>();
            
            for (i, ped) in self.active_pedestrians.iter_mut().enumerate() {
                ped.simulate_timestep(time_scale, &pedestrian_positions[0..i], &pedestrian_positions[i+1..]);
            }
            
            self.time_elapsed += time_scale;
            
            self.update_finished();
            
        }
        
        /// Add pedestrians to the simulation, with behaviours that are chosen with weighted random choices
        pub fn add_pedestrian_set(&mut self, number: usize, group: usize, behaviour: usize) {
            
            for _ in 0..number {
                
            }
            
        }
        
        /// Add a new pedestrian to the simulation
        pub fn add_pedestrian(&mut self, group: usize, start: usize, end: usize, target_speed: f64, etiquette: pedestrian::Etiquette) {
            self.available_pedestrians.push(
                pedestrian::Walker::new(self.area.clone(), group, start, end, target_speed, etiquette)
            );
        }
        
        /// Make some number of pedestrians active, depending on pedestrian_add_rate
        fn update_active(&mut self) {
            // Debug code to randomly add finished pedestrians back into the available pool
            //if self.finished_pedestrians.len() > 0 && rand::random::<f64>() > 0.5 {
            //    let mut new_ped = self.finished_pedestrians.pop().unwrap();
            //    new_ped.x = 0.0;
            //    new_ped.y = rand::random::<f64>()*6.0+1.0;
            //    self.available_pedestrians.push(new_ped);
            //}
            
            while self.available_pedestrians.len() > 0 && self.time_elapsed > ((self.active_pedestrians.len() + self.finished_pedestrians.len()) as f64) / self.pedestrian_add_rate {
                self.active_pedestrians.push(self.available_pedestrians.pop().unwrap());
            }
        }
        
        /// Check all active pedestrians and remove any that have reached their destinations
        fn update_finished(&mut self) {
            let mut i = 0;
            while i < self.active_pedestrians.len() {
                let ped = &self.active_pedestrians[i];
                let dest = ped.get_dest_coords();
                if ((ped.x - dest.0)*(ped.x - dest.0) + (ped.y - dest.1)*(ped.y - dest.1)).sqrt() < TARGET_LOCATION_RADIUS {
                    self.finished_pedestrians.push( self.active_pedestrians.remove(i) );
                } else {
                    i += 1;
                }
            }
        }
        
        /// Draw this simulation with RayLib
        /// 
        /// * `rl_handle` - The RaylibDrawHandle used to draw the objects
        /// * `offset` - The x and y offset of this object, in pixels
        pub fn draw(&self, rl_handle: &mut RaylibDrawHandle, offset: (i32, i32)) {
            
            self.area.draw(rl_handle, offset);
            
            for ped in &self.active_pedestrians {
                ped.draw(rl_handle, offset);
            }
            
        }
        
    }
    
    impl SimArea {
        pub fn new() -> SimArea {
            SimArea {
                boundaries: Vec::new(),
                start_positions: Vec::new(),
                end_positions: Vec::new()
            }
        }
        
        pub fn add_wall(&mut self, point1: (f64, f64), point2: (f64, f64)) {
            self.boundaries.push(
                Wall::new(point1.0, point1.1, point2.0, point2.1)
            );
        }
        
        pub fn add_start_end_group(&mut self, starts: Vec<(f64, f64)>, ends: Vec<(f64, f64)>) {
            self.start_positions.push(starts);
            self.end_positions.push(ends);
        }
        
        /// Draw this environment with RayLib
        pub fn draw(&self, rl_handle: &mut RaylibDrawHandle, offset: (i32, i32)) {
            
            // Add metre gridlines
            let max_x = self.boundaries.iter().map(|wall| wall.x1.max(wall.x2) as i32).max().unwrap();
            let max_y = self.boundaries.iter().map(|wall| wall.y1.max(wall.y2) as i32).max().unwrap();
            for x in 0..max_x {
                rl_handle.draw_line(
                    offset.0 + DRAW_SCALE*x,
                    offset.1,
                    offset.0 + DRAW_SCALE*x,
                    offset.1 + DRAW_SCALE*max_y,
                    Color::from_hex("b0b0b0").unwrap()
                );
            }
            for y in 0..max_y {
                rl_handle.draw_line(
                    offset.0,
                    offset.1 + DRAW_SCALE*y,
                    offset.0 + DRAW_SCALE*max_x,
                    offset.1 + DRAW_SCALE*y,
                    Color::from_hex("b0b0b0").unwrap()
                );
            }
            
            
            // Draw the walls
            for wall in &self.boundaries {
                wall.draw(rl_handle, offset);
            }
            
            // Draw the start & end points
            for (x,y) in (&self.start_positions).iter().flatten() {
                rl_handle.draw_ellipse(
                    offset.0 + DRAW_SCALE*(*x as i32),
                    offset.1 + DRAW_SCALE*(*y as i32),
                    10.0,
                    10.0,
                    Color::from_hex("3cbd00").unwrap()
                );
            }
            for (x,y) in (&self.end_positions).iter().flatten() {
                rl_handle.draw_ellipse(
                    offset.0 + DRAW_SCALE*(*x as i32),
                    offset.1 + DRAW_SCALE*(*y as i32),
                    10.0,
                    10.0,
                    Color::from_hex("005de9").unwrap()
                );
            }
            
        }
        
    }
    
    impl Wall {
        pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Wall {
            Wall {
                x1, x2, y1, y2
            }
        }
        
        /// Given a point P, determine the vector that points from the closest point on the line to P
        /// 
        /// Output form: (distance, (normal x, normal y))
        pub fn get_normal_vector(&self, p: (f64, f64)) -> (f64, (f64, f64)) {
            // Define some useful vector functions
            fn vec_dot(v1: (f64, f64), v2: (f64, f64)) -> f64 { v1.0*v2.0 + v1.1*v2.1 }
            fn vec_add(v1: (f64, f64), v2: (f64, f64)) -> (f64, f64) { (v1.0 + v2.0, v1.1 + v2.1) }
            fn vec_sub(v1: (f64, f64), v2: (f64, f64)) -> (f64, f64) { (v1.0 - v2.0, v1.1 - v2.1) }
            fn vec_mul(v: (f64, f64), k: f64) -> (f64, f64) { (v.0 * k, v.1 * k) }
            /// Find the square of the distance between two points P1 and P2
            fn vec_dist_sq(p1: (f64, f64), p2: (f64, f64)) -> f64 {
                (p2.0 - p1.0)*(p2.0 - p1.0) + (p2.1 - p1.1)*(p2.1 - p1.1)
            }
            
            // A denotes the first point (x1,y1) of the line
            let a = (self.x1, self.y1);
            // B denotes the second point (x2,y2) of the line
            let b = (self.x2, self.y2);
            
            let ap = vec_sub(p,a);
            let ab = vec_sub(b,a);
            
            // Scalar projection of AP onto AB
            let scalar_proj_ap_onto_ab = vec_dot(ap,ab)/vec_dot(ab,ab);
            
            // D is the point on the line AB closest to P
            let d = vec_add(a, vec_mul(ab, scalar_proj_ap_onto_ab));
            let ad = vec_sub(d,a);
            
            // Solve AD = λ * AB for λ
            let λ = if ab.0.abs() > ab.1.abs() {ad.0 / ab.0} else {ad.1 / ab.1};
            
            // Find closest point on the line to P
            let closest_point;
            if λ <= 0.0 {
                closest_point = a;
            } else if λ >= 1.0 {
                closest_point = b;
            } else {
                closest_point = d;
            }
            
            // Distance from closest_point to P
            let dist = vec_dist_sq(closest_point, p).sqrt();
            
            // Edge case: given point lies on the line
            if dist == 0.0 {
                return (0.0, (0.0,0.0));
            }
            
            // normal_vec is the vector that points from closest_point on the line to P
            let normal_vec = vec_sub(p, closest_point);
            
            return (dist, normal_vec);
            
        }
        
        /// Draw this wall with RayLib
        pub fn draw(&self, rl_handle: &mut RaylibDrawHandle, offset: (i32, i32)) {
            
            rl_handle.draw_line(
                offset.0 + DRAW_SCALE*(self.x1 as i32),
                offset.1 + DRAW_SCALE*(self.y1 as i32),
                offset.0 + DRAW_SCALE*(self.x2 as i32),
                offset.1 + DRAW_SCALE*(self.y2 as i32),
                Color::from_hex("000000").unwrap()
            );
            
        }
        
    }
    
}
