use crate::{iter, N};

fn IX(x: u32, y: u32) -> u32 {
    return x + y * N;
}

pub struct FluidCube {
    size: u32,

    //timestep
    dt: f32,
    //diffusion_amount -> controls how the velocity and the vector diffuse
    diffuusion: f32,
    //viscosity -> thikcness
    viscosity: f32,
    //maybe previoeus density
    s: Vec<f32>,
    density: Vec<f32>,

    Vx: Vec<f32>,
    Vy: Vec<f32>,

    Vx0: Vec<f32>,
    Vy0: Vec<f32>,
}

impl FluidCube {
    pub fn new(dt_outside: f32, diff_outside: f32, visc_outside: f32) -> Self {
        Self {
            size: N,
            dt: dt_outside,
            diffuusion: diff_outside,
            viscosity: visc_outside,

            //not needed
            //    dt = dt;
            //    diff: diff_outside;
            //    visc: visc_outside;
            s: vec![0.; N.pow(2).try_into().unwrap()],
            density: vec![0.; N.pow(2).try_into().unwrap()],

            Vx: vec![0.; N.pow(2).try_into().unwrap()],
            Vy: vec![0.; N.pow(2).try_into().unwrap()],

            Vx0: vec![0.; N.pow(2).try_into().unwrap()],
            Vy0: vec![0.; N.pow(2).try_into().unwrap()],
        }
    }

    pub fn add_density(&mut self, x: u32, y: u32, amount: f32) {
        let index: u32 = IX(x, y);
        // self.density.get_mut(index as usize) amount   ;

        self.density[index as usize] = amount;
    }

    pub fn add_velocity(&mut self, x: u32, y: u32, amountX: f32, amountY: f32) {
        let index: u32 = IX(x, y);
        // self.density.get_mut(index as usize) amount   ;

        self.Vx[index as usize] = amountX;
        self.Vy[index as usize] = amountY;
    }
}

fn diffuse(b: u32, mut x: &mut Vec<f32>, mut x0: &mut Vec<f32>, diff: f32, dt: f32) {
    let a: f32 = dt * diff * ((N - 2) * (N - 2)) as f32;
    lin_solve(b, x, x0, a, (1 as f32) + (6 as f32) * a);
}

fn lin_solve(b: u32, mut x: &mut Vec<f32>, mut x0: &mut Vec<f32>, a: f32, c: f32) {
    let cRecip: f32 = 1.0 / c;

    for k in (0..iter) {
        for j in (1..N - 1) {
            for i in (1..N - 1) {
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
) {
    for j in (1..N - 1) {
        for i in (1..N - 1) {
            div[IX(i, j) as usize] = -0.5
                * (velocX[IX(i + 1, j) as usize] - velocX[IX(i - 1, j) as usize]
                    + velocY[IX(i, j + 1) as usize]
                    - velocY[IX(i, j - 1) as usize]) as f32
                / N as f32;
            p[IX(i, j) as usize] = 0.;
        }
    }

    set_bnd(0, div);
    set_bnd(0, p);
    lin_solve(0, p, div, 1.0, 6.0);

    for j in 1..N - 1 {
        for i in 1..N - 1 {
            velocX[IX(i, j) as usize] -=
                0.5 * (p[IX(i + 1, j) as usize] - p[IX(i - 1, j) as usize]) as f32 * N as f32;
            velocY[IX(i, j) as usize] -=
                0.5 * (p[IX(i, j + 1) as usize] - p[IX(i, j - 1) as usize]) as f32 * N as f32;
        }
    }

    set_bnd(1, velocX);
    set_bnd(2, velocY);
}

fn advect(b: u32, mut d: &mut Vec<f32>, d0: Vec<f32>, velocX: Vec<f32>, velocY: Vec<f32>, dt: f32) {
    let [mut i0, mut i1, mut j0, mut j1] = [0.; 4];

    let dtx: f32 = dt * (N - 2) as f32;
    let dty: f32 = dt * (N - 2) as f32;

    let [mut s0, mut s1, mut t0, mut t1] = [0.; 4];
    let [mut tmp1, mut tmp2, mut x, mut y] = [0.; 4];

    let N_float: f32 = N as f32;
    let [mut ifloat, mut jfloat] = [1.; 2];
    // let [i, j] = [None;2];

    for j in (1..N - 1) {
        for i in (1..N - 1) {
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

            ifloat += 1.;
        }
        jfloat += 1.;
    }

    set_bnd(b, d);
}

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
