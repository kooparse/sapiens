use amethyst::prelude::*;
use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::components::LocalTransform;
use amethyst::renderer::{ElementState, Event, KeyboardInput, VirtualKeyCode,
                         WindowEvent};

use utils::GameSphere;
use utils::GameLight;
use utils::GameCamera;

pub struct GameState;

const PLANET_COLOR: [f32; 4] = [0., 0.1, 0.1, 0.];
const MOON_COLOR: [f32; 4] = [0.1, 0.0, 0., 0.1];

impl State for GameState {
    fn on_start(&mut self, world: &mut World) {
        println!("Game loop is started...");

        let mut moon_transform = LocalTransform::default();
        moon_transform.translation = Vector3::new(1., 1., 1.);
        moon_transform.scale = [0.5; 3].into();

        // Initialize objects in our scene
        GameCamera::new(world);
        GameLight::new(world);
        GameSphere::new(world, PLANET_COLOR, LocalTransform::default());
        GameSphere::new(world, MOON_COLOR, moon_transform);
    }

    // fn update(&mut self, world: &mut World) -> Trans{
    //     // let tex_storage = &world.read_resource();
    //     // let material_defaults = &world.read_resource::<MaterialDefaults>();
    //     println!("Update...");
    //     Trans::None
    // }

    fn handle_event(&mut self, _: &mut World, event: Event) -> Trans {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => Trans::Quit,
                _ => Trans::None,
            },
            _ => Trans::None,
        }
    }
}
