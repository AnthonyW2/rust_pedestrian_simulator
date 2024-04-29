pub mod simulator {
    
    //use crate::simulation::pedestrian::pedestrian;
    use crate::simulation::pedestrian::pedestrian::Walker;
    
    /// Contains all information related to a crowd simulation
    pub struct CrowdSim<'a> {
        /// The 2D space where the simulation takes place
        area: SimArea,
        /// All the walkers contained in the simulation
        pedestrians: Vec<Walker<'a>>
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
    
    impl CrowdSim<'_> {
        /// Create a new CrowdSim object.
        /// 
        /// * `area` - A `SimArea` object describing the space for the simulation to be set in.
        pub fn new(area: SimArea) -> CrowdSim<'static> {
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
    }
    
    impl SimArea {
        pub fn new() -> SimArea {
            SimArea {
                boundaries: Vec::new(),
                start_positions: Vec::new(),
                end_positions: Vec::new()
            }
        }
        
        pub fn add_pedestrian_group(&mut self, number: u32, behaviours: Vec<usize>, behaviour_probabilities: Vec<f64>) {
            
        }
    }
    
}
