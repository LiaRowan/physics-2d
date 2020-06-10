use amethyst::{
    core::transform::Transform,
    ecs::prelude::{ Component, DenseVecStorage },
};

struct BoxBounds {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

pub struct BoxCollider {
    pub id: i32,
    pub immobile: bool,
    bounds: BoxBounds,
    current_local: Option<Transform>,
    prev_local: Option<Transform>,
}

impl BoxCollider {
    pub fn new(id: i32, immobile: bool, (top, left, right, bottom): (f32, f32, f32, f32))
    -> BoxCollider {
        let bounds = BoxBounds { top, left, right, bottom };

        BoxCollider { id, immobile, bounds, current_local: None, prev_local: None }
    }

    fn local_bounds(&self) -> Option<BoxBounds> {
        return match &self.current_local {
            None => None,
            Some(local) => {
                let x = local.translation[0];
                let y = local.translation[1];

                Some(BoxBounds {
                    left: x + self.bounds.left,
                    right: x + self.bounds.right,
                    top: y + self.bounds.top,
                    bottom: y + self.bounds.bottom,
                })
            },
        }
    }

    pub fn has_collision(&self, other: &BoxCollider) -> bool {
        return match (self.local_bounds(), other.local_bounds()) {
            (Some(bounds), Some(other_bounds)) => {
                bounds.left <= other_bounds.right &&
                bounds.right >= other_bounds.left &&
                bounds.bottom <= other_bounds.top &&
                bounds.top >= other_bounds.bottom
            },
            _ => false,
        }
    }

    pub fn update_locals(&mut self, locals: &Transform) {
        self.prev_local = self.current_local.clone();
        self.current_local = Some(locals.clone());
    }
}

impl Component for BoxCollider {
    type Storage = DenseVecStorage<Self>;
}
