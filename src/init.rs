use amethyst::core::Transform;
use amethyst::prelude::*;
use amethyst::renderer::Camera;
use amethyst::window::ScreenDimensions;


pub fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    let size = world.read_resource::<ScreenDimensions>();
    transform.set_translation_xyz(size.width() * 0.5, size.height() * 0.5, 1.0);

    let cam = Camera::standard_2d(size.width(), size.height());
    drop(size);
    world.create_entity()
        .with(cam)
        .with(transform)
        .build();
}