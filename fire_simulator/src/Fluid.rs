use bevy::prelude::Component;
use bevy::prelude::*;

use crate::material_coords::{self, CoordsList, Coords};

// pub struct CoordsList(Vec<Coords>);

// pub struct Coords{
//     x: u32,
//     y: u32,
//     burned: bool,
// }

// impl CoordsList{
//     pub fn add_coords(&mut self) -> &Coords{
//         let new_coords = Coords{
//                 x: 5,
//                 y: 5,
//                 burned: false
//         };
//         self.0.push(new_coords); 
//         return &self.0[self.0.len()-1];

//     }
// }

pub static N: u32 = 25 * 4;
static ITER: u32 = 16;

pub fn ix(mut x: u32, mut y: u32) -> u32 {
    if x > N - 1 {
        x = N - 1;
    }

    if y > N - 1 {
        y = N - 1;
    }
    x + y * N
}

#[derive(Default, Component, Clone)]
pub struct FluidMatrix {
    pub delta_time: f32,
    pub diffusion: f32,
    pub viscosity: f32,
    old_density: Vec<f32>,
    density: Vec<f32>,

    vx: Vec<f32>,
    vy: Vec<f32>,

    vx0: Vec<f32>,
    vy0: Vec<f32>,
    pub fire_x: u32,
    pub fire_y: u32,
    pub amount: f32,
    pub amount_x: f32,
    pub amount_y: f32,

    pub frames: u32,

    pub materials_coords: Vec<(u32, u32)>,
}

impl FluidMatrix {
    pub fn new() -> Self {
        Self {
            delta_time: 0.1,
            diffusion: 0.001,
            viscosity: 0.0000001,

            old_density: vec![0.; N.pow(2) as usize],
            density: vec![0.; N.pow(2) as usize],

            vx: vec![0.; N.pow(2) as usize],
            vy: vec![0.; N.pow(2) as usize],

            vx0: vec![0.; N.pow(2) as usize],
            vy0: vec![0.; N.pow(2) as usize],

            fire_x: 5,
            fire_y: 5,

            amount: 25.0,
            amount_x: 15.0,
            amount_y: 15.0,

            frames: 20,
            materials_coords: vec![],
        }
    }

    pub fn step(&mut self, coords: &mut ResMut<CoordsList>) {
        let visc: f32 = self.viscosity;
        let diff: f32 = self.diffusion;
        let delta_time: f32 = self.delta_time;
        let vx: &mut Vec<f32> = &mut self.vx;
        let vy: &mut Vec<f32> = &mut self.vy;
        let vx0: &mut Vec<f32> = &mut self.vx0;
        let vy0: &mut Vec<f32> = &mut self.vy0;
        let old_density: &mut Vec<f32> = &mut self.old_density;
        let density: &mut Vec<f32> = &mut self.density;

        diffuse(1, vx0, vx, visc, delta_time, &self.materials_coords,coords);
        diffuse(2, vy0, vy, visc, delta_time, &self.materials_coords,coords);

        project(vx0, vy0, vx, vy, &self.materials_coords,coords);

        advect(1, vx, vx0, vx0, vy0, delta_time, &self.materials_coords,coords);
        advect(2, vy, vy0, vx0, vy0, delta_time, &self.materials_coords,coords);

        project(vx, vy, vx0, vy0, &self.materials_coords,coords);

        diffuse(
            0,
            old_density,
            density,
            diff,
            delta_time,
            &self.materials_coords,coords,
        );
        advect(
            0,
            density,
            old_density,
            vx,
            vy,
            delta_time,
            &self.materials_coords,coords,
        );
    }

    pub fn add_density(&mut self, x: u32, y: u32, amount: f32) {
        let index: u32 = ix(x, y);
        self.density[index as usize] = amount;
    }

    pub fn add_velocity(&mut self, x: u32, y: u32, amount_x: f32, amount_y: f32) {
        let index: u32 = ix(x, y);
        self.vx[index as usize] = amount_x;
        self.vy[index as usize] = amount_y;
    }

