mod light;
mod sphere;
mod camera;

pub type Color = [f32; 4];

pub use self::light::GameLight;
pub use self::sphere::GameSphere;
pub use self::camera::GameCamera;
