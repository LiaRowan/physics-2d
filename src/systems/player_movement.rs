use amethyst::{
    core::{
        Time,
        transform::components::Transform,
    },
    ecs::{ Join, Read, ReadStorage, System, WriteStorage },
    input::InputHandler,
};

use components::Player;

pub struct PlayerMovement;

impl<'s> System<'s> for PlayerMovement {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (players, mut locals, input, time): Self::SystemData) {
        for (player, local) in (&players, &mut locals).join() {
            let movement_x = input.axis_value("player_movement_x");
            let movement_y = input.axis_value("player_movement_y");

            if let Some(mv_x) = movement_x {
                local.translation[0] +=
                    time.delta_seconds() * player.walk_speed * mv_x as f32;
            }
            if let Some(mv_y) = movement_y {
                local.translation[1] +=
                    time.delta_seconds() * player.walk_speed * mv_y as f32;
            }
        }
    }
}