    pub fn get_density(&mut self) -> &Vec<f32> {
        &self.density
    }
}

fn diffuse(
    b: u32,
    x: &mut Vec<f32>,
    x0: &mut Vec<f32>,
    diffusion: f32,
    delta_time: f32,
    materials_cords: &Vec<(u32, u32)>,
    coords: &mut ResMut<CoordsList>
) {
    let a: f32 = delta_time * diffusion * ((N - 2) * (N - 2)) as f32;
    lin_solve(b, x, x0, a, (1 as f32) + (4 as f32) * a, materials_cords,coords);
}

fn lin_solve(
    b: u32,
    x: &mut Vec<f32>,
    x0: &mut Vec<f32>,
    a: f32,
    c: f32,
    materials_cords: &Vec<(u32, u32)>,
    coords: &mut ResMut<CoordsList>
) {
    let c_recip: f32 = 1.0 / c;
    for _k in 0..ITER {
        for j in 1..N - 1 {
            for i in 1..N - 1 {
                let mut material_flag = false;

                for material in coords.material_coords.iter(){
                    if material.x == i && material.y == j {
                        material_flag = true;
                        break;
                    }
                }

                // for material in materials_cords {
                //     if material.0 == i && material.1 == j {
                //         material_flag = true;
                //         break;
                //     }
                // }

                if material_flag {
                    x[ix(i, j) as usize] = 0.;
                    continue;
                }
                x[ix(i, j) as usize] = (x0[ix(i, j) as usize]
                    + a * (x[ix(i + 1, j) as usize]
                        + x[ix(i - 1, j) as usize]
                        + x[ix(i, j + 1) as usize]
                        + x[ix(i, j - 1) as usize]))
                    * c_recip;
            }
        }
    }

    set_bnd(b, x);
}

fn project(
    veloc_x: &mut Vec<f32>,
    veloc_y: &mut Vec<f32>,
    p: &mut Vec<f32>,
    div: &mut Vec<f32>,
    materials_cords: &Vec<(u32, u32)>,
    coords: &mut ResMut<CoordsList>
) {
    for j in 1..N - 1 {
        for i in 1..N - 1 {
            let mut material_flag = false;
            
            for material in coords.material_coords.iter() {
                if material.x == i && material.y == j {
                    material_flag = true;
                    break;
                }
            }
            
            // for material in materials_cords {
            //     if material.0 == i && material.1 == j {
            //         material_flag = true;
            //         break;
            //     }
            // }
            p[ix(i, j) as usize] = 0.;

            if material_flag {
                div[ix(i, j) as usize] = 0.;
                continue;
            }

            div[ix(i, j) as usize] = -0.5
                * (veloc_x[ix(i + 1, j) as usize] - veloc_x[ix(i - 1, j) as usize]
                    + veloc_y[ix(i, j + 1) as usize]
                    - veloc_y[ix(i, j - 1) as usize]) as f32
                / N as f32;
        }
    }
    // println!("here = {:?}",coords);

    set_bnd(0, div);
    set_bnd(0, p);
    lin_solve(0, p, div, 1.0, 4.0, materials_cords,coords);

    for j in 1..N - 1 {
        for i in 1..N - 1 {
            let mut material_flag = false;
            
            for material in coords.material_coords.iter() {
                if material.x == i && material.y == j {
                    material_flag = true;
                    break;
                }
            }

            // for material in materials_cords {
                
            //     if material.0 == i && material.1 == j {
            //         material_flag = true;
            //         break;
            //     }
            // }
            if material_flag {
                veloc_x[ix(i, j) as usize] = 0.;
                veloc_y[ix(i, j) as usize] = 0.;
                continue;
            }

            veloc_x[ix(i, j) as usize] -=
                0.5 * (p[ix(i + 1, j) as usize] - p[ix(i - 1, j) as usize]) as f32 * N as f32;
            veloc_y[ix(i, j) as usize] -=
                0.5 * (p[ix(i, j + 1) as usize] - p[ix(i, j - 1) as usize]) as f32 * N as f32;
        }
    }

    set_bnd(1, veloc_x);
    set_bnd(2, veloc_y);
}

