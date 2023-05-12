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
    pub fn add_coords(&mut self, new_coords: Coords) {
        self.material_coords.push(new_coords);
        println!("added material");

    }
    pub fn add_material_to_scene(&mut self,position_x: u32,position_y: u32)->bool{
        let new_coords = Coords {
            x: position_x,
            y: position_y,
            burned: false,
        };

        if self.material_coords.contains(&new_coords){
            print!("contain");
            false
        }
        else{
            self.add_coords(new_coords);
            println!("do not contains");
            true
            
        }
    }
}

impl Default for CoordsList {
    fn default() -> Self {
        Self {
            material_coords: vec![],
        }
    }
}
