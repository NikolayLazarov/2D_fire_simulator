// use crate::{iter, N};

use bevy::ecs::system::EntityCommands;
use bevy::prelude::Component;
use bevy::prelude::*;

pub static N: u32 = 25 * 4;
static iter: u32 = 16;

pub fn IX(mut x: u32, mut y: u32) -> u32 {
    if x < 0 {
        x = 0;
    }
    if x > N - 1 {
        x = N - 1;
    }

    if y < 0 {
        y = 0;
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

    Vx: Vec<f32>,
    Vy: Vec<f32>,

    Vx0: Vec<f32>,
    Vy0: Vec<f32>,
    pub fire_x: u32,
    pub fire_y: u32,
    pub amount: f32,
    pub amount_x: f32,
    pub amount_y: f32,

    pub frames: u32,

    pub fire_size: u32,
    pub fire_range: u32,

    pub counter_range: u32,

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

            Vx: vec![0.; N.pow(2) as usize],
            Vy: vec![0.; N.pow(2) as usize],

            Vx0: vec![0.; N.pow(2) as usize],
            Vy0: vec![0.; N.pow(2) as usize],

            fire_x: 5,
            fire_y: 5,

            amount: 25.0,
            amount_x: 15.0,
            amount_y: 15.0,

            frames: 20,
            fire_size: 2,
            fire_range: 5,
            counter_range: 1,

            materials_coords: vec![],
        }
    }

    pub fn step(&mut self) {
        let visc: f32 = self.viscosity;
        let diff: f32 = self.diffusion;
        let delta_time: f32 = self.delta_time;
        let Vx: &mut Vec<f32> = &mut self.Vx;
        let Vy: &mut Vec<f32> = &mut self.Vy;
        let Vx0: &mut Vec<f32> = &mut self.Vx0;
        let Vy0: &mut Vec<f32> = &mut self.Vy0;
        let old_density: &mut Vec<f32> = &mut self.old_density;
        let density: &mut Vec<f32> = &mut self.density;

        diffuse(1, Vx0, Vx, visc, delta_time, &self.materials_coords);
        diffuse(2, Vy0, Vy, visc, delta_time, &self.materials_coords);

        project(Vx0, Vy0, Vx, Vy, &self.materials_coords);

        advect(1, Vx, Vx0, Vx0, Vy0, delta_time, &self.materials_coords);
        advect(2, Vy, Vy0, Vx0, Vy0, delta_time, &self.materials_coords);

        project(Vx, Vy, Vx0, Vy0, &self.materials_coords);

        diffuse(
            0,
            old_density,
            density,
            diff,
            delta_time,
            &self.materials_coords,
        );
        advect(
            0,
            density,
            old_density,
            Vx,
            Vy,
            delta_time,
            &self.materials_coords,
        );
    }

    pub fn add_density(&mut self, x: u32, y: u32, amount: f32) {
        //here
        let index: u32 = IX(x, y);
        self.density[index as usize] = amount;
    }

    pub fn add_velocity(&mut self, x: u32, y: u32, amountX: f32, amountY: f32) {
        let index: u32 = IX(x, y);
        self.Vx[index as usize] = amountX;
        self.Vy[index as usize] = amountY;
    }

    pub fn get_density(&mut self) -> &Vec<f32> {
        &self.density
    }
}

fn diffuse(
    b: u32,
    mut x: &mut Vec<f32>,
    mut x0: &mut Vec<f32>,
    diffusion: f32,
    delta_time: f32,
    materials_cords: &Vec<(u32, u32)>,
) {
    let a: f32 = delta_time * diffusion * ((N - 2) * (N - 2)) as f32;
    lin_solve(b, x, x0, a, (1 as f32) + (4 as f32) * a, materials_cords);
}

fn lin_solve(
    b: u32,
    mut x: &mut Vec<f32>,
    mut x0: &mut Vec<f32>,
    a: f32,
    c: f32,
    materials_cords: &Vec<(u32, u32)>,
) {
    let cRecip: f32 = 1.0 / c;
    for k in (0..iter) {
        for j in (1..N - 1) {
            for i in (1..N - 1) {
                let mut material_flag = false;

                for material in materials_cords {
                    if material.0 == i && material.1 == j {
                        material_flag = true;
                        break;
                    }
                }

                if material_flag {
                    x[IX(i, j) as usize] = 0.;
                    continue;
                }
                x[IX(i, j) as usize] = (x0[IX(i, j) as usize]
                    + a * (x[IX(i + 1, j) as usize]
                        + x[IX(i - 1, j) as usize]
                        + x[IX(i, j + 1) as usize]
                        + x[IX(i, j - 1) as usize]))
                    * cRecip;
            }
        }
    }

    set_bnd(b, x);
}

