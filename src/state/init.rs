use amethyst::{
    prelude::{ GameData, State, StateData, Trans },
    renderer::Event,
};


pub struct Init;

impl<'a, 'b> State<GameData<'a, 'b>> for Init {
    fn handle_event(&mut self, _: StateData<GameData>, _: Event) -> Trans<GameData<'a, 'b>> {
        Trans::None
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }
}
