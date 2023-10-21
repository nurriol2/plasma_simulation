pub mod simulation_loop{
    const NI: usize = 21; // Number of mesh nodes
    const QE: f64 = 1.602176565e-19; // (Coulombs), elementary charge
    const EPS0: f64 = 8.85418782e-12; // C/V/m, vacuum permitivity
    const ME: f64 = 9.10938215e-31; // kg, electron mass 

    pub fn run() {
        // Initialization
        let mut phi: Vec<f64> = vec![0.0; NI]; // Potential
        let mut rho: Vec<f64> = vec![QE; NI]; // Charge density
        let mut ef: Vec<f64> = Vec::with_capacity(NI); // Electric field
        
        for i in 0..NI{
            phi[i] = 0.0; 
            rho[i] = QE*1e12; 
            ef[i] = 0.0; 
        }
    }
}

fn main() {

}
