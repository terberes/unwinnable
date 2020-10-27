use amethyst::{SimpleState, StateData, GameData};
use crate::ball;
use crate::ball::Ball;
use amethyst::prelude::*;
use amethyst::core::ecs::Entity;
use std::collections::HashSet;
use amethyst::window::ScreenDimensions;
use std::cmp::min;
use amethyst::renderer::debug_drawing::{DebugLines, DebugLinesParams, DebugLinesComponent};
use amethyst::core::math::{Point2, Point3};
use amethyst::renderer::palette::Srgba;
use crate::algorithm::calculate_strategy;

pub const BALL_RADIUS: f32 = 50.0;
pub const BALL_PADDING: f32 = 5.0;
pub const MAX_BALLS: u32 = 666;

#[derive(Default, Debug)]
pub struct GameRules {
    pub ball_count: u32,
    pub allowed_take_variants: Vec<u32>
}

#[derive(Default, Debug)]
pub struct Game {
    pub rules: GameRules,
    pub strategy: Vec<u32>,
}

impl Game {
    pub fn new(ball_count: u32, allowed_take_variants: Vec<u32>) -> Self {
        // Я знаю что вычисления не делают на UI Thread'е, но whatever
        let strategy =
            calculate_strategy(ball_count, &allowed_take_variants);
        Self {
            rules: GameRules {
                ball_count,
                allowed_take_variants
            },
            strategy
        }
    }
}

fn get_row(num: f32) -> f32 {
    (-1.0 + (1.0 + 8.0 * num).sqrt()) / 2.0
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.register::<Ball>();
        data.world.insert(DebugLines::new());
        data.world.insert(DebugLinesParams {
            line_width: 2.0
        });

        let ball_count = self.rules.ball_count;

        let screen = (*data.world.read_resource::<ScreenDimensions>()).clone();
        let game_area_center = screen.width() as f32 / 3.0;

        // let mut lines = DebugLinesComponent::with_capacity(100);
        // lines.add_rectangle_2d(
        //     Point2::new(0.0, 0.0),
        //     Point2::new(screen.width() / 3.0 * 2.0, screen.height()),
        //     0.0,
        //     Srgba::new(1.0, 0.0, 0.0, 1.0),
        // );
        // lines.add_line(
        //     Point3::new(game_area_center, screen.height(), 0.0),
        //     Point3::new(game_area_center, 0.0, 0.0),
        //     Srgba::new(0.0, 1.0, 0.0, 1.0));
        // data.world.register::<DebugLinesComponent>();
        // data.world
        //     .create_entity()
        //     .with(lines)
        //     .build();

        assert!(ball_count <= MAX_BALLS);
        let row_count = get_row(ball_count as f32).ceil() as u32;

        // Arrangement, not an array
        let mut arr = vec![0u32; row_count as usize];

        for _ in 0..ball_count {
            'inner: for i in (0..row_count).rev() {
                if arr[i as usize] < i + 1 {
                    arr[i as usize] += 1;
                    break 'inner;
                }
            }
        }

        debug_assert_eq!(arr.iter().sum::<u32>(), ball_count);

        // 2 thirds of the screen divided by count of balls in the largest row
        let ball_space = (screen.width() as f32 / 3.0 * 2.0)
            .min(screen.height() - BALL_PADDING * 8.0) / (row_count as f32);

        let ball_radius = ((ball_space - 2.0 * BALL_PADDING) / 2.0).min(BALL_RADIUS);

        let ball_padding = (ball_radius / 10.0).min(BALL_PADDING);

        for i in 0..row_count {
            let current_ball_count = arr[arr.len() - 1 - (i as usize)];
            if current_ball_count == 0 {
                continue;
            }
            let center = current_ball_count as f32 / 2.0;

            let y = (i as f32) * ((ball_radius + BALL_PADDING) * 2.0)
                + BALL_RADIUS + ball_padding * 4.0;

            for j in 1..=current_ball_count {
                let pos = center as f32 - j as f32 - 0.5;
                let x = game_area_center +
                    ((pos + 1.0) * 2.0 * (ball_radius + ball_padding));

                ball::create_ball(data.world, ball_radius, x, y)
            }
        }
    }
}

pub fn debug_ball(world: &mut World) {
    let size = world.read_resource::<ScreenDimensions>();
    let x = size.width() / 2.0;
    let y = size.height() / 2.0;
    drop(size);
    ball::create_ball(world, BALL_RADIUS, x, y);
}