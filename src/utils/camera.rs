use amethyst::prelude::*;

use amethyst::core::cgmath::{Deg, Matrix4};
use amethyst::core::transform::components::Transform;
use amethyst::renderer::{Camera, Projection};

pub struct GameCamera;

impl GameCamera {
    pub fn new(world: &mut World) {
        let transform = Matrix4::from_translation([0.0, 0.0, -4.0].into())
            * Matrix4::from_angle_y(Deg(180.));

        world
            .create_entity()
            .with(Camera::from(Projection::perspective(1.3, Deg(60.0))))
            .with(Transform(transform.into()))
            .build();
    }
}
