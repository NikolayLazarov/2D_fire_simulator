mod Fluid;
//maybe move to Fluid.rs
static N: u32 = 256;
static iter: u32 = 10;

fn settings() {
    // dimensions
    // size(N, N);
}

// loop that does the things
fn setup() {
    let fluid = Fluid::FluidCube::new(0.1, 0., 0.);
}

//not sure but maybe loop that draws
// void draw(){

// }

fn main() {
    println!("Hi",);
}
