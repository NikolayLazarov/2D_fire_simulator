use bevy::prelude::Resource;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Coords {
    pub x: u32,
    pub y: u32,
    pub burned: bool,
}

#[derive(Debug, Resource)]
pub struct CoordsList {
    pub material_coords: Vec<Coords>,
}

impl CoordsList {
    pub fn add_coords(&mut self, position_x: u32, position_y: u32) {
        let new_coords = Coords {
            x: position_x,
            y: position_y,
            burned: false,
        };
        self.material_coords.push(new_coords);
    }
}

impl Default for CoordsList {
    fn default() -> Self {
        Self {
            material_coords: vec![],
        }
    }
}
