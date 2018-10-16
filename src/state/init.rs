use amethyst::{
    assets::{ AssetStorage, Loader },
    core::{
        cgmath::{ Matrix4, Vector3 },
        transform::{ GlobalTransform, Transform },
    },
    prelude::*,
    renderer::{
        Camera,
        Event,
        PngFormat,
        Projection,
        Sprite,
        Texture,
        TextureHandle,
        WithSpriteRender,
    },
};

use components::{ BoxCollider, Player };

pub enum WallSize {
    Tall,
}

pub struct Init;

impl<'a, 'b> State<GameData<'a, 'b>> for Init {
    fn handle_event(&mut self, _: StateData<GameData>, _: Event) -> Trans<GameData<'a, 'b>> {
        Trans::None
    }

    fn update(&mut self, state: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        state.data.update(&state.world);
        Trans::None
    }

    fn on_start(&mut self, state: StateData<GameData>) {
        let spritesheet = get_spritesheet(state.world);

        state.world.register::<BoxCollider>();
        state.world.register::<Player>();

        make_wall(
            state.world,
            spritesheet.clone(),
            WallSize::Tall,
            (ARENA_WIDTH / 5., ARENA_WIDTH / 2.)
        );

        initialize_player(state.world, spritesheet);

        initialize_camera(state.world);
    }
}


const ARENA_WIDTH: f32 = 160.;
const ARENA_HEIGHT: f32 = 160.;

const SPRITE_UNIT: f32 = 16.;
const SPRITESHEET_SIZE: (f32, f32) = (SPRITE_UNIT * 4., SPRITE_UNIT * 2.);

fn get_spritesheet(world: &World) -> TextureHandle {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();

    loader.load(
        "./assets/spritesheets/shapes.png",
        PngFormat,
        Default::default(),
        (),
        &texture_storage,
    )
}

fn initialize_camera(world: &mut World) {
    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            0., ARENA_WIDTH, ARENA_HEIGHT, 0.,
        )))
        .with(GlobalTransform(
            Matrix4::from_translation(Vector3::new(0., 0., 1.)).into()
        ))
        .build();
}

fn initialize_player(world: &mut World, spritesheet: TextureHandle) {
    let sprite = Sprite{
        left: 0.,
        right: SPRITE_UNIT,
        top: 0.,
        bottom: SPRITE_UNIT,
    };
    let mut transform = Transform::default();
    transform.set_position(Vector3::new(ARENA_WIDTH / 2., ARENA_HEIGHT / 2., 0.));

    world.create_entity()
        .with_sprite(&sprite, spritesheet, SPRITESHEET_SIZE)
        .expect("Failed to add sprite render on player")
        .with(GlobalTransform::default())
        .with(transform)
        .with(Player::new())
        .with(BoxCollider{
            id: 1,
            immobile: false,
            left: SPRITE_UNIT / -2.,
            right: SPRITE_UNIT / 2.,
            top: SPRITE_UNIT / 2.,
            bottom: SPRITE_UNIT / -2.,
        })
        .build();
}

fn make_wall(
    world: &mut World,
    spritesheet: TextureHandle,
    size: WallSize,
    (x, y): (f32, f32)
) {
    let sprite = match size {
        WallSize::Tall => Sprite {
            left: SPRITE_UNIT,
            right: SPRITE_UNIT * 2.,
            top: 0.,
            bottom: SPRITE_UNIT * 2.,
        },
    };

    let collision = match size {
        WallSize::Tall => BoxCollider{
            id: 2,
            immobile: true,
            left: SPRITE_UNIT / -2.,
            right: SPRITE_UNIT / 2.,
            top: SPRITE_UNIT,
            bottom: -SPRITE_UNIT,
        },
    };

    let mut transform = Transform::default();
    transform.set_position(Vector3::new(x, y, 0.));

    world.create_entity()
        .with_sprite(&sprite, spritesheet, SPRITESHEET_SIZE)
        .expect("Failed to add sprite render on wall")
        .with(GlobalTransform::default())
        .with(transform)
        .with(collision)
        .build();
}
