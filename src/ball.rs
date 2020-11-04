use amethyst::core::ecs::{Component, DenseVecStorage, Entity, Write};
use amethyst::prelude::*;
use amethyst::renderer::{SpriteRender, SpriteSheet, Texture, ImageFormat, Sprite};
use amethyst::core::Transform;
use amethyst::assets::{Handle, AssetStorage, Loader};
use amethyst::core::ecs::{System, Read, WriteStorage, ReadExpect};
use amethyst::input::{InputHandler, StringBindings};


use amethyst::renderer::resources::Tint;
use amethyst::window::ScreenDimensions;



use amethyst::renderer::palette::Srgba;




#[derive(Clone)]
pub struct BallSprite {
    pub sprite: Handle<SpriteSheet>,
    // pub radius: u32,
}

#[derive(Default, Debug)]
pub struct Ball {
    pub radius: f32,
    pub recently_toggled: bool,
    pub selected: bool,
}

impl Ball {
    pub fn new(radius: f32) -> Ball {
        Ball { radius, ..Default::default() }
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

pub fn create_ball(world: &mut World, radius: f32, x: f32, y: f32) -> Entity {
    let sprite = load_sprite(world, radius);
    initialise_ball(world, sprite, radius, x, y)
}

pub fn remove_cached_sprite(world: &mut World) {
    world.insert::<Option<BallSprite>>(None);
}


fn initialise_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>,
                   radius: f32, x: f32, y: f32) -> Entity {
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
        .build()
}
// fn initialize_ball(world: &mut World) { // as UiImage
//
// }

fn load_sprite(world: &mut World, ball_radius: f32) -> Handle<SpriteSheet> {
    let diameter = (ball_radius * 2.0).floor() as u32;

    if world.has_value::<Option<BallSprite>>() {
        let sprite =
            world.fetch::<Option<BallSprite>>().as_ref().cloned();
        if sprite.is_some() {
            return sprite.unwrap().sprite;
        }
    }
    let s = load_sprite_sheet(world, diameter);
    world.insert(Some(BallSprite {
        sprite: s.clone()
    }));
    s
}


fn load_sprite_sheet(world: &mut World, diameter: u32)
                     -> Handle<SpriteSheet> {
    log::info!("Loading spritesheet");
    let offset_x = 0;
    let offset_y = 0;
    let offsets = [0.0, 0.0];

    let sprite = Sprite::from_pixel_values(
        diameter, diameter,
        diameter, diameter,
        offset_x, offset_y, offsets,
        false, false,
    );
    let texture = {
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
    let sheet = SpriteSheet {
        texture,
        sprites: vec![sprite],
    };
    let mut sprite_sheet_store =
        world.fetch_mut::<AssetStorage<SpriteSheet>>();
    sprite_sheet_store.insert(sheet)
}

pub struct CurrentGame {
    pub can_select_balls: bool,
    pub balls_selected: u32,
    pub balls_selected_max: u32,
}

pub struct BallMouseControl;

impl<'s> System<'s> for BallMouseControl {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        Write<'s, Option<CurrentGame>>,
        ReadExpect<'s, ScreenDimensions>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Tint>
    );

    fn run(&mut self, (_input, _game_state, _screen,
        _transforms, _balls, _tints): Self::SystemData) {
        // if let Some(state) = game_state.as_mut() {
        //     for (mut ball, mut trans, tint) in
        //     (&mut balls, &mut transforms, &mut tints).join() {
        //         if let Some((x, y)) = input.mouse_position() {
        //             let mouse = Point2::new(x, screen.height() - y);
        //             let ball_pos = Point2::new(trans.translation().x,
        //                                        trans.translation().y);
        //             if distance(&mouse, &ball_pos) <= ball.radius {
        //                 let button_down =
        //                     input.mouse_button_is_down(MouseButton::Left);
        //
        //                 if button_down
        //                     && !ball.recently_toggled
        //                     && state.can_select_balls
        //                     && (state.balls_selected < state.balls_selected_max || ball.selected) {
        //                     ball.selected.toggle();
        //                     if ball.selected {
        //                         state.balls_selected += 1;
        //                     } else {
        //                         state.balls_selected -= 1;
        //                     }
        //                     ball.recently_toggled = true;
        //                 } else if !button_down {
        //                     ball.recently_toggled = false;
        //                 }
        //
        //                 // Colors on hover
        //                 if ball.selected {
        //                     tint.0.blue = 1.2;
        //                     tint.0.red = 1.2;
        //                     tint.0.green = 1.5;
        //                     // tint.0.green = 0.5;
        //                     // tint.0.alpha = 1.5;
        //                 } else {
        //                     // tint.0.blue = 0.5;
        //                     // tint.0.green = 0.5;
        //                     // tint.0.alpha = 1.5;
        //                     tint.0.red = 1.3;
        //                     tint.0.blue = 1.3;
        //                     tint.0.green = 1.3;
        //                 }
        //             } else {
        //                 // Colors in idle
        //                 if ball.selected {
        //                     tint.0.red = 0.6;
        //                     tint.0.green = 1.0;
        //                     tint.0.blue = 0.6;
        //                     // tint.0.alpha = 1.0;
        //                 } else {
        //                     tint.0.red = 1.0;
        //                     tint.0.green = 1.0;
        //                     tint.0.blue = 1.0;
        //                     // tint.0.alpha = 1.0;
        //                 }
        //             }
        //         }
        //     }
        // }
    }
}