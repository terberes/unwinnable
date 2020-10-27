use amethyst::core::ecs::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{SpriteRender, SpriteSheet, Texture, ImageFormat, SpriteSheetFormat, Sprite};
use amethyst::core::Transform;
use amethyst::assets::{Handle, AssetStorage, Loader};
use amethyst::window::ScreenDimensions;
use amethyst::renderer::resources::Tint;
use amethyst::renderer::palette::Srgba;
use amethyst::core::math::Vector3;

#[derive(Clone)]
pub struct BallSprite {
    pub sprite: Handle<SpriteSheet>,
    // pub radius: u32,
}

#[derive(Default, Debug)]
pub struct Ball {
    pub radius: f32,
    pub hovered: bool,
}

impl Ball {
    pub fn new(radius: f32) -> Ball {
        Ball { radius, hovered: false }
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

pub fn create_ball(world: &mut World, radius: f32, x: f32, y: f32) {
    let sprite = load_sprite(world, radius);
    initialise_ball(world, sprite, radius, x, y);
}

fn initialise_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>,
                   radius: f32, x: f32, y: f32) {
    // Create the translation.
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(x, y, 0.0);

    // Assign the sprite for the ball. The ball is the only sprite in the sheet.
    let sprite_render =
        SpriteRender::new(sprite_sheet_handle.clone(), 0);


    let tint = Tint(Srgba::new(1.0, 1.0, 1.0, 1.0));

    world.create_entity()
        .with(sprite_render)
        .with(tint)
        .with(Ball::new(radius))
        .with(local_transform)
        .build();
}
// fn initialize_ball(world: &mut World) { // as UiImage
//
// }

fn load_sprite(world: &mut World, ball_radius: f32) -> Handle<SpriteSheet> {
    let diameter = (ball_radius * 2.0).floor() as u32;

    if world.has_value::<BallSprite>() {
        world.fetch::<BallSprite>().sprite.clone()
    } else {
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage =
                world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "sprites/ball.png",
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };
        let sprite =
            load_sprite_sheet(texture_handle, diameter, diameter);
        let mut sprite_sheet_store =
            world.fetch_mut::<AssetStorage<SpriteSheet>>();
        let s = sprite_sheet_store.insert(sprite.clone());
        drop(sprite_sheet_store);
        world.insert(BallSprite {
            sprite: s.clone()
        });
        s
    }
}


fn load_sprite_sheet(texture: Handle<Texture>, width: u32, height: u32) -> SpriteSheet {
    let image_w = width;
    let image_h = height;
    let sprite_w = width;
    let sprite_h = height;

    let offset_x = 0;
    let offset_y = 0;
    let offsets = [0.0, 0.0];

    let sprite = Sprite::from_pixel_values(
        image_w, image_h, sprite_w, sprite_h, offset_x,
        offset_y, offsets, false, false,
    );
    SpriteSheet {
        texture,
        sprites: vec![sprite],
    }
}