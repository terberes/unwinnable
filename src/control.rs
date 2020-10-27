use amethyst::core::ecs::{System, Read, WriteStorage, ReadStorage, Write, ReadExpect};
use amethyst::input::{InputHandler, StringBindings, Button};
use amethyst::core::math::{distance, Point2, Vector3};
use amethyst::core::Transform;
use crate::ball::Ball;
use amethyst::core::ecs::Join;
use amethyst::renderer::resources::Tint;
use amethyst::renderer::rendy::wsi::winit::VirtualKeyCode;
use amethyst::assets::HotReloadStrategy;
use amethyst::window::ScreenDimensions;

pub struct BallMouseControl;

impl<'s> System<'s> for BallMouseControl {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, ScreenDimensions>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Tint>
    );

    fn run(&mut self, (input, screen, mut transforms, mut balls, mut tints): Self::SystemData) {
        for (mut ball, mut trans, tint) in
        (&mut balls, &mut transforms, &mut tints).join() {
            if let Some((x, y)) = input.mouse_position() {
                let mouse = Point2::new(x, screen.height() - y);
                let ball_pos = Point2::new(trans.translation().x,
                                           trans.translation().y);
                if distance(&mouse, &ball_pos) <= ball.radius {
                    if !ball.hovered {
                        log::debug!("Ball hovered: {:?}", ball_pos.coords.data);
                        ball.hovered = true;
                        tint.0.blue = 0.5;
                        tint.0.green = 0.5;
                        tint.0.alpha = 0.3;
                    }
                } else {
                    ball.hovered = false;
                    tint.0.red = 1.0;
                    tint.0.green = 1.0;
                    tint.0.blue = 1.0;
                    tint.0.alpha = 1.0;
                }
            }
        }
    }
}