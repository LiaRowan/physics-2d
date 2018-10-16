use amethyst::{
    core::transform::Transform,
    ecs::prelude::{ Join, ReadStorage, System },
};

use components::box_collider::{ BoxBounds, BoxCollider };

pub struct Collision;

impl<'s> System<'s> for Collision {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, BoxCollider>,
    );

    fn run(&mut self, (locals, box_cs): Self::SystemData) {
        let mobile_colliders = (&locals, &box_cs).join()
            .filter(|(_, collider)| !collider.immobile);

        for (local, box_c) in mobile_colliders {
            let others = (&locals, &box_cs).join()
                .filter(|(_, other)| box_c.id != other.id);

            for (other_local, other_box_c) in others {
                let a_bounds = box_c.to_bounds(&local);
                let b_bounds = other_box_c.to_bounds(&other_local);

                if has_collision(a_bounds, b_bounds) {
                    println!("collision between {} and {}", box_c.id, other_box_c.id);
                }
            }
        }
    }
}

fn has_collision(a: BoxBounds, b: BoxBounds) -> bool {
    a.left <= b.right &&
    a.right >= b.left &&
    a.bottom <= b.top &&
    a.top >= b.bottom
}
