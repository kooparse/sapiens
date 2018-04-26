use amethyst::prelude::World;
use amethyst::renderer::{AmbientColor, Light, PointLight, Rgba};

const AMBIENT_LIGHT_COLOUR: Rgba = Rgba(0.1, 0.1, 0.1, 1.);
const POINT_LIGHT_COLOUR: Rgba = Rgba(1., 1., 1., 1.);
const LIGHT_POSITION: [f32; 3] = [2., 2., -2.];
const LIGHT_RADIUS: f32 = 7.0;
const LIGHT_INTENSITY: f32 = 5.0;

pub struct GameLight;

impl GameLight {
    pub fn new(world: &mut World) {
        world.add_resource(AmbientColor(AMBIENT_LIGHT_COLOUR));

        let light: Light = PointLight {
            center: LIGHT_POSITION.into(),
            radius: LIGHT_RADIUS,
            intensity: LIGHT_INTENSITY,
            color: POINT_LIGHT_COLOUR,
            ..Default::default()
        }.into();

        world.create_entity().with(light).build();
    }
}
