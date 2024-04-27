pub mod simulator {
    
    //use crate::simulation::pedestrian::pedestrian;
    use crate::simulation::pedestrian::pedestrian::walker as ped;
    
    pub struct crowd_sim {
        area: sim_area,
        pedestrians: Vec<ped>
    }
    
    pub struct sim_area {
        boundaries: Vec<wall>,
    }
    
    pub struct wall {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    }
    
    impl crowd_sim {
        pub fn simulate_timestep() {
            println!("Simulating one timestep...");
        }
    }
    
}
