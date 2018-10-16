use amethyst::{
    core::transform::Transform,
    ecs::prelude::{ Component, DenseVecStorage },
};

pub struct BoxBounds {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

pub struct BoxCollider {
    pub id: i32,
    pub immobile: bool,
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl BoxCollider {
    pub fn to_bounds(&self, local: &Transform) -> BoxBounds {
        let x = local.translation[0];
        let y = local.translation[1];

        BoxBounds {
            left: x + self.left,
            right: x + self.right,
            top: y + self.top,
            bottom: y + self.bottom,
        }
    }
}

impl Component for BoxCollider {
    type Storage = DenseVecStorage<Self>;
}