fn project(
    mut velocX: &mut Vec<f32>,
    mut velocY: &mut Vec<f32>,
    mut p: &mut Vec<f32>,
    mut div: &mut Vec<f32>,
    materials_cords: &Vec<(u32, u32)>,
) {
    for j in (1..N - 1) {
        for i in (1..N - 1) {
            let mut material_flag = false;
            for material in materials_cords {
                if material.0 == i && material.1 == j {
                    material_flag = true;
                    break;
                }
            }
            p[IX(i, j) as usize] = 0.;

            if material_flag {
                div[IX(i, j) as usize] = 0.;
                continue;
            }

            div[IX(i, j) as usize] = -0.5
                * (velocX[IX(i + 1, j) as usize] - velocX[IX(i - 1, j) as usize]
                    + velocY[IX(i, j + 1) as usize]
                    - velocY[IX(i, j - 1) as usize]) as f32
                / N as f32;
        }
    }

    set_bnd(0, div);
    set_bnd(0, p);
    lin_solve(0, p, div, 1.0, 4.0, materials_cords);

    for j in 1..N - 1 {
        for i in 1..N - 1 {
            let mut material_flag = false;
            for material in materials_cords {
                if material.0 == i && material.1 == j {
                    material_flag = true;
                    break;
                }
            }
            if material_flag {
                velocX[IX(i, j) as usize] = 0.;
                velocY[IX(i, j) as usize] = 0.;
                continue;
            }

            velocX[IX(i, j) as usize] -=
                0.5 * (p[IX(i + 1, j) as usize] - p[IX(i - 1, j) as usize]) as f32 * N as f32;
            velocY[IX(i, j) as usize] -=
                0.5 * (p[IX(i, j + 1) as usize] - p[IX(i, j - 1) as usize]) as f32 * N as f32;
        }
    }

    set_bnd(1, velocX);
    set_bnd(2, velocY);
}

fn advect(
    b: u32,
    d: &mut Vec<f32>,
    d0: &Vec<f32>,
    velocX: &Vec<f32>,
    velocY: &Vec<f32>,
    dt: f32,
    materials_cords: &Vec<(u32, u32)>,
) {
    let [mut i0, mut i1, mut j0, mut j1] = [0.; 4];

    let dtx: f32 = dt * (N - 2) as f32;
    let dty: f32 = dt * (N - 2) as f32;

    let [mut s0, mut s1, mut t0, mut t1] = [0.; 4];
    let [mut tmp1, mut tmp2, mut x, mut y] = [0.; 4];

    let N_float: f32 = N as f32;

    let [mut ifloat, mut jfloat] = [1.; 2];

    for j in (1..N - 1) {
        for i in (1..N - 1) {
            let mut material_flag = false;
            for material in materials_cords {
                if material.0 == i && material.1 == j {
                    material_flag = true;
                    break;
                }
            }
            if material_flag {
                d[IX(i, j) as usize] = 0.;
                continue;
            }
            tmp1 = dtx * velocX[IX(i, j) as usize];
            tmp2 = dty * velocY[IX(i, j) as usize];
            x = ifloat - tmp1;
            y = jfloat - tmp2;

            if (x < 0.5) {
                x = 0.5;
            }
            if (x > N_float + 0.5) {
                x = N_float + 0.5
            };
            i0 = x.floor();
            i1 = i0 + 1.;
            if (y < 0.5) {
                y = 0.5;
            }
            if (y > N_float + 0.5) {
                y = N_float + 0.5;
            }
            j0 = y.floor();
            j1 = j0 + 1.;

            s1 = x - i0;
            s0 = 1. - s1;
            t1 = y - j0;
            t0 = 1. - t1;

            let i0i: u32 = i0 as u32;
            let i1i: u32 = i1 as u32;
            let j0i: u32 = j0 as u32;
            let j1i: u32 = j1 as u32;

            d[IX(i, j) as usize] = s0 * (t0 * d0[IX(i0i, j0i) as usize])
                + (t1 * d0[IX(i0i, j1i) as usize])
                + s1 * (t0 * d0[IX(i1i, j0i) as usize])
                + (t1 * d0[IX(i1i, j1i) as usize]);

            if d[IX(i, j) as usize] > 1. {
                d[IX(i, j) as usize] = 1.;
                // println!("cords = {},{} value = {}", i,j, d[IX(i,j)as usize]);
            }
            ifloat += 1.;
        }
        ifloat = 1.;
        jfloat += 1.;
    }

    set_bnd(b, d);
}
//keeps the sumilation from overflowing
fn set_bnd(b: u32, mut x: &mut Vec<f32>) {
    for i in 1..N - 1 {
        x[IX(i, 0) as usize] = if b == 2 {
            -x[IX(i, 1) as usize]
        } else {
            x[IX(i, 1) as usize]
        };

        x[IX(i, N - 1) as usize] = if b == 2 {
            -x[IX(i, N - 2) as usize]
        } else {
            x[IX(i, N - 2) as usize]
        };
    }

    for j in 1..N - 1 {
        x[IX(0, j) as usize] = if b == 1 {
            -x[IX(1, j) as usize]
        } else {
            x[IX(1, j) as usize]
        };
        x[IX(N - 1, j) as usize] = if b == 1 {
            -x[IX(N - 2, j) as usize]
        } else {
            x[IX(N - 2, j) as usize]
        };
    }
    x[IX(0, 0) as usize] = 0.5 * (x[IX(1, 0) as usize] + x[IX(0, 1) as usize]);

    x[IX(0, N - 1) as usize] = 0.5 * (x[IX(1, N - 1) as usize] + x[IX(0, N - 2) as usize]);

    x[IX(N - 1, 0) as usize] = 0.5 * (x[IX(N - 2, 0) as usize] + x[IX(N - 1, 1) as usize]);

    x[IX(N - 1, N - 1) as usize] =
        0.5 * (x[IX(N - 2, N - 1) as usize] + x[IX(N - 1, N - 2) as usize]);
}
