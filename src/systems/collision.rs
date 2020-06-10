use amethyst::{
    core::transform::Transform,
    ecs::prelude::{ Join, ReadStorage, System, WriteStorage },
};

use components::box_collider::BoxCollider;

pub struct Collision;

impl<'s> System<'s> for Collision {
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, BoxCollider>,
    );

    fn run(&mut self, (locals, mut colliders): Self::SystemData) {
        (&locals, &mut colliders).join()
            .for_each(|(local, collider)| collider.update_locals(local));

        &colliders.join()
            .filter(|collider| !collider.immobile)
            .for_each(|collider| {
                check_collisions(collider, &colliders)
            });
    }
}

fn check_collisions(collider: &BoxCollider, all_colliders: &WriteStorage<BoxCollider>) {
    all_colliders.join()
        .filter(|other| other.id != collider.id)
        .for_each(|other_collider| {
            if collider.has_collision(other_collider) {
                println!("collision between {} and {}!", collider.id, other_collider.id);
            }
        });
}

