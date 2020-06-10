use amethyst::{
    ecs::prelude::{ Component, DenseVecStorage },
};


pub struct Player {
    pub walk_speed: f32,
}

impl Player {
    pub fn new() -> Player {
        Player { walk_speed: 35. }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
