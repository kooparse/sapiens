use amethyst::prelude::*;
use amethyst::assets::{Handle, Loader};
use amethyst::core::transform::components::{LocalTransform, Transform};
use amethyst::renderer::{Material, MaterialDefaults, Mesh, PosNormTex};
use genmesh::{MapToVertices, Triangulate, Vertices};
use genmesh::generators::SphereUV;

use utils::Color;
const SPHERE_TRIANGLES: usize = 32;

pub struct GameSphere;

impl GameSphere {
    pub fn new(world: &mut World, color: Color, transform: LocalTransform) {
        let (mesh, material) = {
            let loader = world.read_resource::<Loader>();
            let mesh: Handle<Mesh> = loader.load_from_data(
                generate_sphere(SPHERE_TRIANGLES, SPHERE_TRIANGLES).into(),
                (),
                &world.read_resource(),
            );

            let tex_storage = &world.read_resource();
            let material_defaults = &world.read_resource::<MaterialDefaults>();
            let albedo = loader.load_from_data(color.into(), (), &tex_storage);

            let material = Material {
                albedo,
                ..material_defaults.0.clone()
            };

            (mesh, material)
        };

        world
            .create_entity()
            .with(Transform::default())
            .with(transform)
            .with(mesh)
            .with(material)
            .build();
    }
}

fn generate_sphere(u: usize, v: usize) -> Vec<PosNormTex> {
    SphereUV::new(u, v)
        .vertex(|vertex| PosNormTex {
            position: vertex.pos,
            normal: vertex.normal,
            tex_coord: [0.1, 0.1],
        })
        .triangulate()
        .vertices()
        .collect()
}
