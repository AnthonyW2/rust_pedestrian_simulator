pub mod simulator {
    
    use std::sync::Arc;
    
    use crate::simulation::pedestrian::pedestrian;
    //use crate::simulation::pedestrian::pedestrian::Walker;
    
    /// Contains all information related to a crowd simulation
    pub struct CrowdSim {
        /// The 2D space where the simulation takes place
        area: Arc<SimArea>,
        /// All the walkers contained in the simulation
        pedestrians: Vec<pedestrian::Walker>
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
        pub fn new(area: Arc<SimArea>) -> CrowdSim {
            CrowdSim {
                area,
                pedestrians: Vec::new()
            }
        }
        
        /// Simulate a small period of time in a single step.
        /// 
        /// * `time_scale` - The amount of time (in seconds) that passes during each timestep
        pub fn simulate_timestep(&mut self, time_scale: f64) {
            println!("Simulating one timestep...");
            
            for ped in &mut self.pedestrians {
                ped.simulate_timestep(time_scale);
            }
        }
        
        /// Add pedestrians to the simulation, with behaviours that are chosen with weighted random choices
        pub fn add_pedestrian_group(&mut self, number: usize, behaviours: Vec<usize>, behaviour_weights: Vec<f64>) {
            
            for _ in 0..number {
                
            }
            
        }
        
        /// Add pedestrians to the simulation, with behaviours that are chosen with weighted random choices
        pub fn add_pedestrian(&mut self, group: usize, start: usize, end: usize, target_speed: f64) {
            self.pedestrians.push(
                pedestrian::Walker::new(self.area.clone(), group, start, end, target_speed)
            );
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
    }
    
    impl Wall {
        pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Wall {
            Wall {
                x1, x2, y1, y2
            }
        }
        
        /// Find a vector to nudge the pedestrian, which resolves the collision with this wall
        pub fn get_walker_collision_vector(&self, pedestrian: &pedestrian::Walker) -> (f64, f64){
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
            // P denotes the coordinates of the pedestrian
            let p = (pedestrian.x, pedestrian.y);
            
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
            
            // Edge case: if the pedestrian is on the line, don't do anything
            if dist == 0.0 {
                return (0.0,0.0);
            }
            
            if dist < pedestrian::PEDESTRIAN_RADIUS {
                // Pedestrian needs to be nudged away from the wall by some multiple (k) of away_vec
                
                // away_vec is the vector that points from closest_point to p
                let away_vec = vec_sub(p, closest_point);
                
                let k = pedestrian::PEDESTRIAN_RADIUS/dist - 1.0;
                
                return vec_mul(away_vec, k);
            }
            
            // No nudge required, return 0 vector
            return (0.0,0.0);
            
        }
    }
    
}