fn advect(
    b: u32,
    d: &mut Vec<f32>,
    d0: &Vec<f32>,
    veloc_x: &Vec<f32>,
    veloc_y: &Vec<f32>,
    dt: f32,
    materials_cords: &Vec<(u32, u32)>,
    coords: &mut ResMut<CoordsList>
) {
    let dtx: f32 = dt * (N - 2) as f32;
    let dty: f32 = dt * (N - 2) as f32;

    let n_float: f32 = N as f32;

    let [mut ifloat, mut jfloat] = [1.; 2];

    for j in 1..N - 1 {
        for i in 1..N - 1 {
            let mut material_flag = false;
            
            for material in coords.material_coords.iter() {
                if material.x == i && material.y == j {
                    material_flag = true;
                    break;
                }
            }
            
            // for material in materials_cords {
            //     if material.0 == i && material.1 == j {
            //         material_flag = true;
            //         break;
            //     }
            // }
            if material_flag {
                d[ix(i, j) as usize] = 0.;
                continue;
            }
            let tmp1 = dtx * veloc_x[ix(i, j) as usize];
            let tmp2 = dty * veloc_y[ix(i, j) as usize];
            let mut x = ifloat - tmp1;
            let mut y = jfloat - tmp2;

            if x < 0.5 {
                x = 0.5;
            }
            if x > n_float + 0.5 {
                x = n_float + 0.5
            };
            let i0 = x.floor();
            let i1 = i0 + 1.;
            if y < 0.5 {
                y = 0.5;
            }
            if y > n_float + 0.5 {
                y = n_float + 0.5;
            }
            let j0 = y.floor();
            let j1 = j0 + 1.;

            let s1 = x - i0;
            let s0 = 1. - s1;
            let t1 = y - j0;
            let t0 = 1. - t1;

            let i0i: u32 = i0 as u32;
            let i1i: u32 = i1 as u32;
            let j0i: u32 = j0 as u32;
            let j1i: u32 = j1 as u32;

            d[ix(i, j) as usize] = s0 * (t0 * d0[ix(i0i, j0i) as usize])
                + (t1 * d0[ix(i0i, j1i) as usize])
                + s1 * (t0 * d0[ix(i1i, j0i) as usize])
                + (t1 * d0[ix(i1i, j1i) as usize]);

            if d[ix(i, j) as usize] > 1. {
                d[ix(i, j) as usize] = 1.;
            }
            ifloat += 1.;
        }
        ifloat = 1.;
        jfloat += 1.;
    }

    set_bnd(b, d);
}
fn set_bnd(b: u32, x: &mut Vec<f32>) {
    for i in 1..N - 1 {
        x[ix(i, 0) as usize] = if b == 2 {
            -x[ix(i, 1) as usize]
        } else {
            x[ix(i, 1) as usize]
        };

        x[ix(i, N - 1) as usize] = if b == 2 {
            -x[ix(i, N - 2) as usize]
        } else {
            x[ix(i, N - 2) as usize]
        };
    }

    for j in 1..N - 1 {
        x[ix(0, j) as usize] = if b == 1 {
            -x[ix(1, j) as usize]
        } else {
            x[ix(1, j) as usize]
        };
        x[ix(N - 1, j) as usize] = if b == 1 {
            -x[ix(N - 2, j) as usize]
        } else {
            x[ix(N - 2, j) as usize]
        };
    }
    x[ix(0, 0) as usize] = 0.5 * (x[ix(1, 0) as usize] + x[ix(0, 1) as usize]);

    x[ix(0, N - 1) as usize] = 0.5 * (x[ix(1, N - 1) as usize] + x[ix(0, N - 2) as usize]);

    x[ix(N - 1, 0) as usize] = 0.5 * (x[ix(N - 2, 0) as usize] + x[ix(N - 1, 1) as usize]);

    x[ix(N - 1, N - 1) as usize] =
        0.5 * (x[ix(N - 2, N - 1) as usize] + x[ix(N - 1, N - 2) as usize]);
}
