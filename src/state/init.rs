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

        initialize_camera(state.world);
        initialize_player(state.world, spritesheet);
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
    transform.translation[0] = ARENA_WIDTH / 2.;
    transform.translation[1] = ARENA_HEIGHT / 2.;

    world.create_entity()
        .with_sprite(&sprite, spritesheet, SPRITESHEET_SIZE)
        .expect("Failed to add sprite render on player")
        .with(GlobalTransform::default())
        .with(transform)
        .build();
}
